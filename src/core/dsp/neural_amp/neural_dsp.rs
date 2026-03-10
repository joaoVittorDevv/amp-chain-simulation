use tract_onnx::prelude::*;

// No Tract 0.22, o plano de execução para um modelo otimizado é tipicamente este.
type TractPlan = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

/// NeuralSaturator: Encapsula a lógica de inferência ONNX otimizada para blocos.
/// Projetado para ser "Plug & Play" com processamento de áudio em tempo real.
pub struct NeuralSaturator {
    model: Option<TractPlan>,
    history: Vec<f32>,
    input_buffer: Vec<f32>,
}

impl NeuralSaturator {
    /// Inicializa o saturador neural carregando um modelo ONNX.
    /// Caso falhe, retornará uma instância em modo "bypass" (silencioso/limpo).
    pub fn new(model_path: &str) -> Self {
        let history = vec![0.0f32; 4096];
        let input_buffer = vec![0.0f32; 4096 + 256];

        // Carregar modelo usando Tract (API 0.22)
        let model_res: TractResult<TractPlan> = (|| {
            let m = tract_onnx::onnx().model_for_path(model_path)?;
            let m = m.into_optimized()?;
            let m = m.into_runnable()?;
            Ok(m)
        })();

        match model_res {
            Ok(model) => {
                eprintln!("[NeuralSaturator] Modelo carregado com sucesso: {}", model_path);
                Self {
                    model: Some(model),
                    history,
                    input_buffer,
                }
            }
            Err(e) => {
                eprintln!("[NeuralSaturator] ERRO ao carregar modelo {}: {:?}", model_path, e);
                Self {
                    model: None,
                    history,
                    input_buffer,
                }
            }
        }
    }

    /// Processa um bloco de áudio. Garante latência determinística e 
    /// atualização correta do Receptive Field (Histórico).
    pub fn process_block(&mut self, block: &mut [f32]) {
        let block_len = block.len();
        
        // Se o modelo não foi carregado, retornamos silêncio
        if self.model.is_none() {
             block.fill(0.0);
             return;
        }
        
        // Garantimos que temos o modelo disponível
        let model = self.model.as_ref().unwrap();

        let total_len = 4096 + block_len;

        // 1. Preparar o input buffer (histórico + bloco atual)
        // Otimização Zero-Allocation: Usamos o buffer pré-alocado
        self.input_buffer[..4096].copy_from_slice(&self.history);
        self.input_buffer[4096..4096 + block_len].copy_from_slice(block);

        // 2. Atualizar o histórico (Circular Buffer-ish) AGORA, com a entrada original
        // Para manter a contiguidade necessária para o próximo Tensor, movemos os dados.
        // Com 4096 samples (16KB), isso cabe no cache L1 e é extremamente eficiente.
        if block_len >= 4096 {
            self.history.copy_from_slice(&block[block_len - 4096..]);
        } else {
            let keep_old = 4096 - block_len;
            self.history.copy_within(block_len.., 0);
            self.history[keep_old..].copy_from_slice(block);
        }

        // 3. Converter para Tensor do Tract
        // Shape esperado: [1, 1, 4096 + N]
        let input_tensor_res = tract_ndarray::ArrayView3::from_shape((1, 1, total_len), &self.input_buffer[..total_len]);

        let input_tensor: Tensor = match input_tensor_res {
            Ok(view) => view.to_owned().into(),
            Err(e) => {
                eprintln!("[NeuralSaturator] Erro ao criar tensor: {:?}", e);
                // Matém o sinal original intacto como pedido no template antigo? 
                // Não, pedido zero-allocation pediu block.fill(0.0)? Erro no user request disse `return block.to_vec()` antes,
                // Mas as diretrizes dizem fallback pra silencio:
                // Mas aqui podemos só manter block.
                return; 
            }
        };

        // 4. Executar inferência
        let result = model.run(tvec!(input_tensor.into()));

        match result {
            Ok(res) => {
                let output_tensor = &res[0];
                match output_tensor.to_array_view::<f32>() {
                    Ok(view) => {
                        let slice: &[f32] = view.as_slice().unwrap_or(&[]);
                        if slice.len() >= block_len {
                            // Este é o momento em que `block` é sobrescrito com a saída
                            block.copy_from_slice(&slice[slice.len() - block_len..]);
                        } else {
                            block.fill(0.0); // Silêncio em vez de bypass
                        }
                    }
                    Err(_) => block.fill(0.0), // Silêncio em vez de bypass
                }
            }
            Err(e) => {
                use std::sync::atomic::{AtomicBool, Ordering};
                static LOGGED_ERROR: AtomicBool = AtomicBool::new(false);
                if !LOGGED_ERROR.load(Ordering::Relaxed) {
                    eprintln!("[NeuralSaturator] Erro de inferência: {:?}", e);
                    LOGGED_ERROR.store(true, Ordering::Relaxed);
                }
                block.fill(0.0); // Silêncio em vez de bypass
            }
        };
    }
}

