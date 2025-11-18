// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Basic information about a deployment rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleResponseDataAttributes {
    /// The timestamp when the deployment rule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Information about the user who created the deployment rule.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::DeploymentRuleResponseDataAttributesCreatedBy,
    /// Whether this rule is run in dry-run mode.
    #[serde(rename = "dry_run")]
    pub dry_run: bool,
    /// The ID of the deployment gate.
    #[serde(rename = "gate_id")]
    pub gate_id: String,
    /// The name of the deployment rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options for deployment rule response representing either faulty deployment detection or monitor options.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::DeploymentRulesOptions,
    /// The type of the deployment rule.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DeploymentRuleResponseDataAttributesType,
    /// The timestamp when the deployment rule was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Information about the user who updated the deployment rule.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<crate::datadogV2::model::DeploymentRuleResponseDataAttributesUpdatedBy>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleResponseDataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: crate::datadogV2::model::DeploymentRuleResponseDataAttributesCreatedBy,
        dry_run: bool,
        gate_id: String,
        name: String,
        options: crate::datadogV2::model::DeploymentRulesOptions,
        type_: crate::datadogV2::model::DeploymentRuleResponseDataAttributesType,
    ) -> DeploymentRuleResponseDataAttributes {
        DeploymentRuleResponseDataAttributes {
            created_at,
            created_by,
            dry_run,
            gate_id,
            name,
            options,
            type_,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(
        mut self,
        value: crate::datadogV2::model::DeploymentRuleResponseDataAttributesUpdatedBy,
    ) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for DeploymentRuleResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleResponseDataAttributesVisitor {
            type Value = DeploymentRuleResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<
                    crate::datadogV2::model::DeploymentRuleResponseDataAttributesCreatedBy,
                > = None;
                let mut dry_run: Option<bool> = None;
                let mut gate_id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::DeploymentRulesOptions> = None;
                let mut type_: Option<
                    crate::datadogV2::model::DeploymentRuleResponseDataAttributesType,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<
                    crate::datadogV2::model::DeploymentRuleResponseDataAttributesUpdatedBy,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dry_run" => {
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_id" => {
                            gate_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _options) = options {
                                match _options {
                                    crate::datadogV2::model::DeploymentRulesOptions::UnparsedObject(_options) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DeploymentRuleResponseDataAttributesType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let dry_run = dry_run.ok_or_else(|| M::Error::missing_field("dry_run"))?;
                let gate_id = gate_id.ok_or_else(|| M::Error::missing_field("gate_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = DeploymentRuleResponseDataAttributes {
                    created_at,
                    created_by,
                    dry_run,
                    gate_id,
                    name,
                    options,
                    type_,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleResponseDataAttributesVisitor)
    }
}
