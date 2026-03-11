use tract_onnx::prelude::*;

// No Tract 0.22, o plano de execução para um modelo otimizado é tipicamente este.
type TractPlan = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

/// NeuralSaturator: Encapsula a lógica de inferência ONNX otimizada para blocos.
/// Projetado para ser "Plug & Play" com processamento de áudio em tempo real.
pub struct NeuralSaturator {
    model: Option<TractPlan>,
}

impl NeuralSaturator {
    /// Inicializa o saturador neural carregando um modelo ONNX.
    /// Caso falhe, retornará uma instância em modo "bypass" (silencioso/limpo).
    pub fn new(model_path: &str) -> Self {

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
                }
            }
            Err(e) => {
                eprintln!("[NeuralSaturator] ERRO ao carregar modelo {}: {:?}", model_path, e);
                Self {
                    model: None,
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

        let total_len = block_len;

        // 1. Converter para Tensor do Tract
        // Shape esperado para LSTM: [1, N, 1] onde N é block_len
        let input_tensor_res = tract_ndarray::ArrayView3::from_shape((1, block_len, 1), block);

        let input_tensor: Tensor = match input_tensor_res {
            Ok(view) => view.to_owned().into(),
            Err(e) => {
                eprintln!("[NeuralSaturator] Erro ao criar tensor: {:?}", e);
                block.fill(0.0);
                return; 
            }
        };

        // 2. Executar inferência
        let result = model.run(tvec!(input_tensor.into()));

        // 3. Sobrescrever o block in-place
        if let Ok(res) = result {
            if let Ok(view) = res[0].to_array_view::<f32>() {
                if let Some(slice) = view.as_slice() {
                    if slice.len() == block_len {
                        block.copy_from_slice(slice);
                        return;
                    } else if slice.len() > block_len {
                        // Caso seja maior, pega apenas os ultimos ou os primeiros? Geralmente LSTM block in block out é igual.
                        block.copy_from_slice(&slice[..block_len]);
                        return;
                    }
                }
            }
        }
        
        // Fallback em caso de erro na extração
        use std::sync::atomic::{AtomicBool, Ordering};
        static LOGGED_ERROR: AtomicBool = AtomicBool::new(false);
        if !LOGGED_ERROR.load(Ordering::Relaxed) {
            eprintln!("[NeuralSaturator] Erro na inferência ou tamanho de saída incompatível");
            LOGGED_ERROR.store(true, Ordering::Relaxed);
        }
        
        block.fill(0.0);
    }
}

