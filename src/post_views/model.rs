use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub title: String,
    pub content: String,
}
