//! Gemini API HTTP client

use reqwest::{Client, StatusCode};
use std::path::Path;
use tauri::AppHandle;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::types::{
    ChatQueryResult, Content, CreateStoreRequest, Document, FileSearchStore,
    FileSearchTool, GenerateContentRequest, GenerateContentResponse,
    GenerationConfig, Operation, Part, Tool, UploadMetadata,
};
use crate::settings;

const BASE_URL: &str = "https://generativelanguage.googleapis.com";
const API_VERSION: &str = "v1beta";

/// Gemini API client
pub struct GeminiClient {
    client: Client,
    app_handle: AppHandle,
}

impl GeminiClient {
    /// Create a new Gemini API client
    pub fn new(app_handle: AppHandle) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, app_handle }
    }

    /// Get API key from settings
    fn get_api_key(&self) -> Result<String, String> {
        settings::get_api_key_sync(&self.app_handle)
            .ok_or_else(|| "Gemini API Key not configured".to_string())
    }

    /// Build API URL with path
    fn api_url(&self, path: &str) -> String {
        format!("{}/{}/{}", BASE_URL, API_VERSION, path)
    }

    /// Build upload URL with path
    fn upload_url(&self, path: &str) -> String {
        format!("{}/upload/{}/{}", BASE_URL, API_VERSION, path)
    }

    // =========================================================================
    // FileSearchStore Operations
    // =========================================================================

    /// Create a new FileSearchStore
    pub async fn create_store(&self, display_name: &str) -> Result<FileSearchStore, String> {
        let api_key = self.get_api_key()?;
        let url = format!("{}?key={}", self.api_url("fileSearchStores"), api_key);

        let body = CreateStoreRequest {
            display_name: Some(display_name.to_string()),
        };

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to create store: {}", e))?;

        self.handle_response(response).await
    }

    /// Delete a FileSearchStore
    pub async fn delete_store(&self, store_name: &str, force: bool) -> Result<(), String> {
        let api_key = self.get_api_key()?;
        let mut url = format!("{}?key={}", self.api_url(store_name), api_key);

        if force {
            url.push_str("&force=true");
        }

        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to delete store: {}", e))?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Ok(()), // Already deleted
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(format!("Failed to delete store: {} - {}", status, body))
            }
        }
    }

    // =========================================================================
    // Document Operations
    // =========================================================================

    /// Upload a document to a FileSearchStore using resumable upload
    pub async fn upload_document(
        &self,
        store_name: &str,
        file_path: &str,
        display_name: Option<&str>,
    ) -> Result<Operation, String> {
        let api_key = self.get_api_key()?;
        let path = Path::new(file_path);

        // Get file size and mime type
        let metadata = tokio::fs::metadata(path)
            .await
            .map_err(|e| format!("Failed to read file metadata: {}", e))?;
        let file_size = metadata.len();
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        // Step 1: Initiate resumable upload
        let upload_url = self
            .initiate_resumable_upload(store_name, file_size, &mime_type, display_name, &api_key)
            .await?;

        // Step 2: Upload file bytes
        self.upload_file_bytes(&upload_url, path, file_size).await
    }

    /// Initiate a resumable upload
    async fn initiate_resumable_upload(
        &self,
        store_name: &str,
        file_size: u64,
        mime_type: &str,
        display_name: Option<&str>,
        api_key: &str,
    ) -> Result<String, String> {
        let url = format!(
            "{}:uploadToFileSearchStore?key={}",
            self.upload_url(store_name),
            api_key
        );

        let metadata = UploadMetadata {
            display_name: display_name.map(|s| s.to_string()),
            ..Default::default()
        };

        let response = self
            .client
            .post(&url)
            .header("X-Goog-Upload-Protocol", "resumable")
            .header("X-Goog-Upload-Command", "start")
            .header("X-Goog-Upload-Header-Content-Length", file_size.to_string())
            .header("X-Goog-Upload-Header-Content-Type", mime_type)
            .header("Content-Type", "application/json")
            .json(&metadata)
            .send()
            .await
            .map_err(|e| format!("Failed to initiate upload: {}", e))?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Failed to initiate upload: {}", body));
        }

        response
            .headers()
            .get("x-goog-upload-url")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| "No upload URL returned".to_string())
    }

    /// Upload file bytes to the upload URL
    async fn upload_file_bytes(
        &self,
        upload_url: &str,
        path: &Path,
        file_size: u64,
    ) -> Result<Operation, String> {
        let mut file = File::open(path)
            .await
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let mut buffer = Vec::with_capacity(file_size as usize);
        file.read_to_end(&mut buffer)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let response = self
            .client
            .post(upload_url)
            .header("Content-Length", file_size.to_string())
            .header("X-Goog-Upload-Offset", "0")
            .header("X-Goog-Upload-Command", "upload, finalize")
            .body(buffer)
            .send()
            .await
            .map_err(|e| format!("Failed to upload file: {}", e))?;

        self.handle_response(response).await
    }

    /// Get a document by name
    pub async fn get_document(&self, document_name: &str) -> Result<Document, String> {
        let api_key = self.get_api_key()?;
        let url = format!("{}?key={}", self.api_url(document_name), api_key);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get document: {}", e))?;

        self.handle_response(response).await
    }

    /// Delete a document
    pub async fn delete_document(&self, document_name: &str) -> Result<(), String> {
        let api_key = self.get_api_key()?;
        let url = format!("{}?key={}&force=true", self.api_url(document_name), api_key);

        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to delete document: {}", e))?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Ok(()), // Already deleted
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(format!("Failed to delete document: {} - {}", status, body))
            }
        }
    }

    // =========================================================================
    // Operation Operations
    // =========================================================================

    /// Get operation status
    pub async fn get_operation(&self, operation_name: &str) -> Result<Operation, String> {
        let api_key = self.get_api_key()?;
        let url = format!("{}?key={}", self.api_url(operation_name), api_key);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get operation: {}", e))?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Chat / Query Operations
    // =========================================================================

    /// Query stores with file search
    pub async fn query_stores(
        &self,
        store_names: &[String],
        query: &str,
    ) -> Result<ChatQueryResult, String> {
        let api_key = self.get_api_key()?;
        let model = "gemini-3-flash-preview";
        let url = format!(
            "{}?key={}",
            self.api_url(&format!("models/{}:generateContent", model)),
            api_key
        );

        // Build system prompt with context
        let now = chrono::Local::now();
        let system_prompt = format!(
            r#"You are a knowledgeable assistant that answers questions based on the user's uploaded documents.

## Core Responsibilities
- Search through the user's document store to find relevant information
- Provide accurate answers based on the retrieved documents
- Cite sources when referencing specific documents
- Acknowledge when information is not found in the documents

## Conversation Context Awareness
- Pay close attention to the conversation history when answering follow-up questions
- For follow-up questions, focus on documents and topics already discussed in the conversation
- Do NOT introduce new documents or topics unless the user explicitly asks for additional information
- Resolve pronouns and references by looking at the previous messages in the conversation
- When asked to reformat, summarize, or elaborate, use ONLY information already provided in the conversation

## Response Guidelines
- Answer in the same language as the user's question
- Be concise but comprehensive
- When citing documents, mention the document name or relevant section
- If the documents don't contain the answer, clearly state this and offer to help with what is available
- Do not make up information that is not in the documents
- Format your response using Markdown for better readability (headings, lists, code blocks, etc.)
- ALWAYS use tables to present structured, comparative, or list-based information

## Environment Context
- Current Time: {}
- Timezone: {}
- Available Document Stores: {}"#,
            now.format("%Y-%m-%d %H:%M:%S"),
            now.format("%Z"),
            store_names.len()
        );

        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(query.to_string()),
                }],
            }],
            system_instruction: Some(Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(system_prompt),
                }],
            }),
            tools: Some(vec![Tool {
                file_search: Some(FileSearchTool {
                    file_search_store_names: store_names.to_vec(),
                }),
            }]),
            generation_config: Some(GenerationConfig {
                temperature: Some(0.7),
                max_output_tokens: Some(4096),
            }),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to query stores: {}", e))?;

        let result: GenerateContentResponse = self.handle_response(response).await?;

        // Extract content and sources
        let content = result
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        // Extract source document names from grounding metadata
        let sources = Self::extract_sources(&result);

        Ok(ChatQueryResult { content, sources })
    }

    /// Generate suggest questions for stores
    pub async fn suggest_questions(
        &self,
        store_names: &[String],
        locale: &str,
    ) -> Result<Vec<String>, String> {
        let api_key = self.get_api_key()?;
        let model = "gemini-3-flash-preview";
        let url = format!(
            "{}?key={}",
            self.api_url(&format!("models/{}:generateContent", model)),
            api_key
        );

        let locale_instruction = match locale {
            "zh-TW" => "繁體中文",
            "ja" => "日本語",
            _ => "English",
        };

        let prompt = format!(
            "Based on the documents in the file search stores, suggest 5 interesting questions that a user might want to ask about the content. \
            Return ONLY a JSON array of 5 question strings, no other text. \
            The questions should be in {}. \
            Example format: [\"Question 1?\", \"Question 2?\", \"Question 3?\", \"Question 4?\", \"Question 5?\"]",
            locale_instruction
        );

        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(prompt),
                }],
            }],
            system_instruction: None,
            tools: Some(vec![Tool {
                file_search: Some(FileSearchTool {
                    file_search_store_names: store_names.to_vec(),
                }),
            }]),
            generation_config: Some(GenerationConfig {
                temperature: Some(0.9),
                max_output_tokens: Some(1024),
            }),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to get suggest questions: {}", e))?;

        let result: GenerateContentResponse = self.handle_response(response).await?;

        // Extract content and parse as JSON array
        let content = result
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        // Try to parse as JSON array, handling markdown code blocks
        let cleaned = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        Ok(serde_json::from_str(cleaned).unwrap_or_else(|_| vec![]))
    }

    // =========================================================================
    // Helper Functions
    // =========================================================================

    /// Extract source document names from grounding metadata
    /// Returns deduplicated list of document names (title field from API response)
    fn extract_sources(response: &GenerateContentResponse) -> Vec<String> {
        response
            .candidates
            .first()
            .and_then(|c| c.grounding_metadata.as_ref())
            .map(|gm| {
                gm.grounding_chunks
                    .iter()
                    .filter_map(|gc| {
                        gc.retrieved_context.as_ref().and_then(|rc| {
                            // Use title as it contains the document display name
                            rc.title.clone().or_else(|| rc.uri.clone())
                        })
                    })
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect()
            })
            .unwrap_or_default()
    }

    // =========================================================================
    // Response Handling
    // =========================================================================

    /// Handle API response and deserialize
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, String> {
        let status = response.status();

        if status.is_success() {
            let body = response
                .text()
                .await
                .map_err(|e| format!("Failed to read response: {}", e))?;

            if body.is_empty() || body == "{}" {
                // For empty responses, try to create a default
                serde_json::from_str("{}").map_err(|e| format!("Failed to parse response: {}", e))
            } else {
                serde_json::from_str(&body).map_err(|e| {
                    format!("Failed to parse response: {} - body: {}", e, body)
                })
            }
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(format!("API error ({}): {}", status, body))
        }
    }
}
