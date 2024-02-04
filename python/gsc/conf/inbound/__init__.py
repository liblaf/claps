import pydantic

from gsc.conf.inbound import mixed as _mixed


class _Inbound(pydantic.BaseModel):
    type: str
    tag: str


Inbound = _mixed.Mixed | _Inbound
