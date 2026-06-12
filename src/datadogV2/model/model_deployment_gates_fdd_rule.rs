// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A faulty deployment detection rule to evaluate as part of a deployment gate evaluation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGatesFDDRule {
    /// Rule-level dry run. When enabled, the rule is evaluated normally but it always returns `pass`. The real result is visible in the Datadog UI.
    #[serde(rename = "dry_run")]
    pub dry_run: Option<bool>,
    /// Human-readable name for this rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options for a `faulty_deployment_detection` rule.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::DeploymentGatesFDDRuleOptions>,
    /// The type identifier for a faulty deployment detection rule.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DeploymentGatesFDDRuleType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGatesFDDRule {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::DeploymentGatesFDDRuleType,
    ) -> DeploymentGatesFDDRule {
        DeploymentGatesFDDRule {
            dry_run: None,
            name,
            options: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn options(
        mut self,
        value: crate::datadogV2::model::DeploymentGatesFDDRuleOptions,
    ) -> Self {
        self.options = Some(value);
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

impl<'de> Deserialize<'de> for DeploymentGatesFDDRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGatesFDDRuleVisitor;
        impl<'a> Visitor<'a> for DeploymentGatesFDDRuleVisitor {
            type Value = DeploymentGatesFDDRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::DeploymentGatesFDDRuleOptions> =
                    None;
                let mut type_: Option<crate::datadogV2::model::DeploymentGatesFDDRuleType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dry_run" => {
                            if v.is_null() {
                                continue;
                            }
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DeploymentGatesFDDRuleType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = DeploymentGatesFDDRule {
                    dry_run,
                    name,
                    options,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGatesFDDRuleVisitor)
    }
}
