use crate::generate_html::{create_html, default_html};

fn base(path: &str, name: &str, desc: &str) -> String {
    create_html(path, name, desc, "/icons/games/cryzen.png", "#443d35")
}

pub fn html(path: &str, query: Vec<(&str, &str)>) -> String {
    match path {
        "/cryzen" => base(
            path,
            "Cryzen.io",
            "Engage in an epic online multiplayer shooter experience. Play instantly with no downloads",
        ),
        "/cryzen/player" => base(
            path,
            "Cryzen.io Player Search",
            "Browse Cryzen.io players and view their in-game stats",
        ),
        p if p.starts_with("/cryzen/player/") => {
            match p.strip_prefix("/cryzen/player/").unwrap_or("") {
                "" => base(
                    path,
                    "Cryzen.io Player Search",
                    "Browse Cryzen.io players and view their in-game stats",
                ),
                name => base(
                    path,
                    name,
                    &format!("View stats for Cryzen.io player {name}"),
                ),
            }
        }
        "/cryzen/leaderboard" => base(
            path,
            "Cryzen.io Leaderboard Search",
            "Browse Cryzen.io leaderboards by various stats",
        ),
        p if p.starts_with("/cryzen/leaderboard/") => {
            let query_str = query
                .into_iter()
                .filter(|(k, v)| !k.is_empty() && !v.is_empty())
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<String>>()
                .join(" | ");

            match query_str.as_str() {
                q if q.is_empty() => base(
                    path,
                    "Cryzen.io Leaderboard Search",
                    "Browse Cryzen.io leaderboards by various stats",
                ),
                q => base(
                    path,
                    "Cryzen.io Leaderboard",
                    &format!("Browse Cryzen.io leaderboard with parameters {q}"),
                ),
            }
        }
        "/cryzen/changelog" => base(
            path,
            "Cryzen.io Changelog",
            "View the latest updates in Cryzen.io",
        ),
        _ => default_html(path),
    }
}
