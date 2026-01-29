// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for dataset dependency.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetDependencyAttributesResponse {
    /// The count of resources that depend on the dataset.
    #[serde(rename = "count")]
    pub count: i64,
    /// Array of IDs of resources that depend on the dataset.
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    /// The type of resource that depends on the dataset.
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetDependencyAttributesResponse {
    pub fn new(
        count: i64,
        ids: Vec<String>,
        resource_type: String,
    ) -> SecurityMonitoringDatasetDependencyAttributesResponse {
        SecurityMonitoringDatasetDependencyAttributesResponse {
            count,
            ids,
            resource_type,
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetDependencyAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetDependencyAttributesResponseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetDependencyAttributesResponseVisitor {
            type Value = SecurityMonitoringDatasetDependencyAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut ids: Option<Vec<String>> = None;
                let mut resource_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ids" => {
                            ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
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
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;
                let ids = ids.ok_or_else(|| M::Error::missing_field("ids"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;

                let content = SecurityMonitoringDatasetDependencyAttributesResponse {
                    count,
                    ids,
                    resource_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetDependencyAttributesResponseVisitor)
    }
}
