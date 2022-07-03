pub fn base_dir() -> String {
    dotenv::var("LOONEYTUBE_VIDEOS_DIR").unwrap()
}
