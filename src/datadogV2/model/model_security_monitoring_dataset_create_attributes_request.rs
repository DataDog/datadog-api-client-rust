// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a security monitoring dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetCreateAttributesRequest {
    /// The definition of a dataset, including its data source, name, indexes, and columns.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
    /// A description of the dataset (maximum 255 characters).
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetCreateAttributesRequest {
    pub fn new(
        definition: crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
        description: String,
    ) -> SecurityMonitoringDatasetCreateAttributesRequest {
        SecurityMonitoringDatasetCreateAttributesRequest {
            definition,
            description,
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetCreateAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetCreateAttributesRequestVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetCreateAttributesRequestVisitor {
            type Value = SecurityMonitoringDatasetCreateAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<
                    crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
                > = None;
                let mut description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;

                let content = SecurityMonitoringDatasetCreateAttributesRequest {
                    definition,
                    description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetCreateAttributesRequestVisitor)
    }
}
