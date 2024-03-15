mod cargo;
mod toml;
use std::env;

// start intialises a new project from the rust book from the URL to the chapter section
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("must pass a URL to a chapter of the rust book".to_string());
    }

    let name: String;
    match url_to_name(&args[1]) {
        None => return Err("invalid URL".to_string()),
        Some(n) => name = n,
    }
    let name = format!("projects/{name}");

    if let Err(err) = cargo::create_project(&name.as_str()) {
        return Err(err.to_string());
    };
    if let Err(err) = toml::add_to_cargo_toml("Cargo.toml", &name.as_str()) {
        return Err(err.to_string());
    };

    return Ok(());
}

// name takes a URL like 'https://doc.rust-lang.org/book/ch03-02-data-types.html'
// and returns 'data_types'
fn url_to_name(url: &str) -> Option<String> {
    return url
        .split("/")
        .last()
        .and_then(|name| name.strip_suffix(".html"))
        .and_then(|name| {
            name.split("-")
                .skip(2) // skip the chapter & subsections
                .map(|part| part.to_string())
                .reduce(|a, b| format!("{}_{}", a, b))
        });
}
