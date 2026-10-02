// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A comparison that selects metric data by a property or measure.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricPropertyFilter {
    /// ID of the measure evaluated by the filter.
    #[serde(rename = "measure_id")]
    pub measure_id: Option<String>,
    /// Comparison applied by the filter.
    #[serde(rename = "operation")]
    pub operation: Option<String>,
    /// ID of the property evaluated by the filter.
    #[serde(rename = "property_id")]
    pub property_id: Option<String>,
    /// Values used by the filter's comparison.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricPropertyFilter {
    pub fn new() -> ExperimentsMetricPropertyFilter {
        ExperimentsMetricPropertyFilter {
            measure_id: None,
            operation: None,
            property_id: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn measure_id(mut self, value: String) -> Self {
        self.measure_id = Some(value);
        self
    }

    pub fn operation(mut self, value: String) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn property_id(mut self, value: String) -> Self {
        self.property_id = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
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

impl Default for ExperimentsMetricPropertyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricPropertyFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricPropertyFilterVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricPropertyFilterVisitor {
            type Value = ExperimentsMetricPropertyFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut measure_id: Option<String> = None;
                let mut operation: Option<String> = None;
                let mut property_id: Option<String> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "measure_id" => {
                            if v.is_null() {
                                continue;
                            }
                            measure_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation" => {
                            if v.is_null() {
                                continue;
                            }
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property_id" => {
                            if v.is_null() {
                                continue;
                            }
                            property_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricPropertyFilter {
                    measure_id,
                    operation,
                    property_id,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsMetricPropertyFilterVisitor)
    }
}
