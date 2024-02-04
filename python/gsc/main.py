import asyncio
import itertools
import math

from gsc import conf as _conf
from gsc import fetch as _fetch
from gsc import group as _group
from gsc.conf import outbound as _outbound
from gsc.conf.outbound import selector as _selector
from gsc.conf.outbound import urltest as _urltest
from gsc.conf.route import rule as _rule
from gsc.group import singbox as _singbox
from gsc.group import smart as _smart


async def main(
    urls: list[str],
    *,
    country: bool = True,
    geo: bool = False,
    ipv: bool = False,
    timeout: float = math.nan,
) -> None:
    conf = _conf.Conf()
    assert conf.outbounds
    outbounds: list[_outbound.Outbound] = await _fetch.fetch(urls)
    need_singbox: bool = geo or ipv or math.isfinite(timeout) and timeout > 0
    groups: list[str]
    if need_singbox:
        singbox = _singbox.SingBox(outbounds)
        groups = await asyncio.gather(
            *[
                _group.group(
                    outbound,
                    country=country,
                    geo=geo,
                    ipv=ipv,
                    timeout=timeout,
                    proxy=singbox.proxy(outbound.tag),
                )
                for outbound in outbounds
            ]
        )
        singbox.proc.terminate()
    else:
        groups = await asyncio.gather(
            *[
                _group.group(
                    outbound, country=country, geo=geo, ipv=ipv, timeout=timeout
                )
                for outbound in outbounds
            ]
        )
    zipped: list[tuple[_outbound.Outbound, str]] = sorted(
        zip(outbounds, groups), key=lambda x: x[1]
    )
    outbounds, groups = zip(*zipped)
    groupby = itertools.groupby(zip(outbounds, groups), key=lambda x: x[1])
    proxy = _smart.Proxy()
    smart_groups: list[_smart.Smart] = [
        proxy,
        _smart.Auto(),
        _smart.OpenAI(),
        _smart.IPv4(),
        _smart.Ipv6(),
    ]
    outbound_groups: list[_outbound.Outbound] = []
    for group, items in groupby:
        outs: list[_outbound.Outbound] = [item[0] for item in items]
        if "OT" in group:
            outbound_groups.append(
                _selector.Selector(tag=group, outbounds=[out.tag for out in outs])
            )
        else:
            outbound_groups.append(
                _urltest.URLTest(tag=group, outbounds=[out.tag for out in outs])
            )
        for smart in smart_groups:
            smart.append(group)
    smart_groups = [smart for smart in smart_groups[1:] if smart.outbounds]
    proxy.outbounds = [
        *[smart.name for smart in smart_groups],
        *[group.tag for group in outbound_groups],
    ]
    conf.outbounds = [
        proxy.build(),
        *[smart.build() for smart in smart_groups],
        *outbound_groups,
        *conf.outbounds,
        *outbounds,
    ]
    if "IPv6" in proxy.outbounds:
        assert conf.route
        assert conf.route.rules
        conf.route.rules.append(_rule.Rule(domain_suffix=["byr.pt"], outbound="IPv6"))
    print(conf.model_dump_json(indent=2, exclude_none=True))
