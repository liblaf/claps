from typing import Optional

import pydantic


class Server(pydantic.BaseModel):
    tag: str
    address: str
    address_resolver: Optional[str] = None
    detour: Optional[str] = None
