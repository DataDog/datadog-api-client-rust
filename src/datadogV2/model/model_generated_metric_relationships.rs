// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for a generated metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeneratedMetricRelationships {
    /// Relationship to the attributes of a generated metric.
    #[serde(rename = "generated_metric_attributes")]
    pub generated_metric_attributes:
        Option<crate::datadogV2::model::GeneratedMetricAttributesRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeneratedMetricRelationships {
    pub fn new() -> GeneratedMetricRelationships {
        GeneratedMetricRelationships {
            generated_metric_attributes: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn generated_metric_attributes(
        mut self,
        value: crate::datadogV2::model::GeneratedMetricAttributesRelationship,
    ) -> Self {
        self.generated_metric_attributes = Some(value);
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

impl Default for GeneratedMetricRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GeneratedMetricRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeneratedMetricRelationshipsVisitor;
        impl<'a> Visitor<'a> for GeneratedMetricRelationshipsVisitor {
            type Value = GeneratedMetricRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut generated_metric_attributes: Option<
                    crate::datadogV2::model::GeneratedMetricAttributesRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "generated_metric_attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            generated_metric_attributes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GeneratedMetricRelationships {
                    generated_metric_attributes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeneratedMetricRelationshipsVisitor)
    }
}
