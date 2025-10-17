use crate::generate_html::{create_html, default_html};

fn base(name: &str, desc: &str) -> String {
    create_html(name, desc, "/icons/voxtulate.png", "#7b7b7b")
}

pub fn html(path: &str) -> String {
    match path {
        "/voxtulate" => base(
            "Voxtulate Client",
            "Unofficial Electron client for Voxiom.io",
        ),
        "/voxtulate/changelog" => base(
            "Voxtulate Client Changelog",
            "View the latest updates in the Voxtulate Client",
        ),
        "/voxtulate/discord" => base(
            "Voxtulate Client Discord Server",
            "Join the Voxtulate Client community on Discord",
        ),
        "/voxtulate/github" => base(
            "Voxtulate Client GitHub",
            "View the Voxtulate Client source code on GitHub",
        ),
        _ => default_html(),
    }
}
