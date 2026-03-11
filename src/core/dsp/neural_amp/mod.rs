use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use ringbuf::HeapRb;
use ringbuf::traits::{Split, Producer, Consumer, Observer};

mod neural_dsp;
pub use neural_dsp::NeuralSaturator;

// Tipos exatos gerados por HeapRb<f32>::new(n).split()
type InProd = ringbuf::wrap::CachingProd<Arc<HeapRb<f32>>>;
type OutCons = ringbuf::wrap::CachingCons<Arc<HeapRb<f32>>>;

/// Processador Neural Amp baseado em Tract (ONNX).
/// Real-Time Safe: inferência ocorre em thread de background (Low Latency).
///
/// Arquitetura de buffers:
/// ┌──────────────┐   in_buf   ┌──────────────────┐  out_buf  ┌─────────────────┐
/// │ Thread Áudio │──────────→ │ Thread Inferência │─────────→ │  Thread Áudio   │
/// │  (push)      │            │ (NeuralSaturator) │           │   (pop local)   │
/// └──────────────┘            └──────────────────┘           └─────────────────┘
pub struct NeuralAmpProcessor {
    input_prod: InProd,
    output_cons: OutCons,
    running: Arc<AtomicBool>,
    thread_handle: Option<thread::JoinHandle<()>>,
    latency: u32,
    model_loaded: bool,
}

impl NeuralAmpProcessor {
    pub fn new(model_path: &str, latency: u32) -> Self {
        // Ring buffers grandes: permite acomodar latência de inferência e jitter da thread
        const RB_CAP: usize = 131072;

        let (in_prod, mut in_cons) = HeapRb::<f32>::new(RB_CAP).split();
        let (mut out_prod, out_cons) = HeapRb::<f32>::new(RB_CAP).split();

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        let model_path_str = model_path.to_string();

        // O histórico agora é fixo em 4096 para DenseWaveNet
        let effective_latency = latency;

        let handle = thread::Builder::new()
            .name("distortion-neural-worker".to_string())
            .spawn(move || {
                // Pre-encher o buffer de saída com silêncio (tamanho da latência reportada)
                // Isso sincroniza o pipeline com o PDC (Plugin Delay Compensation) do HOST.
                for _ in 0..effective_latency {
                    let _ = out_prod.try_push(0.0f32);
                }

                // Inicializar o NeuralSaturator (ONNX via Tract)
                let mut saturator = NeuralSaturator::new(&model_path_str);
                
                // Tamanho de bloco fixo para eficiência de inferência (256 samples p/ throughput SIMD)
                const BLOCK_SIZE: usize = 64;
                let mut chunk_buffer: Vec<f32> = Vec::with_capacity(BLOCK_SIZE);

                eprintln!("[NEURAL AMP] Thread de inferência iniciada com BLOCK_SIZE={}.", BLOCK_SIZE);

                let mut first_block_processed = false;

                while running_clone.load(Ordering::Relaxed) {
                    let available = in_cons.occupied_len();

                    if available >= BLOCK_SIZE {
                        chunk_buffer.clear();
                        
                        // Consumir o bloco fixo do ringbuffer
                        for _ in 0..BLOCK_SIZE {
                            if let Some(s) = in_cons.try_pop() {
                                chunk_buffer.push(s);
                            }
                        }

                        // Processar o bloco através da rede neural
                        // O método process_block gerencia o histórico de 4096 internamente (In-place)
                        saturator.process_block(&mut chunk_buffer);

                        if !first_block_processed {
                            println!("[NEURAL AMP] Primeiro bloco processado com sucesso!");
                            first_block_processed = true;
                        }

                        // Enviar o resultado para o producer de saída
                        for &s in chunk_buffer.iter() {
                            let _ = out_prod.try_push(s);
                        }
                    } else {
                        // Sleep curto para evitar Busy Waiting se não houver samples suficientes
                        thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
                
                eprintln!("[NEURAL AMP] Thread de inferência finalizada.");
            });

        match handle {
            Ok(h) => Self {
                input_prod: in_prod,
                output_cons: out_cons,
                running,
                thread_handle: Some(h),
                latency: effective_latency,
                model_loaded: true,
            },
            Err(_) => {
                let (fp, fc) = HeapRb::<f32>::new(1).split();
                Self {
                    input_prod: fp,
                    output_cons: fc,
                    running,
                    thread_handle: None,
                    latency: 0,
                    model_loaded: false,
                }
            }
        }
    }

    /// Envia um sample para a thread de inferência — Real-Time Safe.
    #[inline(always)]
    pub fn push(&mut self, sample: f32) {
        let _ = self.input_prod.try_push(sample);
    }

    /// Lê um sample processado da fila de output — Real-Time Safe.
    /// Retorna None se o pipeline de inferência ainda não produziu resultados.
    #[inline(always)]
    pub fn pop(&mut self) -> Option<f32> {
        self.output_cons.try_pop()
    }

    pub fn latency(&self) -> u32 {
        self.latency
    }

    pub fn is_ready(&self) -> bool {
        self.model_loaded
    }
}

impl Drop for NeuralAmpProcessor {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

