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
        let input_buffer = Vec::with_capacity(4096 + 128);

        // Carregar modelo usando Tract (API 0.22)
        let model_res: TractResult<TractPlan> = tract_onnx::onnx()
            .model_for_path(model_path)
            .and_then(|m| m.into_optimized())
            .and_then(|m| m.into_runnable());

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
    pub fn process_block(&mut self, block: &[f32]) -> Vec<f32> {
        let block_len = block.len();
        
        // Se o modelo não foi carregado, retornamos o sinal original (bypass automático)
        if self.model.is_none() {
             return block.to_vec();
        }
        
        // Garantimos que temos o modelo disponível
        let model = self.model.as_ref().unwrap();

        let total_len = 4096 + block_len;

        // 1. Preparar o input buffer (histórico + bloco atual)
        // Otimização: Reutilizamos a capacidade do Vec para evitar alocações constantes na thread de DSP.
        self.input_buffer.clear();
        self.input_buffer.extend_from_slice(&self.history);
        self.input_buffer.extend_from_slice(block);

        // 2. Converter para Tensor do Tract
        // Shape esperado: [1, 1, 4096 + N]
        let input_tensor_res = tract_ndarray::ArrayView3::from_shape((1, 1, total_len), &self.input_buffer);

        let input_tensor: Tensor = match input_tensor_res {
            Ok(view) => view.to_owned().into(),
            Err(e) => {
                eprintln!("[NeuralSaturator] Erro ao criar tensor: {:?}", e);
                return block.to_vec();
            }
        };

        // 3. Executar inferência
        let result = model.run(tvec!(input_tensor.into()));

        let output = match result {
            Ok(res) => {
                let output_tensor = &res[0];
                match output_tensor.to_array_view::<f32>() {
                    Ok(view) => {
                        let slice: &[f32] = view.as_slice().unwrap_or(&[]);
                        if slice.len() >= block_len {
                            slice[slice.len() - block_len..].to_vec()
                        } else {
                            block.to_vec()
                        }
                    }
                    Err(_) => block.to_vec(),
                }
            }
            Err(e) => {
                eprintln!("[NeuralSaturator] Erro de inferência: {:?}", e);
                block.to_vec()
            }
        };

        // 4. Atualizar o histórico (Circular Buffer-ish)
        // Para manter a contiguidade necessária para o próximo Tensor, movemos os dados.
        // Com 4096 samples (16KB), isso cabe no cache L1 e é extremamente eficiente.
        if block_len >= 4096 {
            self.history.copy_from_slice(&block[block_len - 4096..]);
        } else {
            let keep_old = 4096 - block_len;
            self.history.copy_within(block_len.., 0);
            self.history[keep_old..].copy_from_slice(block);
        }

        output
    }
}

