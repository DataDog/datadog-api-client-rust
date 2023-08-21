// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationCreateAttributes {
    /// Name of the RUM application.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

