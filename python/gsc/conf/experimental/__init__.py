from typing import Optional

import pydantic

from gsc.conf.experimental import cache_file as _cache_file
from gsc.conf.experimental import clash_api as _clash_api


class Experimental(pydantic.BaseModel):
    cache_file: Optional[_cache_file.CacheFile] = pydantic.Field(
        default_factory=_cache_file.CacheFile
    )
    clash_api: Optional[_clash_api.ClashAPI] = pydantic.Field(
        default_factory=_clash_api.ClashAPI
    )
