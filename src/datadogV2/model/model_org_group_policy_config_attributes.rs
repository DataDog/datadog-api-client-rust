// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an org group policy config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyConfigAttributes {
    /// The allowed values for this config.
    #[serde(rename = "allowed_values")]
    pub allowed_values: Vec<String>,
    /// The default value for this config.
    #[serde(rename = "default_value")]
    pub default_value: serde_json::Value,
    /// The description of the policy config.
    #[serde(rename = "description")]
    pub description: String,
    /// The name of the policy config.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the value for this config.
    #[serde(rename = "value_type")]
    pub value_type: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicyConfigAttributes {
    pub fn new(
        allowed_values: Vec<String>,
        default_value: serde_json::Value,
        description: String,
        name: String,
        value_type: String,
    ) -> OrgGroupPolicyConfigAttributes {
        OrgGroupPolicyConfigAttributes {
            allowed_values,
            default_value,
            description,
            name,
            value_type,
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

impl<'de> Deserialize<'de> for OrgGroupPolicyConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicyConfigAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicyConfigAttributesVisitor {
            type Value = OrgGroupPolicyConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allowed_values: Option<Vec<String>> = None;
                let mut default_value: Option<serde_json::Value> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut value_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowed_values" => {
                            allowed_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_value" => {
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value_type" => {
                            value_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allowed_values =
                    allowed_values.ok_or_else(|| M::Error::missing_field("allowed_values"))?;
                let default_value =
                    default_value.ok_or_else(|| M::Error::missing_field("default_value"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value_type = value_type.ok_or_else(|| M::Error::missing_field("value_type"))?;

                let content = OrgGroupPolicyConfigAttributes {
                    allowed_values,
                    default_value,
                    description,
                    name,
                    value_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyConfigAttributesVisitor)
    }
}
