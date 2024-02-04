from collections.abc import Sequence
from typing import Literal

import pydantic


class URLTest(pydantic.BaseModel):
    type: Literal["urltest"] = "urltest"
    tag: str

    outbounds: Sequence[str] = pydantic.Field(default_factory=list)
