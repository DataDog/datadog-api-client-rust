// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType {
    EXPERIMENT_HAS_ASSIGNMENTS,
    METRIC_HAS_DATA,
    ASSIGNMENT_IMBALANCE,
    METRIC_WINSORIZE_ZERO,
    PRE_EXPERIMENT_IMBALANCE,
    MIXED_ASSIGNMENTS,
    DIMENSIONAL_ASSIGNMENT_IMBALANCE,
    FLAG_HAS_EVALUATIONS,
    IMPLAUSIBLE_PRIOR,
    DIMENSIONAL_DEGRADATION,
    PIPELINE_STATUS,
    MAPPED_ANALYSIS_CONFIGURATION,
    MAPPED_ANALYSIS_COVERAGE,
    MAPPED_ANALYSIS_COLLISIONS,
    MAPPED_ANALYSIS_FANOUT,
    MAPPED_ANALYSIS_CHANGE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType {
    fn to_string(&self) -> String {
        match self {
            Self::EXPERIMENT_HAS_ASSIGNMENTS => String::from("EXPERIMENT_HAS_ASSIGNMENTS"),
            Self::METRIC_HAS_DATA => String::from("METRIC_HAS_DATA"),
            Self::ASSIGNMENT_IMBALANCE => String::from("ASSIGNMENT_IMBALANCE"),
            Self::METRIC_WINSORIZE_ZERO => String::from("METRIC_WINSORIZE_ZERO"),
            Self::PRE_EXPERIMENT_IMBALANCE => String::from("PRE_EXPERIMENT_IMBALANCE"),
            Self::MIXED_ASSIGNMENTS => String::from("MIXED_ASSIGNMENTS"),
            Self::DIMENSIONAL_ASSIGNMENT_IMBALANCE => {
                String::from("DIMENSIONAL_ASSIGNMENT_IMBALANCE")
            }
            Self::FLAG_HAS_EVALUATIONS => String::from("FLAG_HAS_EVALUATIONS"),
            Self::IMPLAUSIBLE_PRIOR => String::from("IMPLAUSIBLE_PRIOR"),
            Self::DIMENSIONAL_DEGRADATION => String::from("DIMENSIONAL_DEGRADATION"),
            Self::PIPELINE_STATUS => String::from("PIPELINE_STATUS"),
            Self::MAPPED_ANALYSIS_CONFIGURATION => String::from("MAPPED_ANALYSIS_CONFIGURATION"),
            Self::MAPPED_ANALYSIS_COVERAGE => String::from("MAPPED_ANALYSIS_COVERAGE"),
            Self::MAPPED_ANALYSIS_COLLISIONS => String::from("MAPPED_ANALYSIS_COLLISIONS"),
            Self::MAPPED_ANALYSIS_FANOUT => String::from("MAPPED_ANALYSIS_FANOUT"),
            Self::MAPPED_ANALYSIS_CHANGE => String::from("MAPPED_ANALYSIS_CHANGE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "EXPERIMENT_HAS_ASSIGNMENTS" => Self::EXPERIMENT_HAS_ASSIGNMENTS,
            "METRIC_HAS_DATA" => Self::METRIC_HAS_DATA,
            "ASSIGNMENT_IMBALANCE" => Self::ASSIGNMENT_IMBALANCE,
            "METRIC_WINSORIZE_ZERO" => Self::METRIC_WINSORIZE_ZERO,
            "PRE_EXPERIMENT_IMBALANCE" => Self::PRE_EXPERIMENT_IMBALANCE,
            "MIXED_ASSIGNMENTS" => Self::MIXED_ASSIGNMENTS,
            "DIMENSIONAL_ASSIGNMENT_IMBALANCE" => Self::DIMENSIONAL_ASSIGNMENT_IMBALANCE,
            "FLAG_HAS_EVALUATIONS" => Self::FLAG_HAS_EVALUATIONS,
            "IMPLAUSIBLE_PRIOR" => Self::IMPLAUSIBLE_PRIOR,
            "DIMENSIONAL_DEGRADATION" => Self::DIMENSIONAL_DEGRADATION,
            "PIPELINE_STATUS" => Self::PIPELINE_STATUS,
            "MAPPED_ANALYSIS_CONFIGURATION" => Self::MAPPED_ANALYSIS_CONFIGURATION,
            "MAPPED_ANALYSIS_COVERAGE" => Self::MAPPED_ANALYSIS_COVERAGE,
            "MAPPED_ANALYSIS_COLLISIONS" => Self::MAPPED_ANALYSIS_COLLISIONS,
            "MAPPED_ANALYSIS_FANOUT" => Self::MAPPED_ANALYSIS_FANOUT,
            "MAPPED_ANALYSIS_CHANGE" => Self::MAPPED_ANALYSIS_CHANGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
