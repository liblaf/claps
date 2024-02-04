from collections.abc import Sequence
from typing import Optional

import pydantic


class Rule(pydantic.BaseModel):
    auth_user: Optional[Sequence[str]] = None
    protocol: Optional[Sequence[str]] = None
    ip_is_private: Optional[bool] = None
    clash_mode: Optional[str] = None
    rule_set: Optional[Sequence[str]] = None
    outbound: str
