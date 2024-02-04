import functools

from dns import message as _message
from dns import query as _query
from loguru import logger

SERVERS: list[str] = [
    "https://101.6.6.6/dns-query",
    "https://cloudflare-dns.com/dns-query",
    "https://1.1.1.1/dns-query",
    "https://dns.alidns.com/dns-query",
]


@functools.cache
def dns() -> str:
    for server in SERVERS:
        try:
            query(server=server)
            logger.success("DNS: {}", server)
            return server
        except Exception as e:
            logger.error("DNS: {} {}", server, e)
    return "local"


@functools.cache
def query(server: str, query: str = "chat.liblaf.me") -> _message.Message:
    q: _message.QueryMessage = _message.make_query(qname=query, rdtype="A")
    if server.startswith("https://"):
        return _query.https(q=q, where=server, timeout=5)  # type: ignore
    raise NotImplementedError()
