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

// It does not handle routes with a slash at the end: e.g., /redline and /redline/ should return be the same OG
pub fn get_html(path: Option<&str>, query: Vec<(&str, &str)>) -> String {
    match path {
        Some(p) if p.starts_with("/cryzen") => routes::cryzen::html(p, query),
        Some(p) if p.starts_with("/kirka") => routes::kirka::html(p),
        Some(p) if p.starts_with("/redline") => routes::redline::html(p),
        Some(p) if p.starts_with("/vectaria") => routes::vectaria::html(p),
        Some(p) if p.starts_with("/voxiom") => routes::voxiom::html(p, query),
        Some(p) if p.starts_with("/voxtulate") => routes::voxtulate::html(p),
        Some(p) => routes::tricko::html(p),
        _ => default_html(path.unwrap_or("/")),
    }
}
