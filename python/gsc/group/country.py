import functools
import re
from typing import Optional

from gsc.conf import outbound as _outbound

PROVIDER2PATTERN: dict[str, dict[str, str]] = {
    "default": {
        "🇦🇷 AR 阿根廷": r"🇦🇷|\bAR\b|阿根廷",
        "🇦🇺 AU 澳大利亚": r"🇦🇺|\bAU\b|澳大利亚",
        "🇧🇷 BR 巴西": r"🇧🇷|\bBR\b|巴西",
        "🇨🇦 CA 加拿大": r"🇨🇦|\bCA\b|加拿大",
        "🇨🇭 CH 瑞士": r"🇨🇭|\bCH\b|瑞士",
        "🇩🇪 DE 德国": r"🇩🇪|\bDE\b|德国",
        "🇪🇸 ES 西班牙": r"🇪🇸|\bES\b|西班牙",
        "🇫🇷 FR 法国": r"🇫🇷|\bFR\b|法国",
        "🇬🇧 UK 英国": r"🇬🇧|\bUK\b|英国",
        "🇭🇰 HK 香港": r"🇭🇰|\bHK\b|港",
        "🇮🇪 IE 爱尔兰": r"🇮🇪|\bIE\b|爱尔兰",
        "🇮🇱 IL 以色列": r"🇮🇱|\bIL\b|以色列",
        "🇮🇳 IN 印度": r"🇮🇳|\bIN\b|印度",
        "🇯🇵 JP 日本": r"🇯🇵|\bJP\b|日|东京",
        "🇰🇷 KR 韩国": r"🇰🇷|\bKR\b|韩国",
        "🇳🇱 NL 荷兰": r"🇳🇱|\bNL\b|荷兰",
        "🇳🇴 NO 挪威": r"🇳🇴|\bNO\b|挪威",
        "🇷🇺 RU 俄罗斯": r"🇷🇺|\bRU\b|俄罗斯",
        "🇷🇺 SG 新加坡": r"🇷🇺|\bSG\b|新",
        "🇸🇪 SE 瑞典": r"🇸🇪|\bSE\b|瑞典",
        "🇹🇭 TH 泰国": r"🇹🇭|\bTH\b|泰国",
        "🇹🇷 TR 土耳其": r"🇹🇷|\bTR\b|土耳其",
        "🇹🇼 TW 台湾": r"🇹🇼|\bTW\b|台",
        "🇺🇦 UA 乌克兰": r"🇺🇦|\bUA\b|乌克兰",
        "🇺🇸 US 美国": r"🇺🇸|\bUS\b|美",
        "🇿🇦 ZA 南非": r"🇿🇦|\bZA\b|南非",
    },
    "FastLink": {
        "🇦🇷 AR 阿根廷": r"阿根廷",
        "🇦🇺 AU 澳大利亚": r"澳大利亚",
        "🇧🇷 BR 巴西": r"巴西",
        "🇨🇦 CA 加拿大": r"加拿大",
        "🇩🇪 DE 德国": r"德国",
        "🇫🇷 FR 法国": r"法国",
        "🇬🇧 UK 英国": r"英国",
        "🇭🇰 HK 香港": r"香港|广港",
        "🇮🇳 IN 印度": r"印度",
        "🇯🇵 JP 日本": r"日本|广日",
        "🇰🇷 KR 韩国": r"韩国",
        "🇳🇱 NL 荷兰": r"荷兰",
        "🇷🇺 RU 俄罗斯": r"俄罗斯",
        "🇷🇺 SG 新加坡": r"新加坡|广新",
        "🇹🇭 TH 泰国": r"泰国",
        "🇹🇷 TR 土耳其": r"土耳其",
        "🇹🇼 TW 台湾": r"台湾|广台",
        "🇺🇸 US 美国": r"美国|广美",
    },
    "FlyingBird": {
        "🇦🇷 AR 阿根廷": r"Argentina-\d{2}",
        "🇬🇧 UK 英国": r"UK-\d{2}",
        "🇭🇰 HK 香港": r"Hong Kong-\d{2}",
        "🇯🇵 JP 日本": r"Japan-\d{2}",
        "🇲🇾 MY 马来西亚": r"Malaysia-\d{2}",
        "🇷🇺 SG 新加坡": r"Singapore-\d{2}",
        "🇹🇷 TR 土耳其": r"Turkey-\d{2}",
        "🇹🇼 TW 台湾": r"Taiwan-\d{2}",
        "🇺🇸 US 美国": r"USA-\d{2}",
    },
    "NTHU.CC": {
        "🇬🇧 UK 英国": r"英国",
        "🇭🇰 HK 香港": r"香港",
        "🇯🇵 JP 日本": r"东京",
        "🇰🇷 KR 韩国": r"韩国",
        "🇷🇺 SG 新加坡": r"新加坡",
        "🇹🇷 TR 土耳其": r"土耳其",
        "🇹🇼 TW 台湾": r"台湾",
        "🇺🇸 US 美国": r"美国",
    },
}


@functools.cache
def country(outbound: _outbound.Outbound) -> Optional[str]:
    match: Optional[re.Match[str]] = re.fullmatch(
        r"(?P<tag>.+) \[(?P<provider>.+)\]", outbound.tag
    )
    tag: str
    pattern: dict[str, str]
    if match:
        tag = match.group("tag")
        provider: str = match.group("provider")
        pattern = PROVIDER2PATTERN.get(provider, PROVIDER2PATTERN["default"])
    else:
        tag = outbound.tag
        pattern = PROVIDER2PATTERN["default"]
    for country, regex in pattern.items():
        if re.search(regex, tag):
            return country
