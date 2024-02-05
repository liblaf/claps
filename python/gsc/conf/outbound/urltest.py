from typing import Literal, Optional

import pydantic


class URLTest(pydantic.BaseModel):
    type: Literal["urltest"] = "urltest"
    tag: str

    outbounds: list[str] = pydantic.Field(default_factory=list)
    url: Optional[str] = "https://cp.cloudflare.com"

    def __hash__(self) -> int:
        return hash(self.tag)
