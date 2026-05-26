// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Deprecated: Override WAF ruleset parameters. Use `protectionPresets` instead.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityPolicyRulesetOverride {
    /// When blocking is enabled, the ruleset will block the traffic it matches.
    #[serde(rename = "blocking")]
    pub blocking: bool,
    /// When false, this ruleset will not match any traffic.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The identifier of the ruleset to override.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityPolicyRulesetOverride {
    pub fn new(
        blocking: bool,
        enabled: bool,
        id: String,
    ) -> ApplicationSecurityPolicyRulesetOverride {
        ApplicationSecurityPolicyRulesetOverride {
            blocking,
            enabled,
            id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityPolicyRulesetOverride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityPolicyRulesetOverrideVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityPolicyRulesetOverrideVisitor {
            type Value = ApplicationSecurityPolicyRulesetOverride;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut blocking: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "blocking" => {
                            blocking = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let blocking = blocking.ok_or_else(|| M::Error::missing_field("blocking"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = ApplicationSecurityPolicyRulesetOverride {
                    blocking,
                    enabled,
                    id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityPolicyRulesetOverrideVisitor)
    }
}
