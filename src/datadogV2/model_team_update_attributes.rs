// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamUpdateAttributes {
    /// An identifier for the color representing the team
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: i32,
    /// Free-form markdown description/content for the team's homepage
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// The team's identifier
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// The name of the team
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}

