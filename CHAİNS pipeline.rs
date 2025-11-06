use super::{Chain, ChainInput, ChainOutput, ChainMetadata, StepInfo};
use crate::llm::{LLMProvider, LLMRequest};
use crate::rag::retriever::Retriever;
use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

pub struct RAGPipeline {
    name: String,
    description: String,
    llm: Arc<dyn LLMProvider>,
    retriever: Arc<Retriever>,
    prompt_template: String,
}

impl RAGPipeline {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        llm: Arc<dyn LLMProvider>,
        retriever: Arc<Retriever>,
        prompt_template: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            llm,
            retriever,
            prompt_template: prompt_template.into(),
        }
    }
}

#[async_trait]
impl Chain for RAGPipeline {
    async fn execute(&self, input: ChainInput) -> Result<ChainOutput> {
        let start = std::time::Instant::now();
        let mut steps = Vec::new();
        
        let query = input.get_string("query")
            .ok_or_else(|| anyhow::anyhow!("Missing 'query' in input"))?;
        
        // Step 1: Retrieve context
        let retrieve_start = std::time::Instant::now();
        let context = self.retriever.build_context(&query).await?;
        let retrieve_duration = retrieve_start.elapsed().as_millis() as u64;
        
        steps.push(StepInfo {
            name: "retrieve_context".to_string(),
            duration_ms: retrieve_duration,
            input: query.clone(),
            output: format!("Retrieved {} characters of context", context.len()),
        });
        
        // Step 2: Build final prompt
        let prompt = self.prompt_template
            .replace("{context}", &context)
            .replace("{query}", &query);
        
        // Step 3: Generate response
        let llm_start = std::time::Instant::now();
        let request = LLMRequest::new(prompt.clone());
        let response = self.llm.generate(&request).await?;
        let llm_duration = llm_start.elapsed().as_millis() as u64;
        
        steps.push(StepInfo {
            name: "llm_generate".to_string(),
            duration_ms: llm_duration,
            input: prompt,
            output: response.text.clone(),
        });
        
        let execution_time = start.elapsed().as_millis() as u64;
        
        Ok(ChainOutput {
            result: serde_json::json!({
                "output": response.text,
                "context_used": context,
                "model": response.model,
            }),
            metadata: ChainMetadata {
                chain_name: self.name.clone(),
                execution_time_ms: execution_time,
                steps,
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
