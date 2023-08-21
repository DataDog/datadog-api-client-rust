// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchServiceLevelObjectiveData {
    /// A service level objective object includes a service level indicator, thresholds
for one or more timeframes, and metadata (`name`, `description`, and `tags`).
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SearchServiceLevelObjectiveAttributes,
    /// A unique identifier for the service level objective object.

Always included in service level objective responses.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The type of the object, must be `slo`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

