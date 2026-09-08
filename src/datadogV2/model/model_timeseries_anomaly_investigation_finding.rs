// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Deterministic explanation for a detected anomaly.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TimeseriesAnomalyInvestigationFinding {
    TimeseriesAnomalyInvestigationInfluentialTagFinding(
        Box<crate::datadogV2::model::TimeseriesAnomalyInvestigationInfluentialTagFinding>,
    ),
    TimeseriesAnomalyInvestigationAnomalyFinding(
        Box<crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomalyFinding>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationFinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TimeseriesAnomalyInvestigationInfluentialTagFinding>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TimeseriesAnomalyInvestigationFinding::TimeseriesAnomalyInvestigationInfluentialTagFinding(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TimeseriesAnomalyInvestigationAnomalyFinding>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TimeseriesAnomalyInvestigationFinding::TimeseriesAnomalyInvestigationAnomalyFinding(_v));
            }
        }

        return Ok(TimeseriesAnomalyInvestigationFinding::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
