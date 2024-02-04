import asyncio
import sys
from collections.abc import Sequence

from gsc import main as _main


def main() -> None:
    urls: Sequence[str] = sys.stdin.readlines()
    asyncio.run(_main.main(urls))


if __name__ == "__main__":
    main()
