// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parameters for creating a deployment rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateDeploymentRuleParamsDataAttributes {
    /// Whether this rule is run in dry-run mode.
    #[serde(rename = "dry_run")]
    pub dry_run: Option<bool>,
    /// The name of the deployment rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options for deployment rule response representing either faulty deployment detection or monitor options. The actual type is determined by the parent's 'type' field.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::DeploymentRulesOptions,
    /// The type of the deployment rule (faulty_deployment_detection or monitor).
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateDeploymentRuleParamsDataAttributes {
    pub fn new(
        name: String,
        options: crate::datadogV2::model::DeploymentRulesOptions,
        type_: String,
    ) -> CreateDeploymentRuleParamsDataAttributes {
        CreateDeploymentRuleParamsDataAttributes {
            dry_run: None,
            name,
            options,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
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

impl<'de> Deserialize<'de> for CreateDeploymentRuleParamsDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateDeploymentRuleParamsDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateDeploymentRuleParamsDataAttributesVisitor {
            type Value = CreateDeploymentRuleParamsDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::DeploymentRulesOptions> = None;
                let mut type_: Option<String> = None;
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
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CreateDeploymentRuleParamsDataAttributes {
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

        deserializer.deserialize_any(CreateDeploymentRuleParamsDataAttributesVisitor)
    }
}
