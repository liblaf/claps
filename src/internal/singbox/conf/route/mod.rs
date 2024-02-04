use serde::{Deserialize, Serialize};

use self::rule::Rule;

pub mod rule;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    rules: Option<Vec<Rule>>,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            rules: Some(vec![
                Rule {
                    protocol: Some(vec!["dns".to_string()]),
                    outbound: "DNS".to_string(),
                    ..Default::default()
                },
                Rule {
                    rule_set: Some(vec!["geosite:category-ads-all".to_string()]),
                    outbound: "BLOCK".to_string(),
                    ..Default::default()
                },
                Rule {
                    ip_is_private: Some(true),
                    rule_set: Some(vec!["geosite:private".to_string()]),
                    outbound: "DIRECT".to_string(),
                    ..Default::default()
                },
                Rule {
                    clash_mode: Some("direct".to_string()),
                    outbound: "DIRECT".to_string(),
                    ..Default::default()
                },
                Rule {
                    clash_mode: Some("global".to_string()),
                    outbound: "PROXY".to_string(),
                    ..Default::default()
                },
                Rule {
                    rule_set: Some(vec!["geoip:cn".to_string(), "geosite:cn".to_string()]),
                    outbound: "DIRECT".to_string(),
                    ..Default::default()
                },
                Rule {
                    rule_set: Some(vec![
                        "geosite:bing".to_string(),
                        "geosite:openai".to_string(),
                    ]),
                    outbound: "💬 OpenAI".to_string(),
                    ..Default::default()
                },
            ]),
        }
    }
}
