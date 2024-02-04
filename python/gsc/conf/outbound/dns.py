from typing import Literal

import pydantic


class DNS(pydantic.BaseModel):
    type: Literal["dns"] = "dns"
    tag: str = "DNS"

    def __hash__(self) -> int:
        return hash(self.tag)
