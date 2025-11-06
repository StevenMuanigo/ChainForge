use super::{VectorMemory, SearchResult};
use anyhow::Result;
use async_trait::async_trait;
use qdrant_client::{
    client::QdrantClient,
    qdrant::{
        CreateCollection, Distance, VectorParams, VectorsConfig, PointStruct,
        SearchPoints, Filter, Condition, FieldCondition, Match,
    },
};
use std::sync::Arc;

pub struct QdrantVectorMemory {
    client: Arc<QdrantClient>,
    collection_name: String,
    vector_size: usize,
}

impl QdrantVectorMemory {
    pub async fn new(url: &str, collection_name: String, vector_size: usize) -> Result<Self> {
        let client = QdrantClient::from_url(url).build()?;
        
        let memory = Self {
            client: Arc::new(client),
            collection_name,
            vector_size,
        };
        
        memory.ensure_collection().await?;
        
        Ok(memory)
    }
    
    async fn ensure_collection(&self) -> Result<()> {
        let collections = self.client.list_collections().await?;
        
        let exists = collections.collections.iter().any(|c| c.name == self.collection_name);
        
        if !exists {
            self.client.create_collection(&CreateCollection {
                collection_name: self.collection_name.clone(),
                vectors_config: Some(VectorsConfig {
                    config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                        VectorParams {
                            size: self.vector_size as u64,
                            distance: Distance::Cosine.into(),
                            ..Default::default()
                        },
                    )),
                }),
                ..Default::default()
            })
            .await?;
            
            tracing::info!("Created Qdrant collection: {}", self.collection_name);
        }
        
        Ok(())
    }
}

#[async_trait]
impl VectorMemory for QdrantVectorMemory {
    async fn store(&self, id: &str, text: &str, embedding: Vec<f32>, metadata: serde_json::Value) -> Result<()> {
        let mut payload = serde_json::Map::new();
        payload.insert("text".to_string(), serde_json::Value::String(text.to_string()));
        payload.insert("metadata".to_string(), metadata);
        
        let point = PointStruct::new(
            id.to_string(),
            embedding,
            payload.into(),
        );
        
        self.client
            .upsert_points_blocking(self.collection_name.clone(), None, vec![point], None)
            .await?;
        
        Ok(())
    }
    
    async fn search(&self, query_embedding: Vec<f32>, top_k: usize, threshold: f32) -> Result<Vec<SearchResult>> {
        let search_result = self.client
            .search_points(&SearchPoints {
                collection_name: self.collection_name.clone(),
                vector: query_embedding,
                limit: top_k as u64,
                score_threshold: Some(threshold),
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;
        
        let results = search_result
            .result
            .iter()
            .filter_map(|point| {
                let text = point.payload.get("text")?.as_str()?.to_string();
                let metadata = point.payload.get("metadata")?.clone();
                
                Some(SearchResult {
                    id: point.id.clone()?.to_string(),
                    text,
                    score: point.score,
                    metadata,
                })
            })
            .collect();
        
        Ok(results)
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        use qdrant_client::qdrant::{DeletePoints, PointsIdsList};
        
        self.client
            .delete_points(
                self.collection_name.clone(),
                None,
                &DeletePoints {
                    collection_name: self.collection_name.clone(),
                    points: Some(qdrant_client::qdrant::delete_points::Points::Points(
                        PointsIdsList {
                            ids: vec![id.into()],
                        },
                    )),
                    ..Default::default()
                },
                None,
            )
            .await?;
        
        Ok(())
    }
}
