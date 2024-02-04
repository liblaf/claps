from collections.abc import Sequence

from gsc import conf as _conf


async def main(urls: Sequence[str]) -> None:
    conf = _conf.Conf()
    print(conf.model_dump_json(indent=2, exclude_none=True))
    raise NotImplementedError()
