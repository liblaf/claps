# Route

```python
def handle(request):
    match request:
        case Domain(domain):
            if domain is PRIVATE:
                ip = resolve(domain)
                return DIRECT(ip)
            elif domain in GEOSITE["ads"]:
                return BLOCK
            elif domain in GEOSITE["cn"]:
                ip = resolve(domain)
                return DIRECT(ip)
            elif domain in GEOSITE["bing", "openai"]:
                return PROXY_US(domain)
            else:
                ip = resolve(domain)
                if ip in GEOIP["cn"]:
                    return DIRECT(ip)
                else:
                    return PROXY(domain)
        case IP(ip):
            if ip is PRIVATE:
                return DIRECT(ip)
            elif ip in GEOIP["cn"]:
                return DIRECT(ip)
            else:
                return PROXY(ip)


def resolve(domain):
    if domain == DNS_CN:
        return DNS_LOCAL(domain)
    elif domain == DNS_CF:
        return DNS_CN(domain)

    if domain is PRIVATE:
        return DNS_LOCAL(domain)
    elif domain in GEOSITE["ads"]:
        return BLOCK
    elif domain in GEOSITE["cn"]:
        return DNS_CN(domain)
    else:
        return DNS_CF(domain)
```
