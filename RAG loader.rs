use super::Document;
use anyhow::Result;
use std::path::Path;

pub struct DocumentLoader;

impl DocumentLoader {
    pub async fn load_text(path: &Path) -> Result<Document> {
        let content = tokio::fs::read_to_string(path).await?;
        Ok(Document::new(
            content,
            path.to_string_lossy().to_string(),
        ))
    }
    
    pub async fn load_pdf(path: &Path) -> Result<Document> {
        let content = tokio::task::spawn_blocking({
            let path = path.to_path_buf();
            move || -> Result<String> {
                let bytes = std::fs::read(&path)?;
                let text = pdf_extract::extract_text_from_mem(&bytes)?;
                Ok(text)
            }
        })
        .await??;
        
        Ok(Document::new(
            content,
            path.to_string_lossy().to_string(),
        ))
    }
    
    pub async fn load_from_url(url: &str) -> Result<Document> {
        let client = reqwest::Client::new();
        let content = client.get(url).send().await?.text().await?;
        
        Ok(Document::new(content, url.to_string()))
    }
    
    pub fn load_from_string(content: String, source: impl Into<String>) -> Document {
        Document::new(content, source.into())
    }
}
