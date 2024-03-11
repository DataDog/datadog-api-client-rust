// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options for cloud_configuration rules.
/// Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.
///
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationComplianceRuleOptions {
    /// Whether the rule is a complex one.
    /// Must be set to true if `regoRule.resourceTypes` contains more than one item. Defaults to false.
    ///
    #[serde(rename = "complexRule")]
    pub complex_rule: Option<bool>,
    /// Rule details.
    #[serde(rename = "regoRule")]
    pub rego_rule: Option<crate::datadogV2::model::CloudConfigurationRegoRule>,
    /// Main resource type to be checked by the rule. It should be specified again in `regoRule.resourceTypes`.
    ///
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationComplianceRuleOptions {
    pub fn new() -> CloudConfigurationComplianceRuleOptions {
        CloudConfigurationComplianceRuleOptions {
            complex_rule: None,
            rego_rule: None,
            resource_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn complex_rule(&mut self, value: bool) -> &mut Self {
        self.complex_rule = Some(value);
        self
    }

    pub fn rego_rule(
        &mut self,
        value: crate::datadogV2::model::CloudConfigurationRegoRule,
    ) -> &mut Self {
        self.rego_rule = Some(value);
        self
    }

    pub fn resource_type(&mut self, value: String) -> &mut Self {
        self.resource_type = Some(value);
        self
    }
}

impl Default for CloudConfigurationComplianceRuleOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudConfigurationComplianceRuleOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationComplianceRuleOptionsVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationComplianceRuleOptionsVisitor {
            type Value = CloudConfigurationComplianceRuleOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut complex_rule: Option<bool> = None;
                let mut rego_rule: Option<crate::datadogV2::model::CloudConfigurationRegoRule> =
                    None;
                let mut resource_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "complexRule" => {
                            if v.is_null() {
                                continue;
                            }
                            complex_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regoRule" => {
                            if v.is_null() {
                                continue;
                            }
                            rego_rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resourceType" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudConfigurationComplianceRuleOptions {
                    complex_rule,
                    rego_rule,
                    resource_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationComplianceRuleOptionsVisitor)
    }
}
