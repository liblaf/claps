def proxy(url: str) -> str:
    url = url.removeprefix("https://")
    return f"https://gfw.liblaf.me/proxy/{url}"
