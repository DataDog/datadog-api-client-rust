// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Timeseries execution metadata for the single request accepted by this API version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationResponseMeta {
    /// Effective start of the timeseries query in milliseconds since the Unix epoch.
    #[serde(rename = "from_date")]
    pub from_date: i64,
    /// Effective timeseries interval in milliseconds.
    #[serde(rename = "interval")]
    pub interval: i64,
    /// Execution status for the request's queries.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationQueryStatus>,
    /// Response metadata type for a timeseries anomaly investigation.
    #[serde(rename = "res_type")]
    pub res_type: crate::datadogV2::model::TimeseriesAnomalyInvestigationMetaType,
    /// Non-fatal warnings produced while executing the investigation.
    #[serde(rename = "results_warnings")]
    pub results_warnings:
        Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationResultsWarning>,
    /// Effective end of the timeseries query in milliseconds since the Unix epoch.
    #[serde(rename = "to_date")]
    pub to_date: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationResponseMeta {
    pub fn new(
        from_date: i64,
        interval: i64,
        queries: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationQueryStatus>,
        res_type: crate::datadogV2::model::TimeseriesAnomalyInvestigationMetaType,
        results_warnings: Vec<
            crate::datadogV2::model::TimeseriesAnomalyInvestigationResultsWarning,
        >,
        to_date: i64,
    ) -> TimeseriesAnomalyInvestigationResponseMeta {
        TimeseriesAnomalyInvestigationResponseMeta {
            from_date,
            interval,
            queries,
            res_type,
            results_warnings,
            to_date,
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationResponseMetaVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationResponseMetaVisitor {
            type Value = TimeseriesAnomalyInvestigationResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_date: Option<i64> = None;
                let mut interval: Option<i64> = None;
                let mut queries: Option<
                    Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationQueryStatus>,
                > = None;
                let mut res_type: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationMetaType,
                > = None;
                let mut results_warnings: Option<
                    Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationResultsWarning>,
                > = None;
                let mut to_date: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_date" => {
                            from_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "res_type" => {
                            res_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _res_type) = res_type {
                                match _res_type {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationMetaType::UnparsedObject(_res_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "results_warnings" => {
                            results_warnings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_date" => {
                            to_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from_date = from_date.ok_or_else(|| M::Error::missing_field("from_date"))?;
                let interval = interval.ok_or_else(|| M::Error::missing_field("interval"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let res_type = res_type.ok_or_else(|| M::Error::missing_field("res_type"))?;
                let results_warnings =
                    results_warnings.ok_or_else(|| M::Error::missing_field("results_warnings"))?;
                let to_date = to_date.ok_or_else(|| M::Error::missing_field("to_date"))?;

                let content = TimeseriesAnomalyInvestigationResponseMeta {
                    from_date,
                    interval,
                    queries,
                    res_type,
                    results_warnings,
                    to_date,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationResponseMetaVisitor)
    }
}
