// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TenancyConfigDataAttributes {
    #[serde(rename = "billing_plan_id")]
    pub billing_plan_id: Option<i32>,
    #[serde(rename = "config_version")]
    pub config_version: Option<i64>,
    #[serde(rename = "cost_collection_enabled")]
    pub cost_collection_enabled: Option<bool>,
    #[serde(rename = "dd_compartment_id")]
    pub dd_compartment_id: Option<String>,
    #[serde(rename = "dd_stack_id")]
    pub dd_stack_id: Option<String>,
    #[serde(rename = "home_region")]
    pub home_region: Option<String>,
    #[serde(rename = "logs_config")]
    pub logs_config: Option<crate::datadogV2::model::TenancyConfigDataAttributesLogsConfig>,
    #[serde(rename = "metrics_config")]
    pub metrics_config: Option<crate::datadogV2::model::TenancyConfigDataAttributesMetricsConfig>,
    #[serde(rename = "parent_tenancy_name")]
    pub parent_tenancy_name: Option<String>,
    #[serde(rename = "regions_config")]
    pub regions_config: Option<crate::datadogV2::model::TenancyConfigDataAttributesRegionsConfig>,
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    #[serde(rename = "tenancy_name")]
    pub tenancy_name: Option<String>,
    #[serde(rename = "user_ocid")]
    pub user_ocid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TenancyConfigDataAttributes {
    pub fn new() -> TenancyConfigDataAttributes {
        TenancyConfigDataAttributes {
            billing_plan_id: None,
            config_version: None,
            cost_collection_enabled: None,
            dd_compartment_id: None,
            dd_stack_id: None,
            home_region: None,
            logs_config: None,
            metrics_config: None,
            parent_tenancy_name: None,
            regions_config: None,
            resource_collection_enabled: None,
            tenancy_name: None,
            user_ocid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn billing_plan_id(mut self, value: i32) -> Self {
        self.billing_plan_id = Some(value);
        self
    }

    pub fn config_version(mut self, value: i64) -> Self {
        self.config_version = Some(value);
        self
    }

    pub fn cost_collection_enabled(mut self, value: bool) -> Self {
        self.cost_collection_enabled = Some(value);
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

    pub fn home_region(mut self, value: String) -> Self {
        self.home_region = Some(value);
        self
    }

    pub fn logs_config(
        mut self,
        value: crate::datadogV2::model::TenancyConfigDataAttributesLogsConfig,
    ) -> Self {
        self.logs_config = Some(value);
        self
    }

    pub fn metrics_config(
        mut self,
        value: crate::datadogV2::model::TenancyConfigDataAttributesMetricsConfig,
    ) -> Self {
        self.metrics_config = Some(value);
        self
    }

    pub fn parent_tenancy_name(mut self, value: String) -> Self {
        self.parent_tenancy_name = Some(value);
        self
    }

    pub fn regions_config(
        mut self,
        value: crate::datadogV2::model::TenancyConfigDataAttributesRegionsConfig,
    ) -> Self {
        self.regions_config = Some(value);
        self
    }

    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
        self
    }

    pub fn tenancy_name(mut self, value: String) -> Self {
        self.tenancy_name = Some(value);
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

impl Default for TenancyConfigDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TenancyConfigDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TenancyConfigDataAttributesVisitor;
        impl<'a> Visitor<'a> for TenancyConfigDataAttributesVisitor {
            type Value = TenancyConfigDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billing_plan_id: Option<i32> = None;
                let mut config_version: Option<i64> = None;
                let mut cost_collection_enabled: Option<bool> = None;
                let mut dd_compartment_id: Option<String> = None;
                let mut dd_stack_id: Option<String> = None;
                let mut home_region: Option<String> = None;
                let mut logs_config: Option<
                    crate::datadogV2::model::TenancyConfigDataAttributesLogsConfig,
                > = None;
                let mut metrics_config: Option<
                    crate::datadogV2::model::TenancyConfigDataAttributesMetricsConfig,
                > = None;
                let mut parent_tenancy_name: Option<String> = None;
                let mut regions_config: Option<
                    crate::datadogV2::model::TenancyConfigDataAttributesRegionsConfig,
                > = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut tenancy_name: Option<String> = None;
                let mut user_ocid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billing_plan_id" => {
                            if v.is_null() {
                                continue;
                            }
                            billing_plan_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_version" => {
                            if v.is_null() {
                                continue;
                            }
                            config_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cost_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            cost_collection_enabled =
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
                        "parent_tenancy_name" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_tenancy_name =
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
                        "tenancy_name" => {
                            if v.is_null() {
                                continue;
                            }
                            tenancy_name =
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

                let content = TenancyConfigDataAttributes {
                    billing_plan_id,
                    config_version,
                    cost_collection_enabled,
                    dd_compartment_id,
                    dd_stack_id,
                    home_region,
                    logs_config,
                    metrics_config,
                    parent_tenancy_name,
                    regions_config,
                    resource_collection_enabled,
                    tenancy_name,
                    user_ocid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TenancyConfigDataAttributesVisitor)
    }
}
