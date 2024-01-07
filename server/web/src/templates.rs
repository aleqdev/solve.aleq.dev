use askama::Template;


#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
  pub title: &'a str
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
  pub title: &'a str
}
