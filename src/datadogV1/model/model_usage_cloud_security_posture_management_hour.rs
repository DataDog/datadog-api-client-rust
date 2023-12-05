// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Cloud Security Management Pro usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageCloudSecurityPostureManagementHour {
    /// The number of Cloud Security Management Pro Azure app services hosts during a given hour.
    #[serde(
        rename = "aas_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aas_host_count: Option<Option<f64>>,
    /// The number of Cloud Security Management Pro AWS hosts during a given hour.
    #[serde(
        rename = "aws_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aws_host_count: Option<Option<f64>>,
    /// The number of Cloud Security Management Pro Azure hosts during a given hour.
    #[serde(
        rename = "azure_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub azure_host_count: Option<Option<f64>>,
    /// The number of Cloud Security Management Pro hosts during a given hour.
    #[serde(
        rename = "compliance_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub compliance_host_count: Option<Option<f64>>,
    /// The total number of Cloud Security Management Pro containers during a given hour.
    #[serde(
        rename = "container_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub container_count: Option<Option<f64>>,
    /// The number of Cloud Security Management Pro GCP hosts during a given hour.
    #[serde(
        rename = "gcp_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub gcp_host_count: Option<Option<f64>>,
    /// The total number of Cloud Security Management Pro hosts during a given hour.
    #[serde(
        rename = "host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub host_count: Option<Option<f64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageCloudSecurityPostureManagementHour {
    pub fn new() -> UsageCloudSecurityPostureManagementHour {
        UsageCloudSecurityPostureManagementHour {
            aas_host_count: None,
            aws_host_count: None,
            azure_host_count: None,
            compliance_host_count: None,
            container_count: None,
            gcp_host_count: None,
            host_count: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }
}