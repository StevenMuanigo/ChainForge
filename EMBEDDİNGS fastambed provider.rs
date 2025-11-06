use super::EmbeddingProvider;
use async_trait::async_trait;
use anyhow::Result;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

pub struct FastEmbedProvider {
    model: TextEmbedding,
    dimension: usize,
}

impl FastEmbedProvider {
    pub fn new(model_name: &str) -> Result<Self> {
        let model_type = match model_name {
            "BAAI/bge-small-en-v1.5" => EmbeddingModel::BGESmallENV15,
            "BAAI/bge-base-en-v1.5" => EmbeddingModel::BGEBaseENV15,
            "BAAI/bge-large-en-v1.5" => EmbeddingModel::BGELargeENV15,
            _ => EmbeddingModel::BGESmallENV15,
        };
        
        let model = TextEmbedding::try_new(InitOptions {
            model_name: model_type,
            show_download_progress: true,
            ..Default::default()
        })?;
        
        let dimension = match model_type {
            EmbeddingModel::BGESmallENV15 => 384,
            EmbeddingModel::BGEBaseENV15 => 768,
            EmbeddingModel::BGELargeENV15 => 1024,
            _ => 384,
        };
        
        Ok(Self { model, dimension })
    }
}

#[async_trait]
impl EmbeddingProvider for FastEmbedProvider {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let embeddings = self.model.embed(text_refs, None)?;
        Ok(embeddings)
    }
    
    async fn embed_query(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(vec![text], None)?;
        Ok(embeddings.into_iter().next().unwrap_or_default())
    }
    
    fn dimension(&self) -> usize {
        self.dimension
    }
}
