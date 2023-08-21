// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestrictionPolicyBinding {
    /// An array of principals. A principal is a subject or group of subjects.
Each principal is formatted as `type:id`. Supported types: `role`, `team` (beta), `user` (beta), and `org`.
The org ID can be obtained through the api/v2/current_user API.
The user principal type accepts service account IDs.
    #[serde(rename = "principals", skip_serializing_if = "Option::is_none")]
    pub principals: Vec<String>,
    /// The role/level of access.
    #[serde(rename = "relation", skip_serializing_if = "Option::is_none")]
    pub relation: String,
}

