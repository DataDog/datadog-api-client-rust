// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single field change between two versions of a dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetVersionFieldChange {
    /// The current value of the field, serialized as a JSON value.
    #[serde(rename = "current")]
    pub current: serde_json::Value,
    /// The name of the field that changed.
    #[serde(rename = "field")]
    pub field: String,
    /// The previous value of the field, serialized as a JSON value.
    #[serde(rename = "previous")]
    pub previous: serde_json::Value,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetVersionFieldChange {
    pub fn new(
        current: serde_json::Value,
        field: String,
        previous: serde_json::Value,
    ) -> SecurityMonitoringDatasetVersionFieldChange {
        SecurityMonitoringDatasetVersionFieldChange {
            current,
            field,
            previous,
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetVersionFieldChange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetVersionFieldChangeVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetVersionFieldChangeVisitor {
            type Value = SecurityMonitoringDatasetVersionFieldChange;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current: Option<serde_json::Value> = None;
                let mut field: Option<String> = None;
                let mut previous: Option<serde_json::Value> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current" => {
                            current = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field" => {
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previous" => {
                            previous = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let current = current.ok_or_else(|| M::Error::missing_field("current"))?;
                let field = field.ok_or_else(|| M::Error::missing_field("field"))?;
                let previous = previous.ok_or_else(|| M::Error::missing_field("previous"))?;

                let content = SecurityMonitoringDatasetVersionFieldChange {
                    current,
                    field,
                    previous,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetVersionFieldChangeVisitor)
    }
}
