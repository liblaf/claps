from typing import Literal, Optional

import pydantic

Level = (
    Literal["trace"]
    | Literal["debug"]
    | Literal["info"]
    | Literal["warn"]
    | Literal["error"]
    | Literal["fatal"]
    | Literal["panic"]
)


class Log(pydantic.BaseModel):
    disabled: Optional[bool] = False
    level: Optional[Level] = "info"
    timestamp: Optional[bool] = True
