from collections.abc import Sequence
from typing import Literal, Optional

import pydantic


class Mixed(pydantic.BaseModel):
    type: Literal["Mixed"] = "Mixed"
    tag: str = "in-mixed"

    # Listen Fields
    listen: str = "127.0.0.1"
    listen_port: Optional[int] = 2080

    class User(pydantic.BaseModel):
        username: str
        password: str

    users: Optional[Sequence[User]] = None
