use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use anyhow::Result;

use claps::common::log::LogResult;

pub fn addrs() -> Result<Vec<IpAddr>> {
    Ok(local_ip_address::list_afinet_netifas()
        .log()?
        .iter()
        .filter_map(|(_name, addr)| {
            if addr.is_global_() {
                Some(addr.to_owned())
            } else {
                None
            }
        })
        .collect())
}

trait IsGlobal {
    fn is_global_(&self) -> bool;
}

trait V4IsGlobal {
    fn is_global_(&self) -> bool;
}

trait V6IsGlobal {
    fn is_global_(&self) -> bool;
    fn is_unicast_link_local_(&self) -> bool;
}

impl IsGlobal for IpAddr {
    fn is_global_(&self) -> bool {
        match self {
            IpAddr::V4(addr) => addr.is_global_(),
            IpAddr::V6(addr) => addr.is_global_(),
        }
    }
}

impl V4IsGlobal for Ipv4Addr {
    fn is_global_(&self) -> bool {
        !(self.octets()[0] == 0 // "This network"
          || self.is_private()
          || self.is_loopback()
          || self.is_link_local()
          // addresses reserved for future protocols (`192.0.0.0/24`)
          ||(self.octets()[0] == 192 && self.octets()[1] == 0 && self.octets()[2] == 0)
          || self.is_documentation()
          || self.is_broadcast())
    }
}

impl V6IsGlobal for Ipv6Addr {
    fn is_global_(&self) -> bool {
        !(self.is_unspecified()
            || self.is_loopback()
            // IPv4-mapped Address (`::ffff:0:0/96`)
            || matches!(self.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
            // IPv4-IPv6 Translat. (`64:ff9b:1::/48`)
            || matches!(self.segments(), [0x64, 0xff9b, 1, _, _, _, _, _])
            // Discard-Only Address Block (`100::/64`)
            || matches!(self.segments(), [0x100, 0, 0, 0, _, _, _, _])
            // IETF Protocol Assignments (`2001::/23`)
            || (matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if b < 0x200)
                && !(
                    // Port Control Protocol Anycast (`2001:1::1`)
                    u128::from_be_bytes(self.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0001
                    // Traversal Using Relays around NAT Anycast (`2001:1::2`)
                    || u128::from_be_bytes(self.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0002
                    // AMT (`2001:3::/32`)
                    || matches!(self.segments(), [0x2001, 3, _, _, _, _, _, _])
                    // AS112-v6 (`2001:4:112::/48`)
                    || matches!(self.segments(), [0x2001, 4, 0x112, _, _, _, _, _])
                    // ORCHIDv2 (`2001:20::/28`)
                    || matches!(self.segments(), [0x2001, b, _, _, _, _, _, _] if (0x20..=0x2F).contains(&b))
                ))
            || self.is_unicast_link_local_())
    }

    fn is_unicast_link_local_(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_ipv4_is_global() {
        let addr = IpAddr::from_str("127.0.0.1").unwrap();
        assert!(!addr.is_global_());
        let addr = IpAddr::from_str("183.173.142.216").unwrap();
        assert!(addr.is_global_());
        let addr = IpAddr::from_str("172.17.0.1").unwrap();
        assert!(!addr.is_global_());
    }

    #[test]
    fn test_ipv6_is_global() {
        let addr = IpAddr::from_str("::1").unwrap();
        assert!(!addr.is_global_());
        let addr = IpAddr::from_str("2402:f000:3:8801:fd1:9ac2:8087:fae8").unwrap();
        assert!(addr.is_global_());
        let addr = IpAddr::from_str("fe80::85f0:7ccd:2aff:70f5").unwrap();
        assert!(!addr.is_global_());
        let addr = IpAddr::from_str("fe80::42:5bff:fed7:d2bf").unwrap();
        assert!(!addr.is_global_());
    }
}
