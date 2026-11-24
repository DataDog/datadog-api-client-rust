// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentHealthIssueResourceKind {
    HOST,
    DEPLOYMENT,
    CLUSTER,
    CLUSTER_AGENT,
    OTEL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AgentHealthIssueResourceKind {
    fn to_string(&self) -> String {
        match self {
            Self::HOST => String::from("host"),
            Self::DEPLOYMENT => String::from("deployment"),
            Self::CLUSTER => String::from("cluster"),
            Self::CLUSTER_AGENT => String::from("cluster_agent"),
            Self::OTEL => String::from("otel"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AgentHealthIssueResourceKind {
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

impl<'de> Deserialize<'de> for AgentHealthIssueResourceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "host" => Self::HOST,
            "deployment" => Self::DEPLOYMENT,
            "cluster" => Self::CLUSTER,
            "cluster_agent" => Self::CLUSTER_AGENT,
            "otel" => Self::OTEL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
