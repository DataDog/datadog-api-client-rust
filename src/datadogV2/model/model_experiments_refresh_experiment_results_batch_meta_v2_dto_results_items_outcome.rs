// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome {
    TRIGGERED,
    SKIPPED_ALREADY_RUNNING,
    SKIPPED_NOT_EDITABLE,
    SKIPPED_ORG_AT_CAPACITY,
    FAILED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome {
    fn to_string(&self) -> String {
        match self {
            Self::TRIGGERED => String::from("TRIGGERED"),
            Self::SKIPPED_ALREADY_RUNNING => String::from("SKIPPED_ALREADY_RUNNING"),
            Self::SKIPPED_NOT_EDITABLE => String::from("SKIPPED_NOT_EDITABLE"),
            Self::SKIPPED_ORG_AT_CAPACITY => String::from("SKIPPED_ORG_AT_CAPACITY"),
            Self::FAILED => String::from("FAILED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome {
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
    for ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "TRIGGERED" => Self::TRIGGERED,
            "SKIPPED_ALREADY_RUNNING" => Self::SKIPPED_ALREADY_RUNNING,
            "SKIPPED_NOT_EDITABLE" => Self::SKIPPED_NOT_EDITABLE,
            "SKIPPED_ORG_AT_CAPACITY" => Self::SKIPPED_ORG_AT_CAPACITY,
            "FAILED" => Self::FAILED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
