// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an existing OCI tenancy integration configuration, including optional credentials, region settings, and collection options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateTenancyConfigDataAttributes {
    /// OCI API signing key credentials used to update the Datadog integration's authentication with the OCI tenancy.
    #[serde(rename = "auth_credentials")]
    pub auth_credentials:
        Option<crate::datadogV2::model::UpdateTenancyConfigDataAttributesAuthCredentials>,
    /// Whether cost data collection from OCI is enabled for the tenancy.
    #[serde(rename = "cost_collection_enabled")]
    pub cost_collection_enabled: Option<bool>,
    /// The home region of the OCI tenancy (for example, us-ashburn-1).
    #[serde(rename = "home_region")]
    pub home_region: Option<String>,
    /// Log collection configuration for updating an OCI tenancy, controlling which compartments and services have log collection enabled.
    #[serde(rename = "logs_config")]
    pub logs_config: Option<crate::datadogV2::model::UpdateTenancyConfigDataAttributesLogsConfig>,
    /// Metrics collection configuration for updating an OCI tenancy, controlling which compartments and services are included or excluded.
    #[serde(rename = "metrics_config")]
    pub metrics_config:
        Option<crate::datadogV2::model::UpdateTenancyConfigDataAttributesMetricsConfig>,
    /// Region configuration for updating an OCI tenancy, specifying which regions are available, enabled, or disabled for data collection.
    #[serde(rename = "regions_config")]
    pub regions_config:
        Option<crate::datadogV2::model::UpdateTenancyConfigDataAttributesRegionsConfig>,
    /// Whether resource collection from OCI is enabled for the tenancy.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    /// The OCID of the OCI user used by the Datadog integration for authentication.
    #[serde(rename = "user_ocid")]
    pub user_ocid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateTenancyConfigDataAttributes {
    pub fn new() -> UpdateTenancyConfigDataAttributes {
        UpdateTenancyConfigDataAttributes {
            auth_credentials: None,
            cost_collection_enabled: None,
            home_region: None,
            logs_config: None,
            metrics_config: None,
            regions_config: None,
            resource_collection_enabled: None,
            user_ocid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth_credentials(
        mut self,
        value: crate::datadogV2::model::UpdateTenancyConfigDataAttributesAuthCredentials,
    ) -> Self {
        self.auth_credentials = Some(value);
        self
    }

    pub fn cost_collection_enabled(mut self, value: bool) -> Self {
        self.cost_collection_enabled = Some(value);
        self
    }

    pub fn home_region(mut self, value: String) -> Self {
        self.home_region = Some(value);
        self
    }

    pub fn logs_config(
        mut self,
        value: crate::datadogV2::model::UpdateTenancyConfigDataAttributesLogsConfig,
    ) -> Self {
        self.logs_config = Some(value);
        self
    }

    pub fn metrics_config(
        mut self,
        value: crate::datadogV2::model::UpdateTenancyConfigDataAttributesMetricsConfig,
    ) -> Self {
        self.metrics_config = Some(value);
        self
    }

    pub fn regions_config(
        mut self,
        value: crate::datadogV2::model::UpdateTenancyConfigDataAttributesRegionsConfig,
    ) -> Self {
        self.regions_config = Some(value);
        self
    }

    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
        self
    }

    pub fn user_ocid(mut self, value: String) -> Self {
        self.user_ocid = Some(value);
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

impl Default for UpdateTenancyConfigDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateTenancyConfigDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateTenancyConfigDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateTenancyConfigDataAttributesVisitor {
            type Value = UpdateTenancyConfigDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_credentials: Option<
                    crate::datadogV2::model::UpdateTenancyConfigDataAttributesAuthCredentials,
                > = None;
                let mut cost_collection_enabled: Option<bool> = None;
                let mut home_region: Option<String> = None;
                let mut logs_config: Option<
                    crate::datadogV2::model::UpdateTenancyConfigDataAttributesLogsConfig,
                > = None;
                let mut metrics_config: Option<
                    crate::datadogV2::model::UpdateTenancyConfigDataAttributesMetricsConfig,
                > = None;
                let mut regions_config: Option<
                    crate::datadogV2::model::UpdateTenancyConfigDataAttributesRegionsConfig,
                > = None;
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
                            if v.is_null() {
                                continue;
                            }
                            auth_credentials =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cost_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            cost_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "home_region" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            user_ocid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateTenancyConfigDataAttributes {
                    auth_credentials,
                    cost_collection_enabled,
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

        deserializer.deserialize_any(UpdateTenancyConfigDataAttributesVisitor)
    }
}
