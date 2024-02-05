pub fn ddns(name: Option<&str>) -> String {
    format!(
        "{}.ddns.liblaf.me",
        name.unwrap_or(whoami::hostname().as_str())
    )
}

pub fn service_host(service: &str, host: Option<&str>) -> String {
    format!(
        "{}-{}.liblaf.me",
        service,
        host.unwrap_or(whoami::hostname().as_str())
    )
}

pub fn tunnel(id: &str) -> String {
    format!("{}.cfargotunnel.com", id)
}
