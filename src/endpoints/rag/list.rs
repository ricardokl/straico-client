use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RagList {
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
    pub v: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct RagListResponseData(Vec<RagList>);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_rag_list_response_data() {
        let json_data = r#"
        [
            {
                "_id": "id1",
                "user_id": "user1",
                "name": "name1",
                "rag_url": "url1",
                "original_filename": "file1",
                "chunking_method": "method1",
                "chunk_size": 10,
                "chunk_overlap": 2,
                "createdAt": "2021-01-01",
                "updatedAt": "2021-01-02",
                "__v": 1
            },
            {
                "_id": "id2",
                "user_id": "user2",
                "name": "name2",
                "rag_url": "url2",
                "original_filename": "file2",
                "chunking_method": "method2",
                "chunk_size": 20,
                "chunk_overlap": 3,
                "createdAt": "2021-02-01",
                "updatedAt": "2021-02-02",
                "__v": 2
            }
        ]
        "#;
        let response: RagListResponseData =
            serde_json::from_str(json_data).expect("Deserialization failed");
        assert_eq!(response.0.len(), 2);
        assert_eq!(*response.0[0].user_id, "user1");
        assert_eq!(*response.0[0].id, "id1");
        assert_eq!(*response.0[0].name, "name1");
        assert_eq!(*response.0[0].rag_url, "url1");
        assert_eq!(*response.0[0].original_filename, "file1");
        assert_eq!(*response.0[0].chunking_method, "method1");
        assert_eq!(response.0[0].chunk_size, 10);
        assert_eq!(response.0[0].chunk_overlap, 2);
        assert_eq!(*response.0[0].created_at, "2021-01-01");
        assert_eq!(*response.0[0].updated_at, "2021-01-02");
        assert_eq!(response.0[0].v, 1);
        assert_eq!(*response.0[1].user_id, "user2");
        assert_eq!(*response.0[1].id, "id2");
        assert_eq!(*response.0[1].name, "name2");
        assert_eq!(*response.0[1].rag_url, "url2");
        assert_eq!(*response.0[1].original_filename, "file2");
        assert_eq!(*response.0[1].chunking_method, "method2");
        assert_eq!(response.0[1].chunk_size, 20);
        assert_eq!(response.0[1].chunk_overlap, 3);
        assert_eq!(*response.0[1].created_at, "2021-02-01");
        assert_eq!(*response.0[1].updated_at, "2021-02-02");
        assert_eq!(response.0[1].v, 2);
    }
}
