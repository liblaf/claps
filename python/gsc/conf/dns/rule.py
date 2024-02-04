from collections.abc import Sequence
from typing import Optional

import pydantic


class Rule(pydantic.BaseModel):
    auth_user: Optional[Sequence[str]] = None
    clash_mode: Optional[str] = None
    rule_set: Optional[Sequence[str]] = None
    outbound: Optional[Sequence[str]] = None
    server: str
    disable_cache: Optional[bool] = None
