// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceCreateAttributes {
    /// The custom URL for a custom region.
    #[serde(rename = "custom_url", skip_serializing_if = "Option::is_none")]
    pub custom_url: String,
    /// The name for the Opsgenie service.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The Opsgenie API key for your Opsgenie service.
    #[serde(rename = "opsgenie_api_key", skip_serializing_if = "Option::is_none")]
    pub opsgenie_api_key: String,
    /// The region for the Opsgenie service.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: OpsgenieServiceRegionType,
}

