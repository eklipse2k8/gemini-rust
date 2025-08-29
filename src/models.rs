use serde::{Deserialize, Serialize};

/// Role of a message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Message from the user
    User,
    /// Message from the model
    Model,
}

/// Content part that can be included in a message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    /// Text content
    Text {
        /// The text content
        text: String,
        /// Whether this is a thought summary (Gemini 2.5 series only)
        #[serde(skip_serializing_if = "Option::is_none")]
        thought: Option<bool>,
    },
    InlineData {
        /// The blob data
        #[serde(rename = "inlineData")]
        inline_data: Blob,
    },
    /// Function call from the model
    FunctionCall {
        /// The function call details
        #[serde(rename = "functionCall")]
        function_call: super::tools::FunctionCall,
    },
    /// Function response (results from executing a function call)
    FunctionResponse {
        /// The function response details
        #[serde(rename = "functionResponse")]
        function_response: super::tools::FunctionResponse,
    },
}

/// Blob for a message part
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    pub mime_type: String,
    pub data: String, // Base64 encoded data
}

impl Blob {
    /// Create a new blob with mime type and data
    pub fn new(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            mime_type: mime_type.into(),
            data: data.into(),
        }
    }
}

/// Content of a message
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Content {
    /// Parts of the content
    pub parts: Vec<Part>,
    /// Role of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl Content {
    /// Create a new text content
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::Text {
                text: text.into(),
                thought: None,
            }],
            role: None,
        }
    }

    /// Create a new content with a function call
    pub fn function_call(function_call: super::tools::FunctionCall) -> Self {
        Self {
            parts: vec![Part::FunctionCall { function_call }],
            role: None,
        }
    }

    /// Create a new content with a function response
    pub fn function_response(function_response: super::tools::FunctionResponse) -> Self {
        Self {
            parts: vec![Part::FunctionResponse { function_response }],
            role: None,
        }
    }

    /// Create a new content with a function response from name and JSON value
    pub fn function_response_json(name: impl Into<String>, response: serde_json::Value) -> Self {
        Self {
            parts: vec![Part::FunctionResponse {
                function_response: super::tools::FunctionResponse::new(name, response),
            }],
            role: None,
        }
    }

    /// Create a new content with inline data (blob data)
    pub fn inline_data(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::InlineData {
                inline_data: Blob::new(mime_type, data),
            }],
            role: None,
        }
    }

    /// Add a role to this content
    pub fn with_role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
}

/// Message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Content of the message
    pub content: Content,
    /// Role of the message
    pub role: Role,
}

impl Message {
    /// Create a new user message with text content
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            content: Content::text(text).with_role(Role::User),
            role: Role::User,
        }
    }

    /// Create a new model message with text content
    pub fn model(text: impl Into<String>) -> Self {
        Self {
            content: Content::text(text).with_role(Role::Model),
            role: Role::Model,
        }
    }

    pub fn embed(text: impl Into<String>) -> Self {
        Self {
            content: Content::text(text),
            role: Role::Model,
        }
    }

    /// Create a new function message with function response content from JSON
    pub fn function(name: impl Into<String>, response: serde_json::Value) -> Self {
        Self {
            content: Content::function_response_json(name, response).with_role(Role::Model),
            role: Role::Model,
        }
    }

    /// Create a new function message with function response from a JSON string
    pub fn function_str(
        name: impl Into<String>,
        response: impl Into<String>,
    ) -> Result<Self, serde_json::Error> {
        let response_str = response.into();
        let json = serde_json::from_str(&response_str)?;
        Ok(Self {
            content: Content::function_response_json(name, json).with_role(Role::Model),
            role: Role::Model,
        })
    }
}

/// Safety rating for content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRating {
    /// The category of the safety rating
    pub category: String,
    /// The probability that the content is harmful
    pub probability: String,
}

/// Citation metadata for content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    /// The citation sources
    pub citation_sources: Vec<CitationSource>,
}

/// Citation source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    /// The URI of the citation source
    pub uri: Option<String>,
    /// The title of the citation source
    pub title: Option<String>,
    /// The start index of the citation in the response
    pub start_index: Option<i32>,
    /// The end index of the citation in the response
    pub end_index: Option<i32>,
    /// The license of the citation source
    pub license: Option<String>,
    /// The publication date of the citation source
    pub publication_date: Option<String>,
}

/// A candidate response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    /// The content of the candidate
    pub content: Content,
    /// The safety ratings for the candidate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    /// The citation metadata for the candidate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
    /// The finish reason for the candidate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    /// The index of the candidate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

/// Metadata about token usage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    /// The number of prompt tokens
    pub prompt_token_count: i32,
    /// The number of response tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates_token_count: Option<i32>,
    /// The total number of tokens
    pub total_token_count: i32,
    /// The number of thinking tokens (Gemini 2.5 series only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts_token_count: Option<i32>,
}

/// Response from the Gemini API for content generation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationResponse {
    /// The candidates generated
    pub candidates: Vec<Candidate>,
    /// The prompt feedback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_feedback: Option<PromptFeedback>,
    /// Usage metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<UsageMetadata>,
}

/// Content of the embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEmbedding {
    /// The values generated
    pub values: Vec<f32>, //Maybe Quantize this
}

/// Response from the Gemini API for content embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEmbeddingResponse {
    /// The embeddings generated
    pub embedding: ContentEmbedding,
}

/// Response from the Gemini API for batch content embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchContentEmbeddingResponse {
    /// The embeddings generated
    pub embeddings: Vec<ContentEmbedding>,
}

/// Feedback about the prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    /// The safety ratings for the prompt
    pub safety_ratings: Vec<SafetyRating>,
    /// The block reason if the prompt was blocked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
}

impl GenerationResponse {
    /// Get the text of the first candidate
    pub fn text(&self) -> String {
        self.candidates
            .first()
            .and_then(|c| {
                c.content.parts.first().and_then(|p| match p {
                    Part::Text { text, thought: _ } => Some(text.clone()),
                    _ => None,
                })
            })
            .unwrap_or_default()
    }

    /// Get function calls from the response
    pub fn function_calls(&self) -> Vec<&super::tools::FunctionCall> {
        self.candidates
            .iter()
            .flat_map(|c| {
                c.content.parts.iter().filter_map(|p| match p {
                    Part::FunctionCall { function_call } => Some(function_call),
                    _ => None,
                })
            })
            .collect()
    }

    /// Get thought summaries from the response
    pub fn thoughts(&self) -> Vec<String> {
        self.candidates
            .iter()
            .flat_map(|c| {
                c.content.parts.iter().filter_map(|p| match p {
                    Part::Text {
                        text,
                        thought: Some(true),
                    } => Some(text.clone()),
                    _ => None,
                })
            })
            .collect()
    }

    /// Get all text parts (both regular text and thoughts)
    pub fn all_text(&self) -> Vec<(String, bool)> {
        self.candidates
            .iter()
            .flat_map(|c| {
                c.content.parts.iter().filter_map(|p| match p {
                    Part::Text { text, thought } => Some((text.clone(), thought.unwrap_or(false))),
                    _ => None,
                })
            })
            .collect()
    }
}

/// Request to generate content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentRequest {
    /// The contents to generate content from
    pub contents: Vec<Content>,
    /// The generation config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// The safety settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    /// The tools that the model can use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<super::tools::Tool>>,
    /// The tool config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    /// The system instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
}

/// Request to embed words
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedContentRequest {
    /// The specified embedding model
    pub model: String,
    /// The chunks content to generate embeddings
    pub content: Content,
    /// The embedding task type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<TaskType>,
    /// The title of the document (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The output_dimensionality (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimensionality: Option<i32>,
}

/// Request to batch embed requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchEmbedContentsRequest {
    /// The list of embed requests
    pub requests: Vec<EmbedContentRequest>,
}

/// Request to batch generate content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGenerateContentRequest {
    /// The list of generate content requests
    pub requests: Vec<GenerateContentRequest>,
}

/// Response from the Gemini API for batch content generation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchGenerateContentResponse {
    /// The generated responses
    pub generation_responses: Vec<GenerationResponse>,
}

/// Configuration for thinking (Gemini 2.5 series only)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    /// The thinking budget (number of thinking tokens)
    ///
    /// - Set to 0 to disable thinking
    /// - Set to -1 for dynamic thinking (model decides)
    /// - Set to a positive number for a specific token budget
    ///
    /// Model-specific ranges:
    /// - 2.5 Pro: 128 to 32768 (cannot disable thinking)
    /// - 2.5 Flash: 0 to 24576
    /// - 2.5 Flash Lite: 512 to 24576
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<i32>,

    /// Whether to include thought summaries in the response
    ///
    /// When enabled, the response will include synthesized versions of the model's
    /// raw thoughts, providing insights into the reasoning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_thoughts: Option<bool>,
}

impl ThinkingConfig {
    /// Create a new thinking config with default settings
    pub fn new() -> Self {
        Self {
            thinking_budget: None,
            include_thoughts: None,
        }
    }

    /// Set the thinking budget
    pub fn with_thinking_budget(mut self, budget: i32) -> Self {
        self.thinking_budget = Some(budget);
        self
    }

    /// Enable dynamic thinking (model decides the budget)
    pub fn with_dynamic_thinking(mut self) -> Self {
        self.thinking_budget = Some(-1);
        self
    }

    /// Include thought summaries in the response
    pub fn with_thoughts_included(mut self, include: bool) -> Self {
        self.include_thoughts = Some(include);
        self
    }
}

impl Default for ThinkingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// The temperature for the model (0.0 to 1.0)
    ///
    /// Controls the randomness of the output. Higher values (e.g., 0.9) make output
    /// more random, lower values (e.g., 0.1) make output more deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// The top-p value for the model (0.0 to 1.0)
    ///
    /// For each token generation step, the model considers the top_p percentage of
    /// probability mass for potential token choices. Lower values are more selective,
    /// higher values allow more variety.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// The top-k value for the model
    ///
    /// For each token generation step, the model considers the top_k most likely tokens.
    /// Lower values are more selective, higher values allow more variety.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// The maximum number of tokens to generate
    ///
    /// Limits the length of the generated content. One token is roughly 4 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,

    /// The candidate count
    ///
    /// Number of alternative responses to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<i32>,

    /// Whether to stop on specific sequences
    ///
    /// The model will stop generating content when it encounters any of these sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// The response mime type
    ///
    /// Specifies the format of the model's response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,

    /// The response schema
    ///
    /// Specifies the JSON schema for structured responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<serde_json::Value>,

    /// The thinking configuration
    ///
    /// Configuration for the model's thinking process (Gemini 2.5 series only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            temperature: Some(0.7),
            top_p: Some(0.95),
            top_k: Some(40),
            max_output_tokens: Some(1024),
            candidate_count: Some(1),
            stop_sequences: None,
            response_mime_type: None,
            response_schema: None,
            thinking_config: None,
        }
    }
}

/// Configuration for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    /// The function calling config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

/// Configuration for function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallingConfig {
    /// The mode for function calling
    pub mode: FunctionCallingMode,
}

/// Mode for function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FunctionCallingMode {
    /// The model may use function calling
    Auto,
    /// The model must use function calling
    Any,
    /// The model must not use function calling
    None,
}

/// Setting for safety
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetySetting {
    /// The category of content to filter
    pub category: HarmCategory,
    /// The threshold for filtering
    pub threshold: HarmBlockThreshold,
}

/// Category of harmful content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    /// Dangerous content
    Dangerous,
    /// Harassment content
    Harassment,
    /// Hate speech
    HateSpeech,
    /// Sexually explicit content
    SexuallyExplicit,
}

/// Threshold for blocking harmful content
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    /// Block content with low probability of harm
    BlockLowAndAbove,
    /// Block content with medium probability of harm
    BlockMediumAndAbove,
    /// Block content with high probability of harm
    BlockHighAndAbove,
    /// Block content with maximum probability of harm
    BlockOnlyHigh,
    /// Never block content
    BlockNone,
}

/// Embedding Task types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskType {
    ///Used to generate embeddings that are optimized to assess text similarity
    SemanticSimilarity,
    ///Used to generate embeddings that are optimized to classify texts according to preset labels
    Classification,
    ///Used to generate embeddings that are optimized to cluster texts based on their similarities
    Clustering,

    ///Used to generate embeddings that are optimized for document search or information retrieval.
    RetrievalDocument,
    RetrievalQuery,
    QuestionAnswering,
    FactVerification,

    /// Used to retrieve a code block based on a natural language query, such as sort an array or reverse a linked list.
    /// Embeddings of the code blocks are computed using RETRIEVAL_DOCUMENT.
    CodeRetrievalQuery,
}
