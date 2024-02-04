from typing import Optional

import pydantic


class Listen(pydantic.BaseModel):
    listen: str
    listen_port: Optional[int] = None
