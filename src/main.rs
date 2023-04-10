fn main() {
    if let Err(e) = colops::get_args().and_then(colops::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
