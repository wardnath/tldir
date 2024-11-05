use anyhow::Result;
use candle::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;

pub struct InferenceModel {
    model: Box<dyn ModelTrait>,
    device: Device,
    tokenizer: tokenizers::Tokenizer,
    logits_processor: LogitsProcessor,
}

trait ModelTrait: Send {
    fn forward(&self, input: &Tensor) -> Result<Tensor>;
}

impl InferenceModel {
    pub fn new(
        model_path: PathBuf,
        tokenizer_path: PathBuf,
        cpu: bool,
    ) -> Result<Self> {
        let device = if cpu {
            Device::Cpu
        } else {
            Device::new_cuda(0)?
        };

        let tokenizer = tokenizers::Tokenizer::from_file(tokenizer_path)?;
        let logits_processor = LogitsProcessor::new(299792458, None, None);

        // Load model weights
        let weights = std::fs::read(model_path)?;
        let vb = unsafe { 
            VarBuilder::from_mmaped_safetensors(&[weights], DType::F32, &device)?
        };

        // Initialize model based on config
        let model = create_model(&vb)?;

        Ok(Self {
            model,
            device,
            tokenizer,
            logits_processor,
        })
    }

    pub async fn generate_summary(&self, text: &str, max_length: usize) -> Result<String> {
        let tokens = self.tokenizer.encode(text, true)?;
        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        
        let output = self.model.forward(&token_ids)?;
        let generated_ids = self.logits_processor.sample(&output)?;
        
        let text = self.tokenizer.decode(
            &generated_ids.to_vec1::<u32>()?,
            true
        )?;
        
        Ok(text)
    }
}
