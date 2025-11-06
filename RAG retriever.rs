se super::{Document, Chunk, chunker::TextChunker};
use crate::embeddings::EmbeddingProvider;
use crate::memory::{VectorMemory, SearchResult};
use anyhow::Result;
use std::sync::Arc;

pub struct Retriever {
    vector_store: Arc<dyn VectorMemory>,
    embedding_provider: Arc<dyn EmbeddingProvider>,
    chunker: TextChunker,
    top_k: usize,
    similarity_threshold: f32,
}

impl Retriever {
    pub fn new(
        vector_store: Arc<dyn VectorMemory>,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        chunk_size: usize,
        chunk_overlap: usize,
        top_k: usize,
        similarity_threshold: f32,
    ) -> Self {
        Self {
            vector_store,
            embedding_provider,
            chunker: TextChunker::new(chunk_size, chunk_overlap),
            top_k,
            similarity_threshold,
        }
    }
    
    pub async fn index_document(&self, document: &Document) -> Result<()> {
        let chunks = self.chunker.chunk_document(document)?;
        
        let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embedding_provider.embed(&texts).await?;
        
        for (chunk, embedding) in chunks.iter().zip(embeddings.iter()) {
            let metadata = serde_json::json!({
                "document_id": document.id,
                "chunk_index": chunk.chunk_index,
                "source": document.source,
            });
            
            self.vector_store
                .store(&chunk.id, &chunk.content, embedding.clone(), metadata)
                .await?;
        }
        
        tracing::info!("Indexed document {} with {} chunks", document.id, chunks.len());
        
        Ok(())
    }
    
    pub async fn retrieve(&self, query: &str) -> Result<Vec<String>> {
        let query_embedding = self.embedding_provider.embed_query(query).await?;
        
        let results = self.vector_store
            .search(query_embedding, self.top_k, self.similarity_threshold)
            .await?;
        
        let contexts: Vec<String> = results.iter().map(|r| r.text.clone()).collect();
        
        Ok(contexts)
    }
    
    pub async fn retrieve_with_scores(&self, query: &str) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embedding_provider.embed_query(query).await?;
        
        self.vector_store
            .search(query_embedding, self.top_k, self.similarity_threshold)
            .await
    }
    
    pub async fn build_context(&self, query: &str) -> Result<String> {
        let contexts = self.retrieve(query).await?;
        
        let context = contexts
            .iter()
            .enumerate()
            .map(|(i, c)| format!("[Context {}]\n{}\n", i + 1, c))
            .collect::<Vec<_>>()
            .join("\n");
        
        Ok(context)
    }
}
