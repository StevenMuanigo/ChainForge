use super::{Document, Chunk};
use anyhow::Result;

pub struct TextChunker {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl TextChunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }
    
    pub fn chunk_document(&self, document: &Document) -> Result<Vec<Chunk>> {
        let text = &document.content;
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut chunk_index = 0;
        
        while start < text.len() {
            let end = std::cmp::min(start + self.chunk_size, text.len());
            let chunk_text = &text[start..end];
            
            let chunk = Chunk {
                id: format!("{}_{}", document.id, chunk_index),
                document_id: document.id.clone(),
                content: chunk_text.to_string(),
                chunk_index,
                metadata: document.metadata.clone(),
            };
            
            chunks.push(chunk);
            
            if end >= text.len() {
                break;
            }
            
            start += self.chunk_size - self.chunk_overlap;
            chunk_index += 1;
        }
        
        Ok(chunks)
    }
    
    pub fn chunk_by_sentences(&self, document: &Document) -> Result<Vec<Chunk>> {
        let sentences: Vec<&str> = document
            .content
            .split(|c| c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut chunk_index = 0;
        
        for sentence in sentences {
            let sentence = sentence.trim();
            
            if current_chunk.len() + sentence.len() > self.chunk_size && !current_chunk.is_empty() {
                chunks.push(Chunk {
                    id: format!("{}_{}", document.id, chunk_index),
                    document_id: document.id.clone(),
                    content: current_chunk.clone(),
                    chunk_index,
                    metadata: document.metadata.clone(),
                });
                
                current_chunk.clear();
                chunk_index += 1;
            }
            
            if !current_chunk.is_empty() {
                current_chunk.push(' ');
            }
            current_chunk.push_str(sentence);
        }
        
        if !current_chunk.is_empty() {
            chunks.push(Chunk {
                id: format!("{}_{}", document.id, chunk_index),
                document_id: document.id.clone(),
                content: current_chunk,
                chunk_index,
                metadata: document.metadata.clone(),
            });
        }
        
        Ok(chunks)
    }
}
