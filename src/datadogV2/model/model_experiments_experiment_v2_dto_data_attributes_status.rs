// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsExperimentV2DTODataAttributesStatus {
    DRAFT,
    SCHEDULED,
    IN_PROGRESS,
    READY_FOR_DECISION,
    DECISION_MADE,
    CANCELLED,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsExperimentV2DTODataAttributesStatus {
    fn to_string(&self) -> String {
        match self {
            Self::DRAFT => String::from("DRAFT"),
            Self::SCHEDULED => String::from("SCHEDULED"),
            Self::IN_PROGRESS => String::from("IN_PROGRESS"),
            Self::READY_FOR_DECISION => String::from("READY_FOR_DECISION"),
            Self::DECISION_MADE => String::from("DECISION_MADE"),
            Self::CANCELLED => String::from("CANCELLED"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsExperimentV2DTODataAttributesStatus {
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

impl<'de> Deserialize<'de> for ExperimentsExperimentV2DTODataAttributesStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DRAFT" => Self::DRAFT,
            "SCHEDULED" => Self::SCHEDULED,
            "IN_PROGRESS" => Self::IN_PROGRESS,
            "READY_FOR_DECISION" => Self::READY_FOR_DECISION,
            "DECISION_MADE" => Self::DECISION_MADE,
            "CANCELLED" => Self::CANCELLED,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
