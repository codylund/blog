/**
 * Structs used for parsing GitHub API JSON responses
 * using serde.
 */

// Description of a GitHub repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    // If I was especially lazy, the description might be null
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub language: String,
}

// A blog post. Used by serde_json to parse JSON responses from GitHub. Only
// members included in this struct will be parsed; all others will be ignored.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogPost {
    pub content: String,
}

// Describes a blog post from a high-level.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogPostMetadata {
    pub title: String,
    pub description: String,
    pub filename: String,
    pub created_date: String,
    pub modified_date: String,
}