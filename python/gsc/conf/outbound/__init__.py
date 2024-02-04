import pydantic

from gsc.conf.outbound import block as _block
from gsc.conf.outbound import direct as _direct
from gsc.conf.outbound import dns as _dns
from gsc.conf.outbound import selector as _selector
from gsc.conf.outbound import urltest as _urltest


class _Outbound(pydantic.BaseModel):
    model_config = pydantic.ConfigDict(extra="allow")
    type: str
    tag: str


Outbound = (
    _direct.Direct
    | _block.Block
    | _dns.DNS
    | _selector.Selector
    | _urltest.URLTest
    | _Outbound
)
