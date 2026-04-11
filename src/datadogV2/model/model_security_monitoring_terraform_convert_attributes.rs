// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the convert request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringTerraformConvertAttributes {
    /// The resource attributes as a JSON object, matching the structure returned by the corresponding Datadog API (for example, the attributes of a suppression rule).
    #[serde(rename = "resource_json")]
    pub resource_json: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringTerraformConvertAttributes {
    pub fn new(
        resource_json: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> SecurityMonitoringTerraformConvertAttributes {
        SecurityMonitoringTerraformConvertAttributes {
            resource_json,
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

impl<'de> Deserialize<'de> for SecurityMonitoringTerraformConvertAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringTerraformConvertAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringTerraformConvertAttributesVisitor {
            type Value = SecurityMonitoringTerraformConvertAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut resource_json: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "resource_json" => {
                            resource_json =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let resource_json =
                    resource_json.ok_or_else(|| M::Error::missing_field("resource_json"))?;

                let content = SecurityMonitoringTerraformConvertAttributes {
                    resource_json,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringTerraformConvertAttributesVisitor)
    }
}
