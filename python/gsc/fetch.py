import asyncio
import itertools
from urllib import parse

import httpx
import opencc

from gsc import conf as _conf
from gsc.conf import outbound as _outbound

URL2PROVIDER: dict[str, str] = {
    "apiv2.lipulai.com": "FastLink",
    "apiv1.v27qae.com": "FlyingBird",
    "www.sub-nthu.com": "NTHU.CC",
}


def url2provider(url: str) -> str:
    split: parse.ParseResult = parse.urlparse(url)
    for key, value in URL2PROVIDER.items():
        if key in split.netloc:
            return value
    return split.netloc


async def fetch(urls: list[str]) -> list[_outbound.Outbound]:
    converter = opencc.OpenCC()
    async with httpx.AsyncClient() as client:

        async def fetch_one(url: str) -> list[_outbound.Outbound]:
            response: httpx.Response = await client.get(
                "https://api.v1.mk/sub",
                params={"target": "singbox", "url": url, "list": True},
            )
            conf = _conf.Conf(**response.json())
            outbounds: list[_outbound.Outbound] = conf.outbounds or []
            provider: str = url2provider(url)
            for outbound in outbounds:
                outbound.tag = converter.convert(outbound.tag) + f" [{provider}]"  # type: ignore
            return outbounds

        outbounds: list[_outbound.Outbound] = list(
            itertools.chain.from_iterable(
                await asyncio.gather(*[fetch_one(url) for url in urls])
            )
        )
        return outbounds
