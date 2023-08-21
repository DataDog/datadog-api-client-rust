// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsListServicesResponse {
    /// Key value in returned object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Name of service available for configuration with Datadog logs.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: String,
}

