from collections.abc import Sequence
from typing import Optional

import pydantic

from gsc.conf.dns import rule as _rule
from gsc.conf.dns import server as _server


class DNS(pydantic.BaseModel):
    servers: Optional[Sequence[_server.Server]] = pydantic.Field(
        default_factory=lambda: [
            _server.Server(
                tag="dns-!cn",
                address="https://cloudflare-dns.com/dns-query",
                address_resolver="dns-cn",
                detour="PROXY",
            ),
            _server.Server(
                tag="dns-cn",
                address="https://dns.tuna.tsinghua.edu.cn:8443/dns-query",
                address_resolver="dns-local",
                detour="DIRECT",
            ),
            _server.Server(tag="dns-local", address="local", detour="DIRECT"),
            _server.Server(tag="dns-block", address="rcode://success"),
        ]
    )
    rules: Optional[Sequence[_rule.Rule]] = pydantic.Field(
        default_factory=lambda: [
            _rule.Rule(outbound=["any"], server="dns-cn"),
            _rule.Rule(
                rule_set=["geosite:category-ads-all"],
                server="dns-block",
                disable_cache=True,
            ),
            _rule.Rule(rule_set=["geosite:private"], server="dns-local"),
            _rule.Rule(rule_set=["geosite:cn"], server="dns-cn"),
        ]
    )
    final: Optional[str] = "dns-!cn"
