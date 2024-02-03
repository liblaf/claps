import ipaddress
import socket
from collections.abc import Sequence, Set

info: Sequence[
    Sequence[
        socket.AddressFamily,
        socket.SocketKind,
        int,
        str,
        Sequence[str, int] | Sequence[str, int, int, int],
    ]
] = socket.getaddrinfo(socket.gethostname(), 0)
addrs_uniq: Set[ipaddress.IPv4Address | ipaddress.IPv6Address] = set(
    ipaddress.ip_address(i[4][0]) for i in info
)
addrs: Sequence[ipaddress.IPv4Address | ipaddress.IPv6Address] = [
    addr for addr in addrs_uniq if addr.is_global
]
print(*addrs, sep="\n")
