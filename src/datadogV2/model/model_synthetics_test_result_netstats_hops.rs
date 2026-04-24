// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Statistics about the number of hops for a network test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultNetstatsHops {
    /// Average number of hops.
    #[serde(rename = "avg")]
    pub avg: Option<f64>,
    /// Maximum number of hops.
    #[serde(rename = "max")]
    pub max: Option<i64>,
    /// Minimum number of hops.
    #[serde(rename = "min")]
    pub min: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultNetstatsHops {
    pub fn new() -> SyntheticsTestResultNetstatsHops {
        SyntheticsTestResultNetstatsHops {
            avg: None,
            max: None,
            min: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn avg(mut self, value: f64) -> Self {
        self.avg = Some(value);
        self
    }

    pub fn max(mut self, value: i64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: i64) -> Self {
        self.min = Some(value);
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

impl Default for SyntheticsTestResultNetstatsHops {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultNetstatsHops {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultNetstatsHopsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultNetstatsHopsVisitor {
            type Value = SyntheticsTestResultNetstatsHops;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg: Option<f64> = None;
                let mut max: Option<i64> = None;
                let mut min: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            avg = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max" => {
                            if v.is_null() {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            if v.is_null() {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultNetstatsHops {
                    avg,
                    max,
                    min,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultNetstatsHopsVisitor)
    }
}
