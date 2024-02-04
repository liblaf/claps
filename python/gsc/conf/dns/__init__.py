from typing import Optional

import pydantic

from gsc.auto import dns as _auto
from gsc.conf.dns import rule as _rule
from gsc.conf.dns import server as _server


class DNS(pydantic.BaseModel):
    servers: Optional[list[_server.Server]] = pydantic.Field(
        default_factory=lambda: [
            _server.Server(
                tag="dns-!cn",
                address="https://cloudflare-dns.com/dns-query",
                address_resolver="dns-cn",
                detour="PROXY",
            ),
            _server.Server(
                tag="dns-cn",
                address=_auto.dns(),
                address_resolver="dns-local",
                detour="DIRECT",
            ),
            _server.Server(tag="dns-local", address="local", detour="DIRECT"),
            _server.Server(tag="dns-block", address="rcode://success"),
        ]
    )
    rules: Optional[list[_rule.Rule]] = pydantic.Field(
        default_factory=lambda: [
            _rule.Rule(outbound=["any"], server="dns-cn"),
            _rule.Rule(
                rule_set=["geosite:category-ads-all"],
                server="dns-block",
                disable_cache=True,
            ),
            _rule.Rule(rule_set=["geosite:private"], server="dns-local"),
            _rule.Rule(clash_mode="direct", server="dns-cn"),
            _rule.Rule(clash_mode="global", server="dns-!cn"),
            _rule.Rule(rule_set=["geosite:cn"], server="dns-cn"),
        ]
    )
    final: Optional[str] = "dns-!cn"
