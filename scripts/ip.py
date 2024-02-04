import ipaddress
import socket

info: list[
    tuple[
        socket.AddressFamily,
        socket.SocketKind,
        int,
        str,
        tuple[str, int] | tuple[str, int, int, int],
    ]
] = socket.getaddrinfo(socket.gethostname(), 0)
addrs_uniq: set[ipaddress.IPv4Address | ipaddress.IPv6Address] = set(
    ipaddress.ip_address(i[4][0]) for i in info
)
addrs: list[ipaddress.IPv4Address | ipaddress.IPv6Address] = [
    addr for addr in addrs_uniq if addr.is_global
]
print(*addrs, sep="\n")
