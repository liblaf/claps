import contextlib
import functools
from typing import Optional

import httpx


@functools.cache
async def ipv(proxy: str) -> Optional[str]:
    ipv4: bool = False
    ipv6: bool = False
    with contextlib.suppress(Exception):
        await ipvx(proxy, 4)
        ipv4 = True
    with contextlib.suppress(Exception):
        await ipvx(proxy, 6)
        ipv6 = True
    if ipv4 and ipv6:
        return "IPv4/6"
    elif ipv4:
        return "IPv4"
    elif ipv6:
        return "IPv6"


@functools.cache
async def ipvx(proxy: str, x: int) -> None:
    async with httpx.AsyncClient(proxy=proxy) as client:
        response: httpx.Response = await client.get(f"https://api-ipv{x}.ip.sb/ip")
        response.raise_for_status()
