use serde::{Deserialize, Serialize};

/// Parses PDF, DOCX, XLSX, PPTX into structured knowledge.
pub struct DocumentProcessor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDocument {
    pub id: String,
    pub file_name: String,
    pub mime_type: String,
    pub text_content: String,
    pub tables: Vec<TableData>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub author: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub page_count: Option<u32>,
    pub source_system: String,
}

impl DocumentProcessor {
    pub fn new() -> Self { Self }

    /// Process a file (path or bytes) and extract text and tables.
    pub async fn process(&self, file_name: &str, _data: &[u8], mime: &str) -> Result<ParsedDocument, String> {
        // In production: use Apache Tika or specialized parsers.
        Ok(ParsedDocument {
            id: uuid::Uuid::new_v4().to_string(),
            file_name: file_name.to_string(),
            mime_type: mime.to_string(),
            text_content: "Extracted text placeholder".into(),
            tables: vec![],
            metadata: DocumentMetadata {
                author: None,
                created_at: Some(chrono::Utc::now()),
                page_count: Some(1),
                source_system: "local".into(),
            },
        })
    }
}
