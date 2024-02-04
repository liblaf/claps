from typing import Optional

import pydantic


class CacheFile(pydantic.BaseModel):
    enabled: Optional[bool] = True
