from typing import Literal

import pydantic


class Direct(pydantic.BaseModel):
    type: Literal["direct"] = "direct"
    tag: str = "DIRECT"
