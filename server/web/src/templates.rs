use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "content.html")]
pub struct ContentTemplate {
    pub user_logged_in: bool
}

#[derive(Template)]
#[template(path = "widgets/login-form.html")]
pub struct LoginFormTemplate {}

#[derive(Template)]
#[template(path = "widgets/register-form.html")]
pub struct RegisterFormTemplate {}
