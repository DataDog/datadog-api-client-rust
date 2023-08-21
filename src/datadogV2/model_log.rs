// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    /// JSON object containing all log attributes and their associated values.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: LogAttributes,
    /// Unique ID of the Log.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Type of the event.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: LogType,
}

