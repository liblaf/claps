from typing import Literal, Optional

import pydantic


class Selector(pydantic.BaseModel):
    type: Literal["selector"] = "selector"
    tag: str

    outbounds: list[str] = pydantic.Field(default_factory=list)
    default: Optional[str] = None

    def __hash__(self) -> int:
        return hash(self.tag)
