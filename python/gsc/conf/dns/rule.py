from typing import Optional

import pydantic


class Rule(pydantic.BaseModel):
    auth_user: Optional[list[str]] = None
    domain_suffix: Optional[list[str]] = None
    clash_mode: Optional[str] = None
    rule_set: Optional[list[str]] = None
    outbound: Optional[list[str]] = None
    server: str
    disable_cache: Optional[bool] = None
