// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchIssuesIncludeQueryParameterItem {
    ISSUE,
    ISSUE_ASSIGNEE,
    ISSUE_CASE,
    ISSUE_TEAM_OWNERS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SearchIssuesIncludeQueryParameterItem {
    fn to_string(&self) -> String {
        match self {
            Self::ISSUE => String::from("issue"),
            Self::ISSUE_ASSIGNEE => String::from("issue.assignee"),
            Self::ISSUE_CASE => String::from("issue.case"),
            Self::ISSUE_TEAM_OWNERS => String::from("issue.team_owners"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SearchIssuesIncludeQueryParameterItem {
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

impl<'de> Deserialize<'de> for SearchIssuesIncludeQueryParameterItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "issue" => Self::ISSUE,
            "issue.assignee" => Self::ISSUE_ASSIGNEE,
            "issue.case" => Self::ISSUE_CASE,
            "issue.team_owners" => Self::ISSUE_TEAM_OWNERS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
