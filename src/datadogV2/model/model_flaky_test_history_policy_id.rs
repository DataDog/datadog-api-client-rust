// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlakyTestHistoryPolicyId {
    MANUAL,
    FIXED,
    DISABLE_FAILURE_RATE,
    DISABLE_BRANCH_FLAKE,
    DISABLE_DAYS_ACTIVE,
    QUARANTINE_FAILURE_RATE,
    QUARANTINE_BRANCH_FLAKE,
    QUARANTINE_DAYS_ACTIVE,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FlakyTestHistoryPolicyId {
    fn to_string(&self) -> String {
        match self {
            Self::MANUAL => String::from("ftm_policy.manual"),
            Self::FIXED => String::from("ftm_policy.fixed"),
            Self::DISABLE_FAILURE_RATE => String::from("ftm_policy.disable.failure_rate"),
            Self::DISABLE_BRANCH_FLAKE => String::from("ftm_policy.disable.branch_flake"),
            Self::DISABLE_DAYS_ACTIVE => String::from("ftm_policy.disable.days_active"),
            Self::QUARANTINE_FAILURE_RATE => String::from("ftm_policy.quarantine.failure_rate"),
            Self::QUARANTINE_BRANCH_FLAKE => String::from("ftm_policy.quarantine.branch_flake"),
            Self::QUARANTINE_DAYS_ACTIVE => String::from("ftm_policy.quarantine.days_active"),
            Self::UNKNOWN => String::from("unknown"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FlakyTestHistoryPolicyId {
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

impl<'de> Deserialize<'de> for FlakyTestHistoryPolicyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ftm_policy.manual" => Self::MANUAL,
            "ftm_policy.fixed" => Self::FIXED,
            "ftm_policy.disable.failure_rate" => Self::DISABLE_FAILURE_RATE,
            "ftm_policy.disable.branch_flake" => Self::DISABLE_BRANCH_FLAKE,
            "ftm_policy.disable.days_active" => Self::DISABLE_DAYS_ACTIVE,
            "ftm_policy.quarantine.failure_rate" => Self::QUARANTINE_FAILURE_RATE,
            "ftm_policy.quarantine.branch_flake" => Self::QUARANTINE_BRANCH_FLAKE,
            "ftm_policy.quarantine.days_active" => Self::QUARANTINE_DAYS_ACTIVE,
            "unknown" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
