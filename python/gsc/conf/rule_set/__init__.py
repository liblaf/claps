import pathlib
from typing import Literal, Optional

import pydantic

from gsc.conf import utils


class Local(pydantic.BaseModel):
    type: Literal["local"] = "local"
    tag: str
    format: Literal["source"] | Literal["binary"]

    path: pathlib.Path


class Remote(pydantic.BaseModel):
    type: Literal["remote"] = "remote"
    tag: str
    format: Literal["source"] | Literal["binary"]

    url: str
    download_detour: Optional[str] = None


RuleSet = Local | Remote


def geoip(name: str) -> Remote:
    return Remote(
        tag=f"geoip:{name}",
        format="binary",
        url=utils.proxy(
            f"https://raw.githubusercontent.com/SagerNet/sing-geoip/rule-set/geoip-{name}.srs"
        ),
        download_detour="DIRECT",
    )


def geosite(name: str) -> Remote:
    return Remote(
        tag=f"geosite:{name}",
        format="binary",
        url=utils.proxy(
            f"https://raw.githubusercontent.com/SagerNet/sing-geosite/rule-set/geosite-{name}.srs"
        ),
        download_detour="DIRECT",
    )
