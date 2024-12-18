// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CSM Coverage Analysis.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmCoverageAnalysis {
    /// The number of fully configured resources.
    #[serde(rename = "configured_resources_count")]
    pub configured_resources_count: Option<i64>,
    /// The coverage percentage.
    #[serde(rename = "coverage")]
    pub coverage: Option<f64>,
    /// The number of partially configured resources.
    #[serde(rename = "partially_configured_resources_count")]
    pub partially_configured_resources_count: Option<i64>,
    /// The total number of resources.
    #[serde(rename = "total_resources_count")]
    pub total_resources_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmCoverageAnalysis {
    pub fn new() -> CsmCoverageAnalysis {
        CsmCoverageAnalysis {
            configured_resources_count: None,
            coverage: None,
            partially_configured_resources_count: None,
            total_resources_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn configured_resources_count(mut self, value: i64) -> Self {
        self.configured_resources_count = Some(value);
        self
    }

    pub fn coverage(mut self, value: f64) -> Self {
        self.coverage = Some(value);
        self
    }

    pub fn partially_configured_resources_count(mut self, value: i64) -> Self {
        self.partially_configured_resources_count = Some(value);
        self
    }

    pub fn total_resources_count(mut self, value: i64) -> Self {
        self.total_resources_count = Some(value);
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

impl Default for CsmCoverageAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CsmCoverageAnalysis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmCoverageAnalysisVisitor;
        impl<'a> Visitor<'a> for CsmCoverageAnalysisVisitor {
            type Value = CsmCoverageAnalysis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configured_resources_count: Option<i64> = None;
                let mut coverage: Option<f64> = None;
                let mut partially_configured_resources_count: Option<i64> = None;
                let mut total_resources_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configured_resources_count" => {
                            if v.is_null() {
                                continue;
                            }
                            configured_resources_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            coverage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "partially_configured_resources_count" => {
                            if v.is_null() {
                                continue;
                            }
                            partially_configured_resources_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_resources_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_resources_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CsmCoverageAnalysis {
                    configured_resources_count,
                    coverage,
                    partially_configured_resources_count,
                    total_resources_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmCoverageAnalysisVisitor)
    }
}
