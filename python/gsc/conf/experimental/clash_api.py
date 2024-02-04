import pathlib
from typing import Optional

import pydantic

from gsc.conf import utils


class ClashAPI(pydantic.BaseModel):
    external_controller: Optional[str] = "127.0.0.1:9090"
    external_ui: Optional[pathlib.Path] = pathlib.Path("ui")
    external_ui_download_url: Optional[str] = utils.proxy(
        "https://github.com/MetaCubeX/Yacd-meta/archive/gh-pages.zip"
    )
    external_ui_download_detour: Optional[str] = "DIRECT"
    secret: Optional[str] = None
    default_mode: Optional[str] = "rule"
