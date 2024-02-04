import base64
import pathlib
import subprocess
import sys
import tempfile

from gsc import conf as _conf
from gsc.conf import outbound as _outbound
from gsc.conf import route as _route
from gsc.conf.inbound import mixed as _mixed
from gsc.conf.outbound import dns as _out_dns
from gsc.conf.route import rule as _rule


def encode(name: str) -> str:
    return base64.urlsafe_b64encode(name.encode()).decode().replace("=", "")


class SingBox:
    proc: subprocess.Popen[bytes]
    tmp: tempfile.TemporaryDirectory[str]
    listen_port: int = 62080

    def __init__(self, outbounds: list[_outbound.Outbound]) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        conf = _conf.Conf(
            dns=None,
            inbounds=[
                _mixed.Mixed(
                    listen_port=self.listen_port,
                    users=[
                        _mixed.Mixed.User(
                            username=encode(outbound.tag), password=encode(outbound.tag)
                        )
                        for outbound in outbounds
                    ],
                )
            ],
            outbounds=[_out_dns.DNS(), *outbounds],
            route=_route.Route(
                rules=[
                    _rule.Rule(protocol=["dns"], outbound="DNS"),
                    *[
                        _rule.Rule(
                            auth_user=[encode(outbound.tag)], outbound=outbound.tag
                        )
                        for outbound in outbounds
                    ],
                ],
                rule_set=[],
                final=None,
            ),
            experimental=None,
        )
        (pathlib.Path(self.tmp.name) / "config.json").write_text(
            conf.model_dump_json(indent=2)
        )
        self.proc = subprocess.Popen(
            args=["sing-box", "run", "--directory", self.tmp.name],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=sys.stderr,
        )

    def proxy(self, name: str) -> str:
        name = encode(name)
        return f"http://{name}:{name}@127.0.0.1:{self.listen_port}/"
