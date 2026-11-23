// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a timeseries savings response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsSavingsTimeseriesResponseAttributes {
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "actual_cost")]
    pub actual_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "effective_savings_rate")]
    pub effective_savings_rate: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    /// Metadata reporting that the cost data lacks the unused costs of some commitments, as with Azure exports
    /// scoped below the billing profile. Utilization is then overstated, and realized savings and effective
    /// savings rate are upper bounds. Omitted when the unused costs are available.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CommitmentsUnusedCostMeta>,
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "on_demand_equivalent_cost")]
    pub on_demand_equivalent_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    /// The query that produces the response values, in the same format as dashboard and notebook widget
    /// requests, so a client can graph the values in a widget. Each formula's `id` names the response value
    /// that it produces.
    #[serde(rename = "query_definition")]
    pub query_definition: Option<crate::datadogV2::model::CommitmentsQueryDefinition>,
    /// A timeseries metric containing timestamps, series values, and optional unit metadata.
    #[serde(rename = "realized_savings")]
    pub realized_savings: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsSavingsTimeseriesResponseAttributes {
    pub fn new(
        actual_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
        effective_savings_rate: crate::datadogV2::model::CommitmentsTimeseriesMetric,
        on_demand_equivalent_cost: crate::datadogV2::model::CommitmentsTimeseriesMetric,
        realized_savings: crate::datadogV2::model::CommitmentsTimeseriesMetric,
    ) -> CommitmentsSavingsTimeseriesResponseAttributes {
        CommitmentsSavingsTimeseriesResponseAttributes {
            actual_cost,
            effective_savings_rate,
            meta: None,
            on_demand_equivalent_cost,
            query_definition: None,
            realized_savings,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::CommitmentsUnusedCostMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn query_definition(
        mut self,
        value: crate::datadogV2::model::CommitmentsQueryDefinition,
    ) -> Self {
        self.query_definition = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsSavingsTimeseriesResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsSavingsTimeseriesResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CommitmentsSavingsTimeseriesResponseAttributesVisitor {
            type Value = CommitmentsSavingsTimeseriesResponseAttributes;

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
                let mut meta: Option<crate::datadogV2::model::CommitmentsUnusedCostMeta> = None;
                let mut on_demand_equivalent_cost: Option<
                    crate::datadogV2::model::CommitmentsTimeseriesMetric,
                > = None;
                let mut query_definition: Option<
                    crate::datadogV2::model::CommitmentsQueryDefinition,
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
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_demand_equivalent_cost" => {
                            on_demand_equivalent_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_definition" => {
                            if v.is_null() {
                                continue;
                            }
                            query_definition =
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

                let content = CommitmentsSavingsTimeseriesResponseAttributes {
                    actual_cost,
                    effective_savings_rate,
                    meta,
                    on_demand_equivalent_cost,
                    query_definition,
                    realized_savings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsSavingsTimeseriesResponseAttributesVisitor)
    }
}
