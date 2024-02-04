from typing import Optional

import pydantic


class Rule(pydantic.BaseModel):
    auth_user: Optional[list[str]] = None
    protocol: Optional[list[str]] = None
    domain_suffix: Optional[list[str]] = None
    ip_is_private: Optional[bool] = None
    clash_mode: Optional[str] = None
    rule_set: Optional[list[str]] = None
    outbound: str
