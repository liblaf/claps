from typing import Literal

import pydantic


class Block(pydantic.BaseModel):
    type: Literal["block"] = "block"
    tag: str = "BLOCK"

    def __hash__(self) -> int:
        return hash(self.tag)
