// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponseData {
    /// Downtime match details.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: MonitorDowntimeMatchResponseAttributes,
    /// The downtime ID.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub id: Option<String>,
    /// Monitor Downtime Match resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MonitorDowntimeMatchResourceType,
}

