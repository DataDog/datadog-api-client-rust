// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCloudSecurityPostureManagementHour {
    /// The number of Cloud Security Posture Management Azure app services hosts during a given hour.
    #[serde(rename = "aas_host_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub aas_host_count: Option<Float64>,
    /// The number of Cloud Security Posture Management AWS hosts during a given hour.
    #[serde(rename = "aws_host_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub aws_host_count: Option<Float64>,
    /// The number of Cloud Security Posture Management Azure hosts during a given hour.
    #[serde(rename = "azure_host_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub azure_host_count: Option<Float64>,
    /// The number of Cloud Security Posture Management hosts during a given hour.
    #[serde(rename = "compliance_host_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub compliance_host_count: Option<Float64>,
    /// The total number of Cloud Security Posture Management containers during a given hour.
    #[serde(rename = "container_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub container_count: Option<Float64>,
    /// The number of Cloud Security Posture Management GCP hosts during a given hour.
    #[serde(rename = "gcp_host_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub gcp_host_count: Option<Float64>,
    /// The total number of Cloud Security Posture Management hosts during a given hour.
    #[serde(rename = "host_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub host_count: Option<Float64>,
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
}

