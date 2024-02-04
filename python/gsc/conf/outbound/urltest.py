from typing import Literal

import pydantic


class URLTest(pydantic.BaseModel):
    type: Literal["urltest"] = "urltest"
    tag: str

    outbounds: list[str] = pydantic.Field(default_factory=list)

    def __hash__(self) -> int:
        return hash(self.tag)
