// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Statistical distributions of long task metrics computed per view across sampled views.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LongTaskStatsPerView {
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "fcp_blocking_time_ms")]
    pub fcp_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "fcp_count")]
    pub fcp_count: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "inp_overlap_blocking_time_ms")]
    pub inp_overlap_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "inp_overlap_count")]
    pub inp_overlap_count: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "lcp_blocking_time_ms")]
    pub lcp_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "lcp_count")]
    pub lcp_count: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "loading_time_blocking_time_ms")]
    pub loading_time_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "loading_time_count")]
    pub loading_time_count: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "total_blocking_time_ms")]
    pub total_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats>,
    /// Statistical distribution (average, min, max) of a long task metric across sampled views.
    #[serde(rename = "total_count")]
    pub total_count: Option<crate::datadogV2::model::LongTaskMetricStats>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LongTaskStatsPerView {
    pub fn new() -> LongTaskStatsPerView {
        LongTaskStatsPerView {
            fcp_blocking_time_ms: None,
            fcp_count: None,
            inp_overlap_blocking_time_ms: None,
            inp_overlap_count: None,
            lcp_blocking_time_ms: None,
            lcp_count: None,
            loading_time_blocking_time_ms: None,
            loading_time_count: None,
            total_blocking_time_ms: None,
            total_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fcp_blocking_time_ms(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.fcp_blocking_time_ms = Some(value);
        self
    }

    pub fn fcp_count(mut self, value: crate::datadogV2::model::LongTaskMetricStats) -> Self {
        self.fcp_count = Some(value);
        self
    }

    pub fn inp_overlap_blocking_time_ms(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.inp_overlap_blocking_time_ms = Some(value);
        self
    }

    pub fn inp_overlap_count(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.inp_overlap_count = Some(value);
        self
    }

    pub fn lcp_blocking_time_ms(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.lcp_blocking_time_ms = Some(value);
        self
    }

    pub fn lcp_count(mut self, value: crate::datadogV2::model::LongTaskMetricStats) -> Self {
        self.lcp_count = Some(value);
        self
    }

    pub fn loading_time_blocking_time_ms(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.loading_time_blocking_time_ms = Some(value);
        self
    }

    pub fn loading_time_count(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.loading_time_count = Some(value);
        self
    }

    pub fn total_blocking_time_ms(
        mut self,
        value: crate::datadogV2::model::LongTaskMetricStats,
    ) -> Self {
        self.total_blocking_time_ms = Some(value);
        self
    }

    pub fn total_count(mut self, value: crate::datadogV2::model::LongTaskMetricStats) -> Self {
        self.total_count = Some(value);
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

impl Default for LongTaskStatsPerView {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LongTaskStatsPerView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LongTaskStatsPerViewVisitor;
        impl<'a> Visitor<'a> for LongTaskStatsPerViewVisitor {
            type Value = LongTaskStatsPerView;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fcp_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats> =
                    None;
                let mut fcp_count: Option<crate::datadogV2::model::LongTaskMetricStats> = None;
                let mut inp_overlap_blocking_time_ms: Option<
                    crate::datadogV2::model::LongTaskMetricStats,
                > = None;
                let mut inp_overlap_count: Option<crate::datadogV2::model::LongTaskMetricStats> =
                    None;
                let mut lcp_blocking_time_ms: Option<crate::datadogV2::model::LongTaskMetricStats> =
                    None;
                let mut lcp_count: Option<crate::datadogV2::model::LongTaskMetricStats> = None;
                let mut loading_time_blocking_time_ms: Option<
                    crate::datadogV2::model::LongTaskMetricStats,
                > = None;
                let mut loading_time_count: Option<crate::datadogV2::model::LongTaskMetricStats> =
                    None;
                let mut total_blocking_time_ms: Option<
                    crate::datadogV2::model::LongTaskMetricStats,
                > = None;
                let mut total_count: Option<crate::datadogV2::model::LongTaskMetricStats> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fcp_blocking_time_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            fcp_blocking_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fcp_count" => {
                            if v.is_null() {
                                continue;
                            }
                            fcp_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inp_overlap_blocking_time_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            inp_overlap_blocking_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inp_overlap_count" => {
                            if v.is_null() {
                                continue;
                            }
                            inp_overlap_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lcp_blocking_time_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            lcp_blocking_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lcp_count" => {
                            if v.is_null() {
                                continue;
                            }
                            lcp_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "loading_time_blocking_time_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            loading_time_blocking_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "loading_time_count" => {
                            if v.is_null() {
                                continue;
                            }
                            loading_time_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_blocking_time_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            total_blocking_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LongTaskStatsPerView {
                    fcp_blocking_time_ms,
                    fcp_count,
                    inp_overlap_blocking_time_ms,
                    inp_overlap_count,
                    lcp_blocking_time_ms,
                    lcp_count,
                    loading_time_blocking_time_ms,
                    loading_time_count,
                    total_blocking_time_ms,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LongTaskStatsPerViewVisitor)
    }
}
