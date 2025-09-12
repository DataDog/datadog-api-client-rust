// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Test statistics for the flaky test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestStats {
    /// The failure rate percentage of the test for the past 7 days. This is the number of failed test runs divided by the total number of test runs (excluding skipped test runs).
    #[serde(
        rename = "failure_rate_pct",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub failure_rate_pct: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestStats {
    pub fn new() -> FlakyTestStats {
        FlakyTestStats {
            failure_rate_pct: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn failure_rate_pct(mut self, value: Option<f64>) -> Self {
        self.failure_rate_pct = Some(value);
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

impl Default for FlakyTestStats {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestStatsVisitor;
        impl<'a> Visitor<'a> for FlakyTestStatsVisitor {
            type Value = FlakyTestStats;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failure_rate_pct: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failure_rate_pct" => {
                            failure_rate_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FlakyTestStats {
                    failure_rate_pct,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestStatsVisitor)
    }
}
