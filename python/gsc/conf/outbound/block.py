from typing import Literal

import pydantic


class Block(pydantic.BaseModel):
    type: Literal["block"] = "block"
    tag: str = "BLOCK"
