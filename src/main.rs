use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

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

fn get_html(path: &str) -> String {
    match path {
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
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            return;
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        let html = get_html(path);
        let content_length = html.len();

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: text/html; charset=UTF-8\r\n\
            Content-Length: {content_length}\r\n\
            Connection: close\r\n\
            \r\n\
            {html}"
        );

        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6601").expect("Failed to bind to port 6601");
    println!("Server running on http://localhost:6601");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
