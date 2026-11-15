//! The `uw` command line tool.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => println!("uw {}", env!("CARGO_PKG_VERSION")),
        _ => {
            println!("uw {} (Faz 0: no commands yet)", env!("CARGO_PKG_VERSION"));
            println!("usage: uw --version");
        }
    }
}
