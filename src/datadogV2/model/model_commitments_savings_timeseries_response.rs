// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing timeseries savings metrics for cloud commitment programs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsSavingsTimeseriesResponse {
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "actual_cost")]
    pub actual_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "effective_savings_rate")]
    pub effective_savings_rate: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "on_demand_equivalent_cost")]
    pub on_demand_equivalent_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "realized_savings")]
    pub realized_savings: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsSavingsTimeseriesResponse {
    pub fn new(
        actual_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
        effective_savings_rate: crate::datadogV2::model::CommitmentsTimeseriesMetric,
        on_demand_equivalent_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
        realized_savings: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    ) -> CommitmentsSavingsTimeseriesResponse {
        CommitmentsSavingsTimeseriesResponse {
            actual_cost,
            effective_savings_rate,
            on_demand_equivalent_cost,
            realized_savings,
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

impl<'de> Deserialize<'de> for CommitmentsSavingsTimeseriesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsSavingsTimeseriesResponseVisitor;
        impl<'a> Visitor<'a> for CommitmentsSavingsTimeseriesResponseVisitor {
            type Value = CommitmentsSavingsTimeseriesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actual_cost: Option<crate::datadogV2::model::CommitmentsTimeseriesMetric> =
                    None;
                let mut effective_savings_rate: Option<
                    crate::datadogV2::model::CommitmentsTimeseriesMetric,
                > = None;
                let mut on_demand_equivalent_cost: Option<
                    crate::datadogV2::model::CommitmentsTimeseriesMetric,
                > = None;
                let mut realized_savings: Option<
                    crate::datadogV2::model::CommitmentsTimeseriesMetric,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actual_cost" => {
                            actual_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "effective_savings_rate" => {
                            effective_savings_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_demand_equivalent_cost" => {
                            on_demand_equivalent_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "realized_savings" => {
                            realized_savings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let actual_cost =
                    actual_cost.ok_or_else(|| M::Error::missing_field("actual_cost"))?;
                let effective_savings_rate = effective_savings_rate
                    .ok_or_else(|| M::Error::missing_field("effective_savings_rate"))?;
                let on_demand_equivalent_cost = on_demand_equivalent_cost
                    .ok_or_else(|| M::Error::missing_field("on_demand_equivalent_cost"))?;
                let realized_savings =
                    realized_savings.ok_or_else(|| M::Error::missing_field("realized_savings"))?;

                let content = CommitmentsSavingsTimeseriesResponse {
                    actual_cost,
                    effective_savings_rate,
                    on_demand_equivalent_cost,
                    realized_savings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsSavingsTimeseriesResponseVisitor)
    }
}
