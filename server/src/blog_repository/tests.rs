use blog_repository::content;
use serde_json;    

static KEY_TITLE: &'static str = "title";
static KEY_DESCRIPTION: &'static str = "description";
static KEY_FILENAME: &'static str = "filename";
static KEY_CREATEDDATE: &'static str = "created_date";
static KEY_MODIFIEDDATE: &'static str = "modified_date";

static MOCK_TITLE: &'static str = "Sample Blog";
static MOCK_DESCRIPTION: &'static str = "There's not much to see here.";
static MOCK_FILENAME: &'static str = "sample.blog";
static MOCK_CREATEDDATE: &'static str = "October 10, 2018";
static MOCK_MODIFIEDDATE: &'static str = "October 11, 2018";

#[test]
fn test_deserialize_blog_metadata() {
    let mut json = String::from("[");
    json.push_str("{");
    json.push_str(&format!("\"{}\": \"{}\",", KEY_TITLE, MOCK_TITLE));
    json.push_str(&format!("\"{}\": \"{}\",", KEY_DESCRIPTION, MOCK_DESCRIPTION));
    json.push_str(&format!("\"{}\": \"{}\",", KEY_FILENAME, MOCK_FILENAME));
    json.push_str(&format!("\"{}\": \"{}\",", KEY_CREATEDDATE, MOCK_CREATEDDATE));
    json.push_str(&format!("\"{}\": \"{}\"", KEY_MODIFIEDDATE, MOCK_MODIFIEDDATE));
    json.push_str("}");
    json.push_str("]");

    let metadata: Vec<content::BlogPostMetadata> = serde_json::from_str(&json).unwrap();

    assert_eq!(metadata[0].title, MOCK_TITLE);
    assert_eq!(metadata[0].description, MOCK_DESCRIPTION);
    assert_eq!(metadata[0].filename, MOCK_FILENAME);
    assert_eq!(metadata[0].created_date, MOCK_CREATEDDATE);
    assert_eq!(metadata[0].modified_date, MOCK_MODIFIEDDATE);
}