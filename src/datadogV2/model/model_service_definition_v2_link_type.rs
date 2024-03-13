// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceDefinitionV2LinkType {
    DOC,
    WIKI,
    RUNBOOK,
    URL,
    REPO,
    DASHBOARD,
    ONCALL,
    CODE,
    LINK,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ServiceDefinitionV2LinkType {
    fn to_string(&self) -> String {
        match self {
            Self::DOC => String::from("doc"),
            Self::WIKI => String::from("wiki"),
            Self::RUNBOOK => String::from("runbook"),
            Self::URL => String::from("url"),
            Self::REPO => String::from("repo"),
            Self::DASHBOARD => String::from("dashboard"),
            Self::ONCALL => String::from("oncall"),
            Self::CODE => String::from("code"),
            Self::LINK => String::from("link"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ServiceDefinitionV2LinkType {
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

impl<'de> Deserialize<'de> for ServiceDefinitionV2LinkType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "doc" => Self::DOC,
            "wiki" => Self::WIKI,
            "runbook" => Self::RUNBOOK,
            "url" => Self::URL,
            "repo" => Self::REPO,
            "dashboard" => Self::DASHBOARD,
            "oncall" => Self::ONCALL,
            "code" => Self::CODE,
            "link" => Self::LINK,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
