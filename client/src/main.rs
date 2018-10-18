extern crate blog;

use blog::Blog;

fn main() {
    let mut blog = Blog::new("https://api.github.com/repos/codylund/blog", "blog");    
    let sample_blog_post = blog.get_blog_post("sample.blog");
    println!("Done: {:?}", sample_blog_post);
}
