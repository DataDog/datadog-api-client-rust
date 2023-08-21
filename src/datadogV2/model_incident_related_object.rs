// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentRelatedObject {
    #[serde(rename = "users")]
	USERS,
    #[serde(rename = "attachments")]
	ATTACHMENTS,
}

impl ToString for IncidentRelatedObject {
    fn to_string(&self) -> String {
        match self {
            Self::USERS => String::from("users"),
            Self::ATTACHMENTS => String::from("attachments"),
        }
    }
}

impl Default for IncidentRelatedObject {
    fn default() -> IncidentRelatedObject {
        Self::USERS
    }
}
