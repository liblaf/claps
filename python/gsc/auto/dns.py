import asyncio
import functools
from urllib import parse

from dns import asyncquery as _query
from dns import message as _message
from loguru import logger

SERVERS: list[str] = [
    "https://101.6.6.6/dns-query",
    "https://cloudflare-dns.com/dns-query",
    "https://1.1.1.1/dns-query",
    "https://dns.alidns.com/dns-query",
]


@functools.cache
async def dns() -> str:
    server: str = "udp://127.0.0.1:60053"
    try:
        await query(server)
        return server
    except Exception as e:
        logger.error("DNS {}: {}", server, e)
    for task in asyncio.as_completed([query(server) for server in SERVERS], timeout=5):
        try:
            result: str = await task
            logger.success("DNS: {}", result)
            return result
        except Exception as e:
            logger.error("DNS: {}", e)
    return "local"


@functools.cache
@logger.catch
async def query(server: str, query: str = "chat.liblaf.me") -> str:
    q: _message.QueryMessage = _message.make_query(qname=query, rdtype="A")
    parsed: parse.ParseResult = parse.urlparse(server)
    match parsed.scheme:
        case "https":
            await _query.https(q=q, where=server, timeout=5)  # type: ignore
            return server
        case "udp" | _:
            await _query.udp(q=q, where=parsed.hostname, port=parsed.port, timeout=5)  # type: ignore
            return server


BEST: str = asyncio.run(dns())
