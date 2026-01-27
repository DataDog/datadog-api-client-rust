// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A WAF policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityPolicyAttributes {
    /// Description of the WAF policy.
    #[serde(rename = "description")]
    pub description: String,
    /// Make this policy the default policy. The default policy is applied to every services not specifically added to another policy.
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
    /// The Name of the WAF policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Presets enabled on this policy.
    #[serde(rename = "protectionPresets")]
    pub protection_presets: Option<Vec<String>>,
    /// Rule overrides applied by the policy.
    #[serde(rename = "rules")]
    pub rules: Option<Vec<crate::datadogV2::model::ApplicationSecurityPolicyRuleOverride>>,
    /// The scope of the WAF policy.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<crate::datadogV2::model::ApplicationSecurityPolicyScope>>,
    /// Version of the WAF ruleset maintained by Datadog used by this policy. 0 is the default value.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityPolicyAttributes {
    pub fn new(description: String, name: String) -> ApplicationSecurityPolicyAttributes {
        ApplicationSecurityPolicyAttributes {
            description,
            is_default: None,
            name,
            protection_presets: None,
            rules: None,
            scope: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn protection_presets(mut self, value: Vec<String>) -> Self {
        self.protection_presets = Some(value);
        self
    }

    pub fn rules(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationSecurityPolicyRuleOverride>,
    ) -> Self {
        self.rules = Some(value);
        self
    }

    pub fn scope(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationSecurityPolicyScope>,
    ) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for ApplicationSecurityPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityPolicyAttributesVisitor {
            type Value = ApplicationSecurityPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut is_default: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut protection_presets: Option<Vec<String>> = None;
                let mut rules: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityPolicyRuleOverride>,
                > = None;
                let mut scope: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityPolicyScope>,
                > = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isDefault" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protectionPresets" => {
                            if v.is_null() {
                                continue;
                            }
                            protection_presets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            if v.is_null() {
                                continue;
                            }
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ApplicationSecurityPolicyAttributes {
                    description,
                    is_default,
                    name,
                    protection_presets,
                    rules,
                    scope,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityPolicyAttributesVisitor)
    }
}
