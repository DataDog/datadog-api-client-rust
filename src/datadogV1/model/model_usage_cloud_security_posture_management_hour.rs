// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cloud Security Management Pro usage for a given organization for a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn aas_host_count(mut self, value: Option<f64>) -> Self {
        self.aas_host_count = Some(value);
        self
    }

    pub fn aws_host_count(mut self, value: Option<f64>) -> Self {
        self.aws_host_count = Some(value);
        self
    }

    pub fn azure_host_count(mut self, value: Option<f64>) -> Self {
        self.azure_host_count = Some(value);
        self
    }

    pub fn compliance_host_count(mut self, value: Option<f64>) -> Self {
        self.compliance_host_count = Some(value);
        self
    }

    pub fn container_count(mut self, value: Option<f64>) -> Self {
        self.container_count = Some(value);
        self
    }

    pub fn gcp_host_count(mut self, value: Option<f64>) -> Self {
        self.gcp_host_count = Some(value);
        self
    }

    pub fn host_count(mut self, value: Option<f64>) -> Self {
        self.host_count = Some(value);
        self
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
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
}

impl Default for UsageCloudSecurityPostureManagementHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageCloudSecurityPostureManagementHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageCloudSecurityPostureManagementHourVisitor;
        impl<'a> Visitor<'a> for UsageCloudSecurityPostureManagementHourVisitor {
            type Value = UsageCloudSecurityPostureManagementHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aas_host_count: Option<Option<f64>> = None;
                let mut aws_host_count: Option<Option<f64>> = None;
                let mut azure_host_count: Option<Option<f64>> = None;
                let mut compliance_host_count: Option<Option<f64>> = None;
                let mut container_count: Option<Option<f64>> = None;
                let mut gcp_host_count: Option<Option<f64>> = None;
                let mut host_count: Option<Option<f64>> = None;
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aas_host_count" => {
                            aas_host_count =
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
                        "compliance_host_count" => {
                            compliance_host_count =
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
                        "host_count" => {
                            host_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }

                let content = UsageCloudSecurityPostureManagementHour {
                    aas_host_count,
                    aws_host_count,
                    azure_host_count,
                    compliance_host_count,
                    container_count,
                    gcp_host_count,
                    host_count,
                    hour,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageCloudSecurityPostureManagementHourVisitor)
    }
}
