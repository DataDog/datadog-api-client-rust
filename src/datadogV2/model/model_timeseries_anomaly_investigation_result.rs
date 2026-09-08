// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Completed result for one timeseries request. The anomalies array is empty when no qualifying anomaly is found.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationResult {
    /// Detected anomalies. This API version returns at most one anomaly.
    #[serde(rename = "anomalies")]
    pub anomalies: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomaly>,
    /// Status value indicating successful completion.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::TimeseriesAnomalyInvestigationCompleteStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationResult {
    pub fn new(
        anomalies: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomaly>,
        status: crate::datadogV2::model::TimeseriesAnomalyInvestigationCompleteStatus,
    ) -> TimeseriesAnomalyInvestigationResult {
        TimeseriesAnomalyInvestigationResult {
            anomalies,
            status,
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationResultVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationResultVisitor {
            type Value = TimeseriesAnomalyInvestigationResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anomalies: Option<
                    Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomaly>,
                > = None;
                let mut status: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationCompleteStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anomalies" => {
                            anomalies = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationCompleteStatus::UnparsedObject(_status) => {
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
                let anomalies = anomalies.ok_or_else(|| M::Error::missing_field("anomalies"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = TimeseriesAnomalyInvestigationResult {
                    anomalies,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationResultVisitor)
    }
}
