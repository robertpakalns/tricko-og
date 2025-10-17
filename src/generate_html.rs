fn create_html(title: &str, desc: &str, img: &str, color: &str) -> String {
    format!(
        "<html>\
            <head>\
                <meta property=\"og:title\" content=\"{title}\" />\
                <meta property=\"og:description\" content=\"{desc}\" />\
                <meta property=\"og:image\" content=\"{img}\" />\
                <meta name=\"theme-color\" content=\"{color}\" />\
            </head>\
        </html>"
    )
}

fn default_html() -> String {
    create_html(
        "Tricko.pro",
        "The best stats site! Explore content for Cryzen.io, Kirka.io, Vectaria.io, and Voxiom.io, as well as the Redline and Voxtulate clients.",
        "/assets/icon.webp",
        "#ffffff",
    )
}

// Not fully implemented yet: dynamic check for /cryzen/leaderboard/?query
fn cryzen_html(path: &str) -> String {
    let h = |name: &str, desc: &str| create_html(name, desc, "/icons/games/cryzen.png", "#443d35");

    match path {
        "/cryzen" => h(
            "Cryzen.io",
            "Engage in an epic online multiplayer shooter experience. Play instantly with no downloads",
        ),
        "/cryzen/player" => h(
            "Cryzen.io Player Search",
            "Browse Cryzen.io players and view their in-game stats",
        ),
        p if p.starts_with("/cryzen/player/") => {
            match p.strip_prefix("/cryzen/player/").unwrap_or("") {
                "" => h(
                    "Cryzen.io Player Search",
                    "Browse Cryzen.io players and view their in-game stats",
                ),
                name => h(name, &format!("View stats for Cryzen.io player {name}")),
            }
        }
        "/cryzen/leaderboard" => h(
            "Cryzen.io Leaderboard Search",
            "Browse Cryzen.io leaderboards by various stats",
        ),
        "/cryzen/changelog" => h(
            "Cryzen.io Changelog",
            "View the latest updates in Cryzen.io",
        ),
        _ => default_html(),
    }
}

fn kirka_html(path: &str) -> String {
    let h = |name: &str, desc: &str| create_html(name, desc, "/icons/games/kirka.png", "#adadad");

    match path {
        "/kirka" => h(
            "Kirka.io",
            "A multiplayer FPS in the browser. No download required to play this io game",
        ),
        "/kirka/player" => h(
            "Kirka.io Player Search",
            "Browse Kirka.io players and view their in-game stats",
        ),
        p if p.starts_with("/kirka/player/") => {
            let id = p.strip_prefix("/kirka/player/").unwrap_or("");

            match id.len() {
                0 => h(
                    "Kirka.io Player Search",
                    "Browse Kirka.io players and view their in-game stats",
                ),
                6 => {
                    let id_up = id.to_uppercase();
                    h(
                        &format!("#{id_up}"),
                        &format!("View stats for Kirka.io player with short ID #{id_up}"),
                    )
                }
                _ => h(
                    &format!("Kirka.io Player"),
                    &format!("View stats for Kirka.io player with ID {id}"),
                ),
            }
        }
        "/kirka/clan" => h(
            "Kirka.io Clan Search",
            "Browse Kirka.io clans and view information about them",
        ),
        p if p.starts_with("/kirka/clan/") => match p.strip_prefix("/kirka/clan/").unwrap_or("") {
            "" => h(
                "Kirka.io Clan Search",
                "Browse Kirka.io clans and view information about them",
            ),
            name => h(
                name,
                &format!("View information about Kirka.io clan {name}"),
            ),
        },
        "/kirka/leaderboard" => h(
            "Kirka.io Leaderboard Search",
            "View Kirka.io leaderboards and explore top players and clans",
        ),
        "/kirka/leaderboard/players" => h(
            "Kirka.io Player Leaderboard",
            "View the top Kirka.io players on the leaderboard",
        ),
        "/kirka/leaderboard/clans" => h(
            "Kirka.io Clan Leaderboard",
            "View the top Kirka.io clans on the leaderboard",
        ),
        "/kirka/market" => h(
            "Kirka.io Market Search",
            "Browse the Kirka.io market and explore skins by various parameters",
        ),
        p if p.starts_with("/kirka/market") => {
            match p.strip_prefix("/kirka/market/").unwrap_or("") {
                "" => h(
                    "Kirka.io Market Search",
                    "Browse the Kirka.io market and explore skins by various parameters",
                ),
                name => h(
                    name,
                    &format!("View information about Kirka.io skin {name}"),
                ),
            }
        }
        "/cryzen/changelog" => h("Kirka.io Changelog", "View the latest updates in Kirka.io"),
        _ => default_html(),
    }
}

fn vectaria_html(path: &str) -> String {
    let h =
        |name: &str, desc: &str| create_html(name, desc, "/icons/games/vectaria.png", "#f1ad91");

    match path {
        "/vectaria" => h(
            "Vectaria.io",
            "A free browser game similar to Minecraft. Engage in building, crafting, digging, and much more",
        ),
        "/vectaria/player" => h(
            "Vectaria.io Player Search",
            "Browse Vectaria.io players and view their in-game stats",
        ),
        p if p.starts_with("/vectaria/player/") => {
            match p.strip_prefix("/vectaria/player/").unwrap_or("") {
                "" => h(
                    "Vectaria.io Player Search",
                    "Browse Vectaria.io players and view their in-game stats",
                ),
                name => h(name, &format!("View stats for Vectaria.io player {name}")),
            }
        }
        "/vectaria/server" => h(
            "Vectaria.io Server Search",
            "Browse Vectaria.io servers and view information about them",
        ),
        p if p.starts_with("/vectaria/server/") => {
            match p.strip_prefix("/vectaria/server/").unwrap_or("") {
                "" => h(
                    "Vectaria.io Server Search",
                    "Browse Vectaria.io servers and view information about them",
                ),
                id => {
                    let id_up = &id.to_uppercase();
                    h(
                        id_up,
                        &format!("View information about Vectaria.io server with ID #{id_up}"),
                    )
                }
            }
        }
        _ => default_html(),
    }
}

fn redline_html(path: &str) -> String {
    let h = |name: &str, desc: &str| create_html(name, desc, "/icons/redline.png", "#9c2220");

    match path {
        "/redline" => h("Redline Client", "Unofficial Electron client for Kirka.io"),
        "/redline/changelog" => h(
            "Redline Client Changelog",
            "View the latest updates in the Redline Client",
        ),
        "/redline/privacy" => h(
            "Redline Client Privacy Policy",
            "Before using the Redline Client, review its Privacy Policy",
        ),
        "/redline/discord" => h(
            "Redline Client Discord Server",
            "Join the Redline Client community on Discord",
        ),
        "/redline/github" => h(
            "Redline Client GitHub",
            "View the Redline Client source code on GitHub",
        ),
        _ => default_html(),
    }
}

fn voxtulate_html(path: &str) -> String {
    let h = |name: &str, desc: &str| create_html(name, desc, "/icons/voxtulate.png", "#7b7b7b");

    match path {
        "/voxtulate" => h(
            "Voxtulate Client",
            "Unofficial Electron client for Voxiom.io",
        ),
        "/voxtulate/changelog" => h(
            "Voxtulate Client Changelog",
            "View the latest updates in the Voxtulate Client",
        ),
        "/voxtulate/discord" => h(
            "Voxtulate Client Discord Server",
            "Join the Voxtulate Client community on Discord",
        ),
        "/voxtulate/github" => h(
            "Voxtulate Client GitHub",
            "View the Voxtulate Client source code on GitHub",
        ),
        _ => default_html(),
    }
}

pub fn get_html(path: Option<&str>) -> String {
    match path {
        Some(p) if p.starts_with("/cryzen") => cryzen_html(p),
        Some(p) if p.starts_with("/kirka") => kirka_html(p),
        Some(p) if p.starts_with("/vectaria") => vectaria_html(p),
        Some(p) if p.starts_with("/redline") => redline_html(p),
        Some(p) if p.starts_with("/voxtulate") => voxtulate_html(p),
        _ => default_html(),
    }
}
