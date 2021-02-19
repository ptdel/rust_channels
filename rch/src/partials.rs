use maud::{DOCTYPE, html, Markup};

fn header(title: &str) -> Markup {
    html! {
        head {
            (DOCTYPE) title { (title) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            span { "rust channels" }
        }
    }
}

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
        (body)
        (footer())
    }
}