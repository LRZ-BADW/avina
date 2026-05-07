fn main() {
    if std::env::var("DOCS_RS")
        .map(|v| v == "1")
        .unwrap_or_default()
    {
        println!("cargo:rustc-env=SQLX_OFFLINE=true");
    }
}
