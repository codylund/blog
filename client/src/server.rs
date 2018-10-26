use iron::Iron;
use iron::prelude::*;
use iron::status;
use persistent::State;
use router::Router;
use std::collections::HashMap;

pub const SERVER_ADDRESS: &'static str = "0.0.0.0:{}";
pub const REPOSITORY_URL: &'static str = "https://api.github.com/repos/codylund/blog";
pub const BLOG_PATH: &'static str = "blog";
pub const METADATA_FILE: &'static str = "metadata.blog";

// Create a new server instance on the provided port
pub fn new(port: u32) {
    // Instantiate a new GitHub file cache
    let mut github_cache = GitHubCache::new(REPOSITORY_URL.to_string(), 
            BLOG_PATH.to_string(), METADATA_FILE.to_string());

    // Set up the routes
    let mut router = Router::new();
    router.get("/", IndexHandler::handle_request, "index");
    router.get("/:query", BlogHandler::handle_request, "query");

    let mut chain = Chain::new(router);
    chain.link(State::<GitHubCache>::both(github_cache));

    // Fire that bad boy up
    let server_address = format!("0.0.0.0:{}", port);
    Iron::new(chain).http(server_address).unwrap();
}


// Cache for all the GitHub files. 
pub struct GitHubCache {
    pub posts: Vec<super::blog_repository::content::BlogPostMetadata>,
    pub cache: HashMap<String, String>,
}

impl GitHubCache {
    // Create a new cache and automatically populate it using the metadata file.
    fn new(repository: String, root: String, metadata_filepath: String) -> GitHubCache {
        let mut cache = HashMap::new();

        // Connect to the GitHub-based blog
        let mut blog = super::Blog::new(&repository, &root); 

        // Get metadata for current blog posts
        let posts = blog.get_blog_post_metadata(&metadata_filepath);
        for post in &posts {
            let sample_blog_post = blog.get_blog_post(&post.filename);
            cache.insert(post.filename.to_string(), sample_blog_post);
        }

        GitHubCache { posts, cache }
    }

    // Cache a file's contents using its path for a key
    fn put_file(&mut self, path: String, content: String) {
        self.cache.insert(path, content);
    }

    // Get the file's contents using its path
    fn get_file(&self, path: String) -> String {
        println!("Fetching {0} from cache.", path);
        self.cache.get(&path).unwrap_or(&String::from("")).to_string()
    }
}

/// Allows storing `GitHubCache` in Iron's persistent data structure.
impl ::iron::typemap::Key for GitHubCache {
    type Value = GitHubCache;
}

trait GetHandler {

    fn get(data: &GitHubCache, path: String) -> String;

    fn handle_request(req: &mut Request) -> IronResult<Response> {
        // Take a read lock on the GitHub cache
        let rwlock = req.get::<State<GitHubCache>>().unwrap();
        let github_cache = rwlock.read().unwrap();

        // Get the query from the request
        let query = req.extensions.get::<Router>()
                .unwrap()
                .find("query")
                .unwrap_or("index")
                .to_string();

        // Get the content based on the query and implementation
        let content = Self::get(&github_cache, query);

        // Build the response
        use iron::headers::{ContentType, AccessControlAllowOrigin};
        use iron::modifiers::Header;

        let mut resp = Response::with((status::Ok, content.to_string()));

        // put the headers on
        resp.set_mut(Header(AccessControlAllowOrigin::Any));
        resp.set_mut(Header(ContentType::html()));

        Ok(resp)
    }
}

struct IndexHandler;
impl GetHandler for IndexHandler {
    fn get(data: &GitHubCache, path: String) -> String {
        let html: Vec<String> = data.posts.iter().map(|metadata| {
            let mut content = String::new();

            content.push_str(&format!("<h1>{}</h1>", metadata.title));
            content.push_str(&format!("<h2>{}</h2>", metadata.description));
            content.push_str(&format!("<h2>Filename: {}</h3>", metadata.filename));
            content.push_str(&format!("<h3>Created: {}</h3>", metadata.created_date));
            content.push_str(&format!("<h3>Modified: {}</h3>", metadata.modified_date));

            content
        }).collect();
        html.join("")
    }
}

struct BlogHandler;
impl GetHandler for BlogHandler {
    fn get(data: &GitHubCache, path: String) -> String {
        data.get_file(path)
    }
}