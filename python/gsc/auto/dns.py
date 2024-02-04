import asyncio
import functools

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
    if server.startswith("https://"):
        await _query.https(q=q, where=server, timeout=5)  # type: ignore
        return server
    raise NotImplementedError()


BEST: str = asyncio.run(dns())
