use std::str::FromStr;

use tiny_http::{Header, Response, Server};

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

fn main() {
    let server = Server::http("0.0.0.0:6601").unwrap();

    for req in server.incoming_requests() {
        let path = req.url();

        let html = match path {
            "/redline" => create_html(
                "Redline Client",
                "Unofficial Electron client for Kirka.io",
                "/icons/redline.png",
                "#9c2220",
            ),
            _ => create_html(
                "Tricko.pro",
                "Tricko.pro - Best Stats Site!",
                "/assets/icon.webp",
                "#ffffff",
            ),
        };

        let header = Header::from_str("Content-Type: text/html").unwrap();

        let response = Response::from_string(html).with_header(header);
        req.respond(response).unwrap();
    }
}
