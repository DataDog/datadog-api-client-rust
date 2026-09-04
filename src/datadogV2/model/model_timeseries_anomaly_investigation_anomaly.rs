// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Most significant anomaly detected in the request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationAnomaly {
    /// Anomaly detection configuration used for the result.
    #[serde(rename = "anomaly_detection")]
    pub anomaly_detection: crate::datadogV2::model::TimeseriesAnomalyInvestigationDetection,
    /// Half-open time interval in milliseconds since the Unix epoch.
    #[serde(rename = "detected_interval")]
    pub detected_interval: crate::datadogV2::model::TimeseriesAnomalyInvestigationInterval,
    /// Half-open time interval in milliseconds since the Unix epoch.
    #[serde(rename = "display_interval")]
    pub display_interval: crate::datadogV2::model::TimeseriesAnomalyInvestigationInterval,
    /// Deterministic explanations for the anomaly, ordered by importance.
    #[serde(rename = "findings")]
    pub findings: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationFinding>,
    /// Most anomalous point within the detected interval.
    #[serde(rename = "maximum_deviation")]
    pub maximum_deviation: crate::datadogV2::model::TimeseriesAnomalyInvestigationMaximumDeviation,
    /// Logical series on which the anomaly was detected.
    #[serde(rename = "series")]
    pub series: crate::datadogV2::model::TimeseriesAnomalyInvestigationSeries,
    /// Summary of optional influential-tag enrichment. Count and key fields are present only when analysis completes; enrichment availability does not affect completion of the investigation result.
    #[serde(rename = "tag_analysis")]
    pub tag_analysis: crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysis,
    /// Direction of an anomaly relative to its expected range.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomalyType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationAnomaly {
    pub fn new(
        anomaly_detection: crate::datadogV2::model::TimeseriesAnomalyInvestigationDetection,
        detected_interval: crate::datadogV2::model::TimeseriesAnomalyInvestigationInterval,
        display_interval: crate::datadogV2::model::TimeseriesAnomalyInvestigationInterval,
        findings: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationFinding>,
        maximum_deviation: crate::datadogV2::model::TimeseriesAnomalyInvestigationMaximumDeviation,
        series: crate::datadogV2::model::TimeseriesAnomalyInvestigationSeries,
        tag_analysis: crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysis,
        type_: crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomalyType,
    ) -> TimeseriesAnomalyInvestigationAnomaly {
        TimeseriesAnomalyInvestigationAnomaly {
            anomaly_detection,
            detected_interval,
            display_interval,
            findings,
            maximum_deviation,
            series,
            tag_analysis,
            type_,
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationAnomaly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationAnomalyVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationAnomalyVisitor {
            type Value = TimeseriesAnomalyInvestigationAnomaly;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anomaly_detection: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationDetection,
                > = None;
                let mut detected_interval: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationInterval,
                > = None;
                let mut display_interval: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationInterval,
                > = None;
                let mut findings: Option<
                    Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationFinding>,
                > = None;
                let mut maximum_deviation: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationMaximumDeviation,
                > = None;
                let mut series: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationSeries,
                > = None;
                let mut tag_analysis: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysis,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomalyType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anomaly_detection" => {
                            anomaly_detection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detected_interval" => {
                            detected_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_interval" => {
                            display_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "findings" => {
                            findings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maximum_deviation" => {
                            maximum_deviation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "series" => {
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_analysis" => {
                            tag_analysis =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomalyType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let anomaly_detection = anomaly_detection
                    .ok_or_else(|| M::Error::missing_field("anomaly_detection"))?;
                let detected_interval = detected_interval
                    .ok_or_else(|| M::Error::missing_field("detected_interval"))?;
                let display_interval =
                    display_interval.ok_or_else(|| M::Error::missing_field("display_interval"))?;
                let findings = findings.ok_or_else(|| M::Error::missing_field("findings"))?;
                let maximum_deviation = maximum_deviation
                    .ok_or_else(|| M::Error::missing_field("maximum_deviation"))?;
                let series = series.ok_or_else(|| M::Error::missing_field("series"))?;
                let tag_analysis =
                    tag_analysis.ok_or_else(|| M::Error::missing_field("tag_analysis"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TimeseriesAnomalyInvestigationAnomaly {
                    anomaly_detection,
                    detected_interval,
                    display_interval,
                    findings,
                    maximum_deviation,
                    series,
                    tag_analysis,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationAnomalyVisitor)
    }
}
