use super::{Chain, ChainInput, ChainOutput, ChainMetadata, StepInfo};
use crate::llm::{LLMProvider, LLMRequest};
use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

pub struct SimpleChain {
    name: String,
    description: String,
    llm: Arc<dyn LLMProvider>,
    prompt_template: String,
}

impl SimpleChain {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        llm: Arc<dyn LLMProvider>,
        prompt_template: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            llm,
            prompt_template: prompt_template.into(),
        }
    }
    
    fn render_prompt(&self, input: &ChainInput) -> String {
        let mut prompt = self.prompt_template.clone();
        
        for (key, value) in &input.variables {
            let placeholder = format!("{{{}}}", key);
            if let Some(value_str) = value.as_str() {
                prompt = prompt.replace(&placeholder, value_str);
            }
        }
        
        prompt
    }
}

#[async_trait]
impl Chain for SimpleChain {
    async fn execute(&self, input: ChainInput) -> Result<ChainOutput> {
        let start = std::time::Instant::now();
        
        let prompt = self.render_prompt(&input);
        
        let request = LLMRequest::new(prompt.clone());
        let response = self.llm.generate(&request).await?;
        
        let execution_time = start.elapsed().as_millis() as u64;
        
        Ok(ChainOutput {
            result: serde_json::json!({
                "output": response.text,
                "model": response.model,
            }),
            metadata: ChainMetadata {
                chain_name: self.name.clone(),
                execution_time_ms: execution_time,
                steps: vec![StepInfo {
                    name: "llm_call".to_string(),
                    duration_ms: response.latency_ms,
                    input: prompt,
                    output: response.text.clone(),
                }],
                total_tokens: response.tokens_used.total_tokens,
                total_cost: response.tokens_used.estimate_cost(&response.model),
            },
        })
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}
