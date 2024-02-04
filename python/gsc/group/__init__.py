import asyncio
import functools
import math
from typing import Optional

from loguru import logger

from gsc.conf import outbound as _outbound
from gsc.group import country as _country
from gsc.group import geo as _geo
from gsc.group import ipv as _ipv

lock = asyncio.Semaphore(64)


@functools.cache
async def group(
    outbound: _outbound.Outbound,
    *,
    country: bool = True,
    geo: bool = False,
    ipv: bool = False,
    timeout: float = math.nan,
    proxy: Optional[str] = None,
) -> str:
    async with lock:
        tags: list[str] = []
        if geo and (c := await _geo.geo(proxy)):
            tags.append(c)
        if len(tags) == 0 and country:
            if c := _country.country(outbound):
                tags.append(c)
            else:
                return "🏳️‍🌈 OT 其他"
        if ipv:
            if c := await _ipv.ipv(proxy):
                tags.append(c)
            else:
                tags.append("IPv?")
        if math.isfinite(timeout) and timeout > 0:
            raise NotImplementedError()
        logger.debug("{} -> {}", outbound.tag, tags)
        return " ".join(tags)
