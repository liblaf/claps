import asyncio
import os
import sys

from gsc import main as _main


def main() -> None:
    urls: list[str] = [url.strip() for url in sys.argv[1:]]
    asyncio.run(
        _main.main(
            urls,
            country=os.getenv("COUNTRY", "true") == "true",
            geo=os.getenv("GEO") == "true",
            ipv=os.getenv("IPV") == "true",
            timeout=float(os.getenv("TIMEOUT", "nan")),
        )
    )


if __name__ == "__main__":
    main()
