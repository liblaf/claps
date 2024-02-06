pub fn ddns(name: Option<&str>) -> String {
    format!(
        "{}.ddns.liblaf.me",
        name.unwrap_or(whoami::hostname().as_str())
    )
}

pub fn tunnel(id: &str) -> String {
    format!("{}.cfargotunnel.com", id)
}
