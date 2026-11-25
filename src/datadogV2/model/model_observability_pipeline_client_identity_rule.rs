// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A named rule. It matches when every constraint it defines matches at least one identity in the client's certificate. It must define at least one of `common_names`, `dns_sans`, or `uri_sans`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineClientIdentityRule {
    /// Patterns for the certificate subject common name. Matching is case-sensitive, and `*` matches any characters, including dots.
    #[serde(rename = "common_names")]
    pub common_names: Option<Vec<String>>,
    /// Patterns for DNS subject alternative names. Matching is case-insensitive, and `*` matches any characters within one DNS label.
    #[serde(rename = "dns_sans")]
    pub dns_sans: Option<Vec<String>>,
    /// Unique rule name, reported in metrics.
    #[serde(rename = "name")]
    pub name: String,
    /// Patterns for URI subject alternative names. Matching is case-sensitive. `*` matches any characters within one path segment, and a trailing `/**` matches the path and everything below it. Wildcards are only supported in the path.
    #[serde(rename = "uri_sans")]
    pub uri_sans: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineClientIdentityRule {
    pub fn new(name: String) -> ObservabilityPipelineClientIdentityRule {
        ObservabilityPipelineClientIdentityRule {
            common_names: None,
            dns_sans: None,
            name,
            uri_sans: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn common_names(mut self, value: Vec<String>) -> Self {
        self.common_names = Some(value);
        self
    }

    pub fn dns_sans(mut self, value: Vec<String>) -> Self {
        self.dns_sans = Some(value);
        self
    }

    pub fn uri_sans(mut self, value: Vec<String>) -> Self {
        self.uri_sans = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineClientIdentityRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineClientIdentityRuleVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineClientIdentityRuleVisitor {
            type Value = ObservabilityPipelineClientIdentityRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut common_names: Option<Vec<String>> = None;
                let mut dns_sans: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut uri_sans: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "common_names" => {
                            if v.is_null() {
                                continue;
                            }
                            common_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dns_sans" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_sans = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uri_sans" => {
                            if v.is_null() {
                                continue;
                            }
                            uri_sans = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ObservabilityPipelineClientIdentityRule {
                    common_names,
                    dns_sans,
                    name,
                    uri_sans,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineClientIdentityRuleVisitor)
    }
}
