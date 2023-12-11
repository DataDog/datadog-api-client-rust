// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of hosts/containers recorded for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageHostHour {
    /// Contains the total number of infrastructure hosts reporting
    /// during a given hour that were running the Datadog Agent.
    #[serde(
        rename = "agent_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agent_host_count: Option<Option<i64>>,
    /// Contains the total number of hosts that reported through Alibaba integration
    /// (and were NOT running the Datadog Agent).
    #[serde(
        rename = "alibaba_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub alibaba_host_count: Option<Option<i64>>,
    /// Contains the total number of Azure App Services hosts using APM.
    #[serde(
        rename = "apm_azure_app_service_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub apm_azure_app_service_host_count: Option<Option<i64>>,
    /// Shows the total number of hosts using APM during the hour,
    /// these are counted as billable (except during trial periods).
    #[serde(
        rename = "apm_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub apm_host_count: Option<Option<i64>>,
    /// Contains the total number of hosts that reported through the AWS integration
    /// (and were NOT running the Datadog Agent).
    #[serde(
        rename = "aws_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aws_host_count: Option<Option<i64>>,
    /// Contains the total number of hosts that reported through Azure integration
    /// (and were NOT running the Datadog Agent).
    #[serde(
        rename = "azure_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub azure_host_count: Option<Option<i64>>,
    /// Shows the total number of containers reported by the Docker integration during the hour.
    #[serde(
        rename = "container_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub container_count: Option<Option<i64>>,
    /// Contains the total number of hosts that reported through the Google Cloud integration
    /// (and were NOT running the Datadog Agent).
    #[serde(
        rename = "gcp_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub gcp_host_count: Option<Option<i64>>,
    /// Contains the total number of Heroku dynos reported by the Datadog Agent.
    #[serde(
        rename = "heroku_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub heroku_host_count: Option<Option<i64>>,
    /// Contains the total number of billable infrastructure hosts reporting during a given hour.
    /// This is the sum of `agent_host_count`, `aws_host_count`, and `gcp_host_count`.
    #[serde(
        rename = "host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub host_count: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour", default, with = "::serde_with::rust::double_option")]
    pub hour: Option<Option<String>>,
    /// Contains the total number of hosts that reported through the Azure App Services integration
    /// (and were NOT running the Datadog Agent).
    #[serde(
        rename = "infra_azure_app_service",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub infra_azure_app_service: Option<Option<i64>>,
    /// Contains the total number of hosts using APM reported by Datadog exporter for the OpenTelemetry Collector.
    #[serde(
        rename = "opentelemetry_apm_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub opentelemetry_apm_host_count: Option<Option<i64>>,
    /// Contains the total number of hosts reported by Datadog exporter for the OpenTelemetry Collector.
    #[serde(
        rename = "opentelemetry_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub opentelemetry_host_count: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Contains the total number of hosts that reported through vSphere integration
    /// (and were NOT running the Datadog Agent).
    #[serde(
        rename = "vsphere_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub vsphere_host_count: Option<Option<i64>>,
}

impl UsageHostHour {
    pub fn new() -> UsageHostHour {
        UsageHostHour {
            agent_host_count: None,
            alibaba_host_count: None,
            apm_azure_app_service_host_count: None,
            apm_host_count: None,
            aws_host_count: None,
            azure_host_count: None,
            container_count: None,
            gcp_host_count: None,
            heroku_host_count: None,
            host_count: None,
            hour: None,
            infra_azure_app_service: None,
            opentelemetry_apm_host_count: None,
            opentelemetry_host_count: None,
            org_name: None,
            public_id: None,
            vsphere_host_count: None,
        }
    }
}
