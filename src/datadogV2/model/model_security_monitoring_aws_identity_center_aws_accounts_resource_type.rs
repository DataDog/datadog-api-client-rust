// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringAWSIdentityCenterAWSAccountsResourceType {
    AWS_IDENTITY_CENTER_AWS_ACCOUNTS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringAWSIdentityCenterAWSAccountsResourceType {
    fn to_string(&self) -> String {
        match self {
            Self::AWS_IDENTITY_CENTER_AWS_ACCOUNTS => {
                String::from("aws_identity_center_aws_accounts")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringAWSIdentityCenterAWSAccountsResourceType {
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

impl<'de> Deserialize<'de> for SecurityMonitoringAWSIdentityCenterAWSAccountsResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aws_identity_center_aws_accounts" => Self::AWS_IDENTITY_CENTER_AWS_ACCOUNTS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
