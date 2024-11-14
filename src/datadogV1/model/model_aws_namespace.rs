// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AWSNamespace {
    ELB,
    APPLICATION_ELB,
    SQS,
    RDS,
    CUSTOM,
    NETWORK_ELB,
    LAMBDA,
    STEP_FUNCTIONS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AWSNamespace {
    fn to_string(&self) -> String {
        match self {
            Self::ELB => String::from("elb"),
            Self::APPLICATION_ELB => String::from("application_elb"),
            Self::SQS => String::from("sqs"),
            Self::RDS => String::from("rds"),
            Self::CUSTOM => String::from("custom"),
            Self::NETWORK_ELB => String::from("network_elb"),
            Self::LAMBDA => String::from("lambda"),
            Self::STEP_FUNCTIONS => String::from("step_functions"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AWSNamespace {
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

impl<'de> Deserialize<'de> for AWSNamespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "elb" => Self::ELB,
            "application_elb" => Self::APPLICATION_ELB,
            "sqs" => Self::SQS,
            "rds" => Self::RDS,
            "custom" => Self::CUSTOM,
            "network_elb" => Self::NETWORK_ELB,
            "lambda" => Self::LAMBDA,
            "step_functions" => Self::STEP_FUNCTIONS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
