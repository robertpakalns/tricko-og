use crate::generate_html::{create_html, default_html};

fn base(name: &str, desc: &str) -> String {
    create_html(name, desc, "/icons/games/vectaria.png", "#f1ad91")
}

pub fn html(path: &str) -> String {
    match path {
        "/vectaria" => base(
            "Vectaria.io",
            "A free browser game similar to Minecraft. Engage in building, crafting, digging, and much more",
        ),
        "/vectaria/player" => base(
            "Vectaria.io Player Search",
            "Browse Vectaria.io players and view their in-game stats",
        ),
        p if p.starts_with("/vectaria/player/") => {
            match p.strip_prefix("/vectaria/player/").unwrap_or("") {
                "" => base(
                    "Vectaria.io Player Search",
                    "Browse Vectaria.io players and view their in-game stats",
                ),
                name => base(name, &format!("View stats for Vectaria.io player {name}")),
            }
        }
        "/vectaria/server" => base(
            "Vectaria.io Server Search",
            "Browse Vectaria.io servers and view information about them",
        ),
        p if p.starts_with("/vectaria/server/") => {
            match p.strip_prefix("/vectaria/server/").unwrap_or("") {
                "" => base(
                    "Vectaria.io Server Search",
                    "Browse Vectaria.io servers and view information about them",
                ),
                id => {
                    let id_up = &id.to_uppercase();
                    base(
                        id_up,
                        &format!("View information about Vectaria.io server with ID #{id_up}"),
                    )
                }
            }
        }
        _ => default_html(),
    }
}
