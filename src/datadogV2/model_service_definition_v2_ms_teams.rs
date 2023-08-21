// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2MSTeams {
    /// Contact value.
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: String,
    /// Contact Microsoft Teams.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Contact type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ServiceDefinitionV2MSTeamsType,
}

