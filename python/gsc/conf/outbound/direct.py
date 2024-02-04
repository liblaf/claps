from typing import Literal

import pydantic


class Direct(pydantic.BaseModel):
    type: Literal["direct"] = "direct"
    tag: str = "DIRECT"

    def __hash__(self) -> int:
        return hash(self.tag)
