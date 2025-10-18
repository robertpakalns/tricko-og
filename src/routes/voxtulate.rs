use crate::generate_html::{create_html, default_html};

fn base(path: &str, name: &str, desc: &str) -> String {
    create_html(path, name, desc, "/icons/voxtulate.png", "#7b7b7b")
}

pub fn html(path: &str) -> String {
    match path {
        "/voxtulate" => base(
            path,
            "Voxtulate Client",
            "Unofficial Electron client for Voxiom.io. Supports Windows, macOS, and Linux",
        ),
        "/voxtulate/changelog" => base(
            path,
            "Voxtulate Client Changelog",
            "View the latest updates in the Voxtulate Client",
        ),
        "/voxtulate/discord" => base(
            path,
            "Voxtulate Client Discord Server",
            "Join the Voxtulate Client community on Discord",
        ),
        "/voxtulate/github" => base(
            path,
            "Voxtulate Client GitHub",
            "View the Voxtulate Client source code on GitHub",
        ),
        _ => default_html(path),
    }
}
