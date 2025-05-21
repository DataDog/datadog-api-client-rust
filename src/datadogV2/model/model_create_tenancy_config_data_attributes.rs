// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CreateTenancyConfigDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateTenancyConfigDataAttributes {
    /// The auth credentials of the user. Consists of a public key fingerprint and private key.
    #[serde(rename = "auth_credentials")]
    pub auth_credentials: crate::datadogV2::model::AuthCredentials,
    /// The config version. It is not recommended to add or change this value, as it is determined internally.
    #[serde(rename = "config_version")]
    pub config_version: Option<i64>,
    /// The OCID of the compartment containing Datadog managed resources.
    #[serde(rename = "dd_compartment_id")]
    pub dd_compartment_id: Option<String>,
    /// The OCID of the resource manager stack for creating Datadog managed resources.
    #[serde(rename = "dd_stack_id")]
    pub dd_stack_id: Option<String>,
    /// The home region of the tenancy to be integrated.
    #[serde(rename = "home_region")]
    pub home_region: String,
    /// The definition of `OCILogsConfig` object.
    #[serde(rename = "logs_config")]
    pub logs_config: Option<crate::datadogV2::model::OCILogsConfig>,
    /// The definition of `OCIMetricsConfig` object.
    #[serde(rename = "metrics_config")]
    pub metrics_config: Option<crate::datadogV2::model::OCIMetricsConfig>,
    /// The definition of `RegionsConfig` object.
    #[serde(rename = "regions_config")]
    pub regions_config: Option<crate::datadogV2::model::RegionsConfig>,
    /// Enable or disable resource collection.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    /// The OCID of the user needed to authenticate and collect data.
    #[serde(rename = "user_ocid")]
    pub user_ocid: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTenancyConfigDataAttributes {
    pub fn new(
        auth_credentials: crate::datadogV2::model::AuthCredentials,
        home_region: String,
        user_ocid: String,
    ) -> CreateTenancyConfigDataAttributes {
        CreateTenancyConfigDataAttributes {
            auth_credentials,
            config_version: None,
            dd_compartment_id: None,
            dd_stack_id: None,
            home_region,
            logs_config: None,
            metrics_config: None,
            regions_config: None,
            resource_collection_enabled: None,
            user_ocid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn config_version(mut self, value: i64) -> Self {
        self.config_version = Some(value);
        self
    }

    pub fn dd_compartment_id(mut self, value: String) -> Self {
        self.dd_compartment_id = Some(value);
        self
    }

    pub fn dd_stack_id(mut self, value: String) -> Self {
        self.dd_stack_id = Some(value);
        self
    }

    pub fn logs_config(mut self, value: crate::datadogV2::model::OCILogsConfig) -> Self {
        self.logs_config = Some(value);
        self
    }

    pub fn metrics_config(mut self, value: crate::datadogV2::model::OCIMetricsConfig) -> Self {
        self.metrics_config = Some(value);
        self
    }

    pub fn regions_config(mut self, value: crate::datadogV2::model::RegionsConfig) -> Self {
        self.regions_config = Some(value);
        self
    }

    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
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

impl<'de> Deserialize<'de> for CreateTenancyConfigDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTenancyConfigDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateTenancyConfigDataAttributesVisitor {
            type Value = CreateTenancyConfigDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_credentials: Option<crate::datadogV2::model::AuthCredentials> = None;
                let mut config_version: Option<i64> = None;
                let mut dd_compartment_id: Option<String> = None;
                let mut dd_stack_id: Option<String> = None;
                let mut home_region: Option<String> = None;
                let mut logs_config: Option<crate::datadogV2::model::OCILogsConfig> = None;
                let mut metrics_config: Option<crate::datadogV2::model::OCIMetricsConfig> = None;
                let mut regions_config: Option<crate::datadogV2::model::RegionsConfig> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut user_ocid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_credentials" => {
                            auth_credentials =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_version" => {
                            if v.is_null() {
                                continue;
                            }
                            config_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dd_compartment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            dd_compartment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dd_stack_id" => {
                            if v.is_null() {
                                continue;
                            }
                            dd_stack_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "home_region" => {
                            home_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_config" => {
                            if v.is_null() {
                                continue;
                            }
                            logs_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics_config" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regions_config" => {
                            if v.is_null() {
                                continue;
                            }
                            regions_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_ocid" => {
                            user_ocid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_credentials =
                    auth_credentials.ok_or_else(|| M::Error::missing_field("auth_credentials"))?;
                let home_region =
                    home_region.ok_or_else(|| M::Error::missing_field("home_region"))?;
                let user_ocid = user_ocid.ok_or_else(|| M::Error::missing_field("user_ocid"))?;

                let content = CreateTenancyConfigDataAttributes {
                    auth_credentials,
                    config_version,
                    dd_compartment_id,
                    dd_stack_id,
                    home_region,
                    logs_config,
                    metrics_config,
                    regions_config,
                    resource_collection_enabled,
                    user_ocid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateTenancyConfigDataAttributesVisitor)
    }
}
