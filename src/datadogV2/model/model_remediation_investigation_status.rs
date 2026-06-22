// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemediationInvestigationStatus {
    OPEN,
    APPROVAL_REQUIRED,
    EXECUTING,
    SUCCEEDED,
    FAILED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RemediationInvestigationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::OPEN => String::from("open"),
            Self::APPROVAL_REQUIRED => String::from("approval_required"),
            Self::EXECUTING => String::from("executing"),
            Self::SUCCEEDED => String::from("succeeded"),
            Self::FAILED => String::from("failed"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RemediationInvestigationStatus {
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

impl<'de> Deserialize<'de> for RemediationInvestigationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "open" => Self::OPEN,
            "approval_required" => Self::APPROVAL_REQUIRED,
            "executing" => Self::EXECUTING,
            "succeeded" => Self::SUCCEEDED,
            "failed" => Self::FAILED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
