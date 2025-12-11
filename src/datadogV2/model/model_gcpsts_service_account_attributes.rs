// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes associated with your service account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPSTSServiceAccountAttributes {
    /// Tags to be associated with GCP metrics and service checks from your account.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// Silence monitors for expected GCE instance shutdowns.
    #[serde(rename = "automute")]
    pub automute: Option<bool>,
    /// Your service account email address.
    #[serde(rename = "client_email")]
    pub client_email: Option<String>,
    /// List of filters to limit the Cloud Run revisions that are pulled into Datadog by using tags.
    /// Only Cloud Run revision resources that apply to specified filters are imported into Datadog.
    /// **Note:** This field is deprecated. Instead, use `monitored_resource_configs` with `type=cloud_run_revision`
    #[deprecated]
    #[serde(rename = "cloud_run_revision_filters")]
    pub cloud_run_revision_filters: Option<Vec<String>>,
    /// List of filters to limit the VM instances that are pulled into Datadog by using tags.
    /// Only VM instance resources that apply to specified filters are imported into Datadog.
    /// **Note:** This field is deprecated. Instead, use `monitored_resource_configs` with `type=gce_instance`
    #[deprecated]
    #[serde(rename = "host_filters")]
    pub host_filters: Option<Vec<String>>,
    /// When enabled, Datadog will activate the Cloud Security Monitoring product for this service account. Note: This requires resource_collection_enabled to be set to true.
    #[serde(rename = "is_cspm_enabled")]
    pub is_cspm_enabled: Option<bool>,
    /// When enabled, Datadog collects metrics where location is explicitly stated as "global" or where location information cannot be deduced from GCP labels.
    #[serde(rename = "is_global_location_enabled")]
    pub is_global_location_enabled: Option<bool>,
    /// When enabled, Datadog applies the `X-Goog-User-Project` header, attributing Google Cloud billing and quota usage to the project being monitored rather than the default service account project.
    #[serde(rename = "is_per_project_quota_enabled")]
    pub is_per_project_quota_enabled: Option<bool>,
    /// When enabled, Datadog scans for all resource change data in your Google Cloud environment.
    #[serde(rename = "is_resource_change_collection_enabled")]
    pub is_resource_change_collection_enabled: Option<bool>,
    /// When enabled, Datadog will attempt to collect Security Command Center Findings. Note: This requires additional permissions on the service account.
    #[serde(rename = "is_security_command_center_enabled")]
    pub is_security_command_center_enabled: Option<bool>,
    /// Configurations for GCP metric namespaces.
    #[serde(rename = "metric_namespace_configs")]
    pub metric_namespace_configs: Option<Vec<crate::datadogV2::model::GCPMetricNamespaceConfig>>,
    /// Configurations for GCP monitored resources.
    #[serde(rename = "monitored_resource_configs")]
    pub monitored_resource_configs:
        Option<Vec<crate::datadogV2::model::GCPMonitoredResourceConfig>>,
    /// Configurations for GCP location filtering, such as region, multi-region, or zone. Only monitored resources that match the specified regions are imported into Datadog. By default, Datadog collects from all locations.
    #[serde(rename = "region_filter_configs")]
    pub region_filter_configs: Option<Vec<String>>,
    /// When enabled, Datadog scans for all resources in your GCP environment.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPSTSServiceAccountAttributes {
    pub fn new() -> GCPSTSServiceAccountAttributes {
        #[allow(deprecated)]
        GCPSTSServiceAccountAttributes {
            account_tags: None,
            automute: None,
            client_email: None,
            cloud_run_revision_filters: None,
            host_filters: None,
            is_cspm_enabled: None,
            is_global_location_enabled: None,
            is_per_project_quota_enabled: None,
            is_resource_change_collection_enabled: None,
            is_security_command_center_enabled: None,
            metric_namespace_configs: None,
            monitored_resource_configs: None,
            region_filter_configs: None,
            resource_collection_enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn automute(mut self, value: bool) -> Self {
        self.automute = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn client_email(mut self, value: String) -> Self {
        self.client_email = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cloud_run_revision_filters(mut self, value: Vec<String>) -> Self {
        self.cloud_run_revision_filters = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn host_filters(mut self, value: Vec<String>) -> Self {
        self.host_filters = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn is_cspm_enabled(mut self, value: bool) -> Self {
        self.is_cspm_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn is_global_location_enabled(mut self, value: bool) -> Self {
        self.is_global_location_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn is_per_project_quota_enabled(mut self, value: bool) -> Self {
        self.is_per_project_quota_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn is_resource_change_collection_enabled(mut self, value: bool) -> Self {
        self.is_resource_change_collection_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn is_security_command_center_enabled(mut self, value: bool) -> Self {
        self.is_security_command_center_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn metric_namespace_configs(
        mut self,
        value: Vec<crate::datadogV2::model::GCPMetricNamespaceConfig>,
    ) -> Self {
        self.metric_namespace_configs = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn monitored_resource_configs(
        mut self,
        value: Vec<crate::datadogV2::model::GCPMonitoredResourceConfig>,
    ) -> Self {
        self.monitored_resource_configs = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn region_filter_configs(mut self, value: Vec<String>) -> Self {
        self.region_filter_configs = Some(value);
        self
    }

    #[allow(deprecated)]
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

impl Default for GCPSTSServiceAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPSTSServiceAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPSTSServiceAccountAttributesVisitor;
        impl<'a> Visitor<'a> for GCPSTSServiceAccountAttributesVisitor {
            type Value = GCPSTSServiceAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Vec<String>> = None;
                let mut automute: Option<bool> = None;
                let mut client_email: Option<String> = None;
                let mut cloud_run_revision_filters: Option<Vec<String>> = None;
                let mut host_filters: Option<Vec<String>> = None;
                let mut is_cspm_enabled: Option<bool> = None;
                let mut is_global_location_enabled: Option<bool> = None;
                let mut is_per_project_quota_enabled: Option<bool> = None;
                let mut is_resource_change_collection_enabled: Option<bool> = None;
                let mut is_security_command_center_enabled: Option<bool> = None;
                let mut metric_namespace_configs: Option<
                    Vec<crate::datadogV2::model::GCPMetricNamespaceConfig>,
                > = None;
                let mut monitored_resource_configs: Option<
                    Vec<crate::datadogV2::model::GCPMonitoredResourceConfig>,
                > = None;
                let mut region_filter_configs: Option<Vec<String>> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            account_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "automute" => {
                            if v.is_null() {
                                continue;
                            }
                            automute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_email" => {
                            if v.is_null() {
                                continue;
                            }
                            client_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_run_revision_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_run_revision_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            host_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_cspm_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_cspm_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_global_location_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_global_location_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_per_project_quota_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_per_project_quota_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_resource_change_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_resource_change_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_security_command_center_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_security_command_center_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_namespace_configs" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_namespace_configs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitored_resource_configs" => {
                            if v.is_null() {
                                continue;
                            }
                            monitored_resource_configs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region_filter_configs" => {
                            if v.is_null() {
                                continue;
                            }
                            region_filter_configs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = GCPSTSServiceAccountAttributes {
                    account_tags,
                    automute,
                    client_email,
                    cloud_run_revision_filters,
                    host_filters,
                    is_cspm_enabled,
                    is_global_location_enabled,
                    is_per_project_quota_enabled,
                    is_resource_change_collection_enabled,
                    is_security_command_center_enabled,
                    metric_namespace_configs,
                    monitored_resource_configs,
                    region_filter_configs,
                    resource_collection_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPSTSServiceAccountAttributesVisitor)
    }
}
