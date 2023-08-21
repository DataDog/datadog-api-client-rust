// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigStatusAttributes {
    /// A list of account emails to notify when the configuration is applied.
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Vec<String>,
    /// The status of the request.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// A list of tag names to apply to the configuration.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

