// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Terraform export response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringTerraformExportAttributes {
    /// The Terraform configuration for the resource.
    #[serde(rename = "output")]
    pub output: Option<String>,
    /// The ID of the exported resource.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The Terraform resource type name.
    #[serde(rename = "type_name")]
    pub type_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringTerraformExportAttributes {
    pub fn new(
        resource_id: String,
        type_name: String,
    ) -> SecurityMonitoringTerraformExportAttributes {
        SecurityMonitoringTerraformExportAttributes {
            output: None,
            resource_id,
            type_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn output(mut self, value: String) -> Self {
        self.output = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringTerraformExportAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringTerraformExportAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringTerraformExportAttributesVisitor {
            type Value = SecurityMonitoringTerraformExportAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut output: Option<String> = None;
                let mut resource_id: Option<String> = None;
                let mut type_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "output" => {
                            if v.is_null() {
                                continue;
                            }
                            output = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type_name" => {
                            type_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let type_name = type_name.ok_or_else(|| M::Error::missing_field("type_name"))?;

                let content = SecurityMonitoringTerraformExportAttributes {
                    output,
                    resource_id,
                    type_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringTerraformExportAttributesVisitor)
    }
}
