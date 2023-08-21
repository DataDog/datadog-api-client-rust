// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMEventAttributes {
    /// JSON object of attributes from RUM events.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: map[string]interface{},
    /// The name of the application or service generating RUM events.
It is used to switch from RUM to APM, so make sure you define the same
value when you use both products.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// Array of tags associated with your event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Timestamp of your event.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: String,
}

