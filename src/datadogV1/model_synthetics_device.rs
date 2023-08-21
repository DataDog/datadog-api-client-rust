// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDevice {
    /// Screen height of the device.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: i64,
    /// The device ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: SyntheticsDeviceID,
    /// Whether or not the device is a mobile.
    #[serde(rename = "isMobile", skip_serializing_if = "Option::is_none")]
    pub is_mobile: bool,
    /// The device name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Screen width of the device.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: i64,
}

