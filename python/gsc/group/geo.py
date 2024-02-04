import contextlib
import functools
import re
from typing import Optional

import httpx

from gsc.group import country as _country


@functools.cache
async def geo(proxy: str) -> Optional[str]:
    with contextlib.suppress(Exception):
        async with httpx.AsyncClient(proxy=proxy) as client:
            response: httpx.Response = await client.get("https://api.ip.sb/geoip")
            response.raise_for_status()
            code: str = response.json()["country_code"]
            for key, pattern in _country.PROVIDER2PATTERN["default"]:
                if re.match(pattern, code):
                    return key
