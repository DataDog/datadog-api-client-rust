// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Contact {
    /// Service owner’s email.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Service owner’s Slack channel.
    #[serde(rename = "slack", skip_serializing_if = "Option::is_none")]
    pub slack: String,
}

