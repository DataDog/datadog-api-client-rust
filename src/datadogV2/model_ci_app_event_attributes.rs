// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppEventAttributes {
    /// JSON object of attributes from CI Visibility test events.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: map[string]interface{},
    /// Array of tags associated with your event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Test run level.
    #[serde(rename = "test_level", skip_serializing_if = "Option::is_none")]
    pub test_level: CIAppTestLevel,
}

