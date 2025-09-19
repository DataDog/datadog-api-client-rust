// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CI pipeline related statistics for the flaky test. This information is only available if test runs are associated with CI pipeline events from CI Visibility.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestPipelineStats {
    /// The number of pipelines that failed due to this test for the past 7 days. This is computed as the sum of failed CI pipeline events associated with test runs where the flaky test failed.
    #[serde(
        rename = "failed_pipelines",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub failed_pipelines: Option<Option<i64>>,
    /// The total time lost by CI pipelines due to this flaky test in milliseconds. This is computed as the sum of the duration of failed CI pipeline events associated with test runs where the flaky test failed.
    #[serde(
        rename = "total_lost_time_ms",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_lost_time_ms: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestPipelineStats {
    pub fn new() -> FlakyTestPipelineStats {
        FlakyTestPipelineStats {
            failed_pipelines: None,
            total_lost_time_ms: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn failed_pipelines(mut self, value: Option<i64>) -> Self {
        self.failed_pipelines = Some(value);
        self
    }

    pub fn total_lost_time_ms(mut self, value: Option<i64>) -> Self {
        self.total_lost_time_ms = Some(value);
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

impl Default for FlakyTestPipelineStats {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestPipelineStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestPipelineStatsVisitor;
        impl<'a> Visitor<'a> for FlakyTestPipelineStatsVisitor {
            type Value = FlakyTestPipelineStats;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failed_pipelines: Option<Option<i64>> = None;
                let mut total_lost_time_ms: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failed_pipelines" => {
                            failed_pipelines =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_lost_time_ms" => {
                            total_lost_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FlakyTestPipelineStats {
                    failed_pipelines,
                    total_lost_time_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestPipelineStatsVisitor)
    }
}
