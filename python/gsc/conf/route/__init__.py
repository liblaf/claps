from collections.abc import Sequence
from typing import Optional

import pydantic

from gsc.conf import rule_set as _rule_set
from gsc.conf.route import rule as _rule


class Route(pydantic.BaseModel):
    rules: Optional[Sequence[_rule.Rule]] = pydantic.Field(
        default_factory=lambda: [
            _rule.Rule(protocol=["dns"], outbound="DNS"),
            _rule.Rule(
                ip_is_private=True, rule_set=["geosite:private"], outbound="DIRECT"
            ),
            _rule.Rule(rule_set=["geosite:category-ads-all"], outbound="BLOCK"),
            _rule.Rule(rule_set=["geoip:cn", "geosite:cn"], outbound="DIRECT"),
            _rule.Rule(clash_mode="direct", outbound="DIRECT"),
            _rule.Rule(clash_mode="global", outbound="PROXY"),
            _rule.Rule(
                rule_set=["geosite:bing", "geosite:openai"], outbound="💬 OpenAI"
            ),
        ]
    )
    rule_set: Optional[Sequence[_rule_set.RuleSet]] = pydantic.Field(
        default_factory=lambda: [
            _rule_set.geoip("cn"),
            _rule_set.geosite("bing"),
            _rule_set.geosite("cn"),
            _rule_set.geosite("openai"),
            _rule_set.geosite("private"),
        ]
    )
    final: Optional[str] = "PROXY"
    auto_detect_interface: Optional[bool] = True
