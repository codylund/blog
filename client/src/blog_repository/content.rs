/**
 * A blog post. Used by serde_json to parse JSON responses from GitHub. Only
 * members included in this struct will be parsed; all others will be ignored.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPost {
    pub content: String,
}

/**
 * Describes a blog post from a high-level.
 */ 
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostMetadata {
    pub title: String,
    pub description: String,
    pub filename: String,
    pub created_date: String,
    pub modified_date: String,
}