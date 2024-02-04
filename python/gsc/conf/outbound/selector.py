from collections.abc import Sequence
from typing import Literal, Optional

import pydantic


class Selector(pydantic.BaseModel):
    type: Literal["selector"] = "selector"
    tag: str

    outbounds: Sequence[str] = pydantic.Field(default_factory=list)
    default: Optional[str] = None
