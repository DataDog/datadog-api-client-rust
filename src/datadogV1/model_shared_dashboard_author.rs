// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardAuthor {
    /// Identifier of the user who shared the dashboard.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// Name of the user who shared the dashboard.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
}

