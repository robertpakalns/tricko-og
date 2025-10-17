use crate::generate_html::{create_html, default_html};

fn base(path: &str, name: &str, desc: &str) -> String {
    create_html(path, name, desc, "/icons/redline.png", "#9c2220")
}

pub fn html(path: &str) -> String {
    match path {
        "/redline" => base(
            path,
            "Redline Client",
            "Unofficial Electron client for Kirka.io",
        ),
        "/redline/changelog" => base(
            path,
            "Redline Client Changelog",
            "View the latest updates in the Redline Client",
        ),
        "/redline/privacy" => base(
            path,
            "Redline Client Privacy Policy",
            "Before using the Redline Client, review its Privacy Policy",
        ),
        "/redline/discord" => base(
            path,
            "Redline Client Discord Server",
            "Join the Redline Client community on Discord",
        ),
        "/redline/github" => base(
            path,
            "Redline Client GitHub",
            "View the Redline Client source code on GitHub",
        ),
        _ => default_html(path),
    }
}
