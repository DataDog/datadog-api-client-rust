// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of hosts/containers recorded for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub hour: Option<Option<chrono::DateTime<chrono::Utc>>>,
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_host_count(mut self, value: Option<i64>) -> Self {
        self.agent_host_count = Some(value);
        self
    }

    pub fn alibaba_host_count(mut self, value: Option<i64>) -> Self {
        self.alibaba_host_count = Some(value);
        self
    }

    pub fn apm_azure_app_service_host_count(mut self, value: Option<i64>) -> Self {
        self.apm_azure_app_service_host_count = Some(value);
        self
    }

    pub fn apm_host_count(mut self, value: Option<i64>) -> Self {
        self.apm_host_count = Some(value);
        self
    }

    pub fn aws_host_count(mut self, value: Option<i64>) -> Self {
        self.aws_host_count = Some(value);
        self
    }

    pub fn azure_host_count(mut self, value: Option<i64>) -> Self {
        self.azure_host_count = Some(value);
        self
    }

    pub fn container_count(mut self, value: Option<i64>) -> Self {
        self.container_count = Some(value);
        self
    }

    pub fn gcp_host_count(mut self, value: Option<i64>) -> Self {
        self.gcp_host_count = Some(value);
        self
    }

    pub fn heroku_host_count(mut self, value: Option<i64>) -> Self {
        self.heroku_host_count = Some(value);
        self
    }

    pub fn host_count(mut self, value: Option<i64>) -> Self {
        self.host_count = Some(value);
        self
    }

    pub fn hour(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn infra_azure_app_service(mut self, value: Option<i64>) -> Self {
        self.infra_azure_app_service = Some(value);
        self
    }

    pub fn opentelemetry_apm_host_count(mut self, value: Option<i64>) -> Self {
        self.opentelemetry_apm_host_count = Some(value);
        self
    }

    pub fn opentelemetry_host_count(mut self, value: Option<i64>) -> Self {
        self.opentelemetry_host_count = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn vsphere_host_count(mut self, value: Option<i64>) -> Self {
        self.vsphere_host_count = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for UsageHostHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageHostHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageHostHourVisitor;
        impl<'a> Visitor<'a> for UsageHostHourVisitor {
            type Value = UsageHostHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_host_count: Option<Option<i64>> = None;
                let mut alibaba_host_count: Option<Option<i64>> = None;
                let mut apm_azure_app_service_host_count: Option<Option<i64>> = None;
                let mut apm_host_count: Option<Option<i64>> = None;
                let mut aws_host_count: Option<Option<i64>> = None;
                let mut azure_host_count: Option<Option<i64>> = None;
                let mut container_count: Option<Option<i64>> = None;
                let mut gcp_host_count: Option<Option<i64>> = None;
                let mut heroku_host_count: Option<Option<i64>> = None;
                let mut host_count: Option<Option<i64>> = None;
                let mut hour: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut infra_azure_app_service: Option<Option<i64>> = None;
                let mut opentelemetry_apm_host_count: Option<Option<i64>> = None;
                let mut opentelemetry_host_count: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut vsphere_host_count: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_host_count" => {
                            agent_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "alibaba_host_count" => {
                            alibaba_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_azure_app_service_host_count" => {
                            apm_azure_app_service_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_host_count" => {
                            apm_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_host_count" => {
                            aws_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_host_count" => {
                            azure_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_count" => {
                            container_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_host_count" => {
                            gcp_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "heroku_host_count" => {
                            heroku_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_count" => {
                            host_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "infra_azure_app_service" => {
                            infra_azure_app_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opentelemetry_apm_host_count" => {
                            opentelemetry_apm_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opentelemetry_host_count" => {
                            opentelemetry_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vsphere_host_count" => {
                            vsphere_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageHostHour {
                    agent_host_count,
                    alibaba_host_count,
                    apm_azure_app_service_host_count,
                    apm_host_count,
                    aws_host_count,
                    azure_host_count,
                    container_count,
                    gcp_host_count,
                    heroku_host_count,
                    host_count,
                    hour,
                    infra_azure_app_service,
                    opentelemetry_apm_host_count,
                    opentelemetry_host_count,
                    org_name,
                    public_id,
                    vsphere_host_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageHostHourVisitor)
    }
}
