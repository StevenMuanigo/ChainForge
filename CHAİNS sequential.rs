use super::{Chain, ChainInput, ChainOutput, ChainMetadata, StepInfo};
use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

pub struct SequentialChain {
    name: String,
    description: String,
    chains: Vec<Arc<dyn Chain>>,
}

impl SequentialChain {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            chains: Vec::new(),
        }
    }
    
    pub fn add_chain(mut self, chain: Arc<dyn Chain>) -> Self {
        self.chains.push(chain);
        self
    }
}

#[async_trait]
impl Chain for SequentialChain {
    async fn execute(&self, mut input: ChainInput) -> Result<ChainOutput> {
        let start = std::time::Instant::now();
        let mut all_steps = Vec::new();
        let mut total_tokens = 0;
        let mut total_cost = 0.0;
        
        for chain in &self.chains {
            let output = chain.execute(input.clone()).await?;
            
            all_steps.extend(output.metadata.steps);
            total_tokens += output.metadata.total_tokens;
            total_cost += output.metadata.total_cost;
            
            // Pass output to next chain
            input = ChainInput::new().with_variable("previous_output", output.result.clone());
        }
        
        let execution_time = start.elapsed().as_millis() as u64;
        
        // Get final output
        let final_output = input.get_string("previous_output")
            .unwrap_or_else(|| "No output".to_string());
        
        Ok(ChainOutput {
            result: serde_json::json!({"output": final_output}),
            metadata: ChainMetadata {
                chain_name: self.name.clone(),
                execution_time_ms: execution_time,
                steps: all_steps,
                total_tokens,
                total_cost,
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
