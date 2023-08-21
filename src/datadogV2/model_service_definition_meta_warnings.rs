// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionMetaWarnings {
    /// The warning instance location.
    #[serde(rename = "instance-location", skip_serializing_if = "Option::is_none")]
    pub instance_location: String,
    /// The warning keyword location.
    #[serde(rename = "keyword-location", skip_serializing_if = "Option::is_none")]
    pub keyword_location: String,
    /// The warning message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
}

