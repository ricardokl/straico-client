use serde::Deserialize;

#[derive(Deserialize)]
pub struct RagByIdData {
    #[serde(rename = "_id")]
    pub id: Box<String>,
    pub user_id: Box<String>,
    pub name: Box<String>,
    pub rag_url: Box<String>,
    pub original_filename: Box<String>,
    pub chunking_method: Box<String>,
    pub chunk_size: u32,
    pub chunk_overlap: u32,
    #[serde(rename = "createdAt")]
    pub created_at: Box<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Box<String>,
    #[serde(rename = "__v")]
    pub v: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_rag_response() {
        let data = json!({
            "_id": "670565d02fc07e1234eb",
            "user_id": "64ada922131d3f5",
            "name": "Test",
            "rag_url": "https://prompt-rack.s3.amazonaws.com/api/rag/64ada93ff7a1d6822131d3f5/d27eaec1-00df-48cd-a6ef-1b17a78d841c/index.faiss",
            "original_filename": "sample_txt.txt",
            "chunking_method": "fixed_size",
            "chunk_size": 1000,
            "chunk_overlap": 50,
            "createdAt": "2024-10-08T17:03:12.078Z",
            "updatedAt": "2024-10-08T17:03:12.078Z",
            "__v": 0
        });
        let json_str = data.to_string();
        let response: RagByIdData = serde_json::from_str(&json_str).expect("Failed to deserialize");
        assert_eq!(*response.id, "670565d02fc07e1234eb");
        assert_eq!(*response.user_id, "64ada922131d3f5");
        assert_eq!(*response.name, "Test");
        assert_eq!(*response.rag_url, "https://prompt-rack.s3.amazonaws.com/api/rag/64ada93ff7a1d6822131d3f5/d27eaec1-00df-48cd-a6ef-1b17a78d841c/index.faiss");
        assert_eq!(*response.original_filename, "sample_txt.txt");
        assert_eq!(*response.chunking_method, "fixed_size");
        assert_eq!(response.chunk_size, 1000);
        assert_eq!(response.chunk_overlap, 50);
        assert_eq!(*response.created_at, "2024-10-08T17:03:12.078Z");
        assert_eq!(*response.updated_at, "2024-10-08T17:03:12.078Z");
        assert_eq!(response.v, 0);
    }
}
