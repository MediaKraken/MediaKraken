use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};

#[get("/about")]
pub fn about() -> Template {
    Template::render("tera/about.html", context! {
        title: "About",
    })
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template("tera/about.html", r#"
        {% extends "tera/base" %}
        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#).expect("valid Tera template");
}
