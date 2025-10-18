use crate::routes;

static BASE_URL: &str = "https://tricko.pro";

pub fn create_html(path: &str, title: &str, desc: &str, img: &str, color: &str) -> String {
    format!(
        "<html>\
            <head>\
                <meta property=\"og:title\" content=\"{title}\" />\
                <meta property=\"og:description\" content=\"{desc}\" />\
                <meta property=\"og:image\" content=\"{}\" />\
                <meta property=\"og:url\" content=\"{}\" />\
                <meta property=\"og:type\" content=\"website\" />\
                <meta name=\"theme-color\" content=\"{color}\" />\
            </head>\
        </html>",
        format!("{BASE_URL}{img}"),
        format!("{BASE_URL}{path}"),
    )
}

pub fn default_html(path: &str) -> String {
    create_html(
        path,
        "Tricko.pro",
        "The best stats site! Explore content for Cryzen.io, Kirka.io, Vectaria.io, and Voxiom.io, as well as the Redline and Voxtulate clients",
        "/assets/icon.webp",
        "#ffffff",
    )
}

pub fn normalize_path(path: &str) -> String {
    let mut p = path.to_string();

    // Replace all double slashes
    // Apache adds double slashes at the beginning of the path
    while p.contains("//") {
        p = p.replace("//", "/");
    }

    if p.len() > 1 {
        p = p.trim_end_matches('/').to_string();
    }

    p
}

pub fn get_html(path: Option<&str>, query: Vec<(&str, &str)>) -> String {
    match normalize_path(path.unwrap_or("/")) {
        p if p.starts_with("/cryzen") => routes::cryzen::html(&p, query),
        p if p.starts_with("/kirka") => routes::kirka::html(&p),
        p if p.starts_with("/redline") => routes::redline::html(&p),
        p if p.starts_with("/vectaria") => routes::vectaria::html(&p),
        p if p.starts_with("/voxiom") => routes::voxiom::html(&p, query),
        p if p.starts_with("/voxtulate") => routes::voxtulate::html(&p),
        p => routes::tricko::html(&p),
    }
}
