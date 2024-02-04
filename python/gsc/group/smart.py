import abc

from gsc.conf import outbound as _outbound
from gsc.conf.outbound import selector as _selector
from gsc.conf.outbound import urltest as _urltest


class Smart:
    name: str
    outbounds: list[str]

    def __init__(self) -> None:
        self.outbounds = []

    @abc.abstractmethod
    def build(self) -> _outbound.Outbound:
        ...

    @abc.abstractmethod
    def append(self, group: str) -> None:
        ...


class Proxy(Smart):
    name: str = "PROXY"

    def build(self) -> _outbound.Outbound:
        return _selector.Selector(
            tag=self.name, outbounds=self.outbounds, default="🚀 Auto"
        )

    def append(self, group: str) -> None:
        self.outbounds.append(group)


class Auto(Smart):
    name: str = "🚀 Auto"

    def build(self) -> _outbound.Outbound:
        return _urltest.URLTest(tag=self.name, outbounds=self.outbounds)

    def append(self, group: str) -> None:
        if "OT" in group:
            return
        self.outbounds.append(group)


class OpenAI(Smart):
    name: str = "💬 OpenAI"

    def build(self) -> _outbound.Outbound:
        return _urltest.URLTest(tag=self.name, outbounds=self.outbounds)

    def append(self, group: str) -> None:
        if "US" in group:
            self.outbounds.append(group)


class IPv4(Smart):
    name: str = "IPv4"

    def build(self) -> _outbound.Outbound:
        return _urltest.URLTest(tag=self.name, outbounds=self.outbounds)

    def append(self, group: str) -> None:
        if ("IPv4" in group) or ("IPv4/6" in group):
            self.outbounds.append(group)


class Ipv6(Smart):
    name: str = "IPv6"

    def build(self) -> _outbound.Outbound:
        return _urltest.URLTest(tag=self.name, outbounds=self.outbounds)

    def append(self, group: str) -> None:
        if ("IPv6" in group) or ("IPv4/6" in group):
            self.outbounds.append(group)
