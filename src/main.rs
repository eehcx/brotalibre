fn main() {
    if let Err(err) = brotalibre::run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
