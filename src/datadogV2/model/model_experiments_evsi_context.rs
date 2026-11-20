// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Forecast horizon and traffic counts used to calculate expected value of sample information (EVSI) for result_run_id. Assumes traffic continues at the rate observed during the lookback window. Omitted if the forecast was not recorded or could not be loaded.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsEVSIContext {
    /// Exact assignment-window duration in days, including fractional days.
	#[serialize_always]
    #[serde(rename = "elapsed_days")]
    pub elapsed_days: Option<f64>,
    /// Forecast horizon in days from the original assignment window.
    #[serde(rename = "horizon_days")]
    pub horizon_days: f64,
    /// Projected additional exposures over the horizon, assuming constant traffic.
	#[serialize_always]
    #[serde(rename = "horizon_exposures")]
    pub horizon_exposures: Option<f64>,
    /// Selected lookback length in complete days.
    #[serde(rename = "lookback_days")]
    pub lookback_days: i64,
    /// Exposures across all variants in the eligible lookback buckets.
	#[serialize_always]
    #[serde(rename = "lookback_exposures")]
    pub lookback_exposures: Option<i64>,
    /// Experiment-wide exposures across the assignment window, including boundary buckets.
	#[serialize_always]
    #[serde(rename = "total_exposures")]
    pub total_exposures: Option<i64>,
    /// Recorded traffic forecast unavailability reason. Per-analysis reasons are reported separately.
	#[serialize_always]
    #[serde(rename = "unavailable_reason")]
    pub unavailable_reason: Option<crate::datadogV2::model::ExperimentsEVSITrafficUnavailableReason>,
    /// Interior UTC daily buckets excluding both boundary days. Includes zero-traffic days and does not certify ingestion completeness.
	#[serialize_always]
    #[serde(rename = "usable_traffic_days")]
    pub usable_traffic_days: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsEVSIContext {
    pub fn new(elapsed_days: Option<f64>, horizon_days: f64, horizon_exposures: Option<f64>, lookback_days: i64, lookback_exposures: Option<i64>, total_exposures: Option<i64>, unavailable_reason: Option<crate::datadogV2::model::ExperimentsEVSITrafficUnavailableReason>, usable_traffic_days: Option<i64>) -> ExperimentsEVSIContext {
        ExperimentsEVSIContext {
            elapsed_days,
            horizon_days,
            horizon_exposures,
            lookback_days,
            lookback_exposures,
            total_exposures,
            unavailable_reason,
            usable_traffic_days,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for ExperimentsEVSIContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsEVSIContextVisitor;
        impl<'a> Visitor<'a> for ExperimentsEVSIContextVisitor {
            type Value = ExperimentsEVSIContext;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed_days: Option<Option<f64>> = None;
                let mut horizon_days: Option<f64> = None;
                let mut horizon_exposures: Option<Option<f64>> = None;
                let mut lookback_days: Option<i64> = None;
                let mut lookback_exposures: Option<Option<i64>> = None;
                let mut total_exposures: Option<Option<i64>> = None;
                let mut unavailable_reason: Option<Option<crate::datadogV2::model::ExperimentsEVSITrafficUnavailableReason>> = None;
                let mut usable_traffic_days: Option<Option<i64>> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elapsed_days" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            elapsed_days = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "horizon_days" => {
                            horizon_days = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "horizon_exposures" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            horizon_exposures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "lookback_days" => {
                            lookback_days = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "lookback_exposures" => {
                            lookback_exposures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "total_exposures" => {
                            total_exposures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "unavailable_reason" => {
                            unavailable_reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unavailable_reason) = unavailable_reason {
                                match _unavailable_reason {
                                    Some(crate::datadogV2::model::ExperimentsEVSITrafficUnavailableReason::UnparsedObject(_unavailable_reason)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "usable_traffic_days" => {
                            usable_traffic_days = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let elapsed_days = elapsed_days.ok_or_else(|| M::Error::missing_field("elapsed_days"))?;
                let horizon_days = horizon_days.ok_or_else(|| M::Error::missing_field("horizon_days"))?;
                let horizon_exposures = horizon_exposures.ok_or_else(|| M::Error::missing_field("horizon_exposures"))?;
                let lookback_days = lookback_days.ok_or_else(|| M::Error::missing_field("lookback_days"))?;
                let lookback_exposures = lookback_exposures.ok_or_else(|| M::Error::missing_field("lookback_exposures"))?;
                let total_exposures = total_exposures.ok_or_else(|| M::Error::missing_field("total_exposures"))?;
                let unavailable_reason = unavailable_reason.ok_or_else(|| M::Error::missing_field("unavailable_reason"))?;
                let usable_traffic_days = usable_traffic_days.ok_or_else(|| M::Error::missing_field("usable_traffic_days"))?;

                let content = ExperimentsEVSIContext {
                    elapsed_days,
                    horizon_days,
                    horizon_exposures,
                    lookback_days,
                    lookback_exposures,
                    total_exposures,
                    unavailable_reason,
                    usable_traffic_days,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsEVSIContextVisitor)
    }
}
