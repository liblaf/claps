from typing import Optional

import pydantic

from gsc.conf import dns as _dns
from gsc.conf import experimental as _experimental
from gsc.conf import inbound as _inbound
from gsc.conf import log as _log
from gsc.conf import outbound as _outbound
from gsc.conf import route as _route
from gsc.conf.inbound import mixed as _mixed
from gsc.conf.outbound import block as _block
from gsc.conf.outbound import direct as _direct
from gsc.conf.outbound import dns as _out_dns


class Conf(pydantic.BaseModel):
    log: Optional[_log.Log] = pydantic.Field(default_factory=_log.Log)
    dns: Optional[_dns.DNS] = pydantic.Field(default_factory=_dns.DNS)
    inbounds: Optional[list[_inbound.Inbound]] = pydantic.Field(
        default_factory=lambda: [_mixed.Mixed()]
    )
    outbounds: Optional[list[_outbound.Outbound]] = pydantic.Field(
        default_factory=lambda: [
            _direct.Direct(),
            _block.Block(),
            _out_dns.DNS(),
        ]
    )
    route: Optional[_route.Route] = pydantic.Field(default_factory=_route.Route)
    experimental: Optional[_experimental.Experimental] = pydantic.Field(
        default_factory=_experimental.Experimental
    )
