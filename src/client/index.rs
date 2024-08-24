use askama::Template;

#[derive(Template)]
#[template(path = "./index.html")]
pub struct Page<'a> {
    pub name: &'a str
}
