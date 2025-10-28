// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Returns the AWS account associated with this integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccount {
    /// Your AWS access key ID. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "access_key_id")]
    pub access_key_id: Option<String>,
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// An object (in the form `{"namespace1":true/false, "namespace2":true/false}`) containing user-supplied overrides
    /// for AWS namespace metric collection. **Important**: This field only contains namespaces explicitly configured through API calls,
    /// not the comprehensive enabled or disabled status of all namespaces. If a namespace is absent from this field, it uses Datadog's
    /// internal defaults (all namespaces enabled by default, except `AWS/SQS`, `AWS/ElasticMapReduce`, and `AWS/Usage`).
    /// For a complete view of all namespace statuses, use the V2 AWS Integration API instead.
    #[serde(rename = "account_specific_namespace_rules")]
    pub account_specific_namespace_rules: Option<std::collections::BTreeMap<String, bool>>,
    /// Whether Datadog collects cloud security posture management resources from your AWS account. This includes additional resources not covered under the general `resource_collection`.
    #[serde(rename = "cspm_resource_collection_enabled")]
    pub cspm_resource_collection_enabled: Option<bool>,
    /// An array of [AWS regions](<https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints>)
    /// to exclude from metrics collection.
    #[serde(rename = "excluded_regions")]
    pub excluded_regions: Option<Vec<String>>,
    /// Whether Datadog collects additional attributes and configuration information about the resources in your AWS account. Required for `cspm_resource_collection`.
    #[serde(rename = "extended_resource_collection_enabled")]
    pub extended_resource_collection_enabled: Option<bool>,
    /// The array of EC2 tags (in the form `key:value`) defines a filter that Datadog uses when collecting metrics from EC2.
    /// Wildcards, such as `?` (for single characters) and `*` (for multiple characters) can also be used.
    /// Only hosts that match one of the defined tags
    /// will be imported into Datadog. The rest will be ignored.
    /// Host matching a given tag can also be excluded by adding `!` before the tag.
    /// For example, `env:production,instance-type:c1.*,!region:us-east-1`
    #[serde(rename = "filter_tags")]
    pub filter_tags: Option<Vec<String>>,
    /// Array of tags (in the form `key:value`) to add to all hosts
    /// and metrics reporting through this integration.
    #[serde(rename = "host_tags")]
    pub host_tags: Option<Vec<String>>,
    /// Whether Datadog collects metrics for this AWS account.
    #[serde(rename = "metrics_collection_enabled")]
    pub metrics_collection_enabled: Option<bool>,
    /// Deprecated in favor of 'extended_resource_collection_enabled'. Whether Datadog collects a standard set of resources from your AWS account.
    #[deprecated]
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    /// Your Datadog role delegation name.
    #[serde(rename = "role_name")]
    pub role_name: Option<String>,
    /// Your AWS secret access key. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "secret_access_key")]
    pub secret_access_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAccount {
    pub fn new() -> AWSAccount {
        #[allow(deprecated)]
        AWSAccount {
            access_key_id: None,
            account_id: None,
            account_specific_namespace_rules: None,
            cspm_resource_collection_enabled: None,
            excluded_regions: None,
            extended_resource_collection_enabled: None,
            filter_tags: None,
            host_tags: None,
            metrics_collection_enabled: None,
            resource_collection_enabled: None,
            role_name: None,
            secret_access_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn access_key_id(mut self, value: String) -> Self {
        self.access_key_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn account_specific_namespace_rules(
        mut self,
        value: std::collections::BTreeMap<String, bool>,
    ) -> Self {
        self.account_specific_namespace_rules = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn cspm_resource_collection_enabled(mut self, value: bool) -> Self {
        self.cspm_resource_collection_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn excluded_regions(mut self, value: Vec<String>) -> Self {
        self.excluded_regions = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn extended_resource_collection_enabled(mut self, value: bool) -> Self {
        self.extended_resource_collection_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn filter_tags(mut self, value: Vec<String>) -> Self {
        self.filter_tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn host_tags(mut self, value: Vec<String>) -> Self {
        self.host_tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn metrics_collection_enabled(mut self, value: bool) -> Self {
        self.metrics_collection_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn role_name(mut self, value: String) -> Self {
        self.role_name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn secret_access_key(mut self, value: String) -> Self {
        self.secret_access_key = Some(value);
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

impl Default for AWSAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountVisitor;
        impl<'a> Visitor<'a> for AWSAccountVisitor {
            type Value = AWSAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_key_id: Option<String> = None;
                let mut account_id: Option<String> = None;
                let mut account_specific_namespace_rules: Option<
                    std::collections::BTreeMap<String, bool>,
                > = None;
                let mut cspm_resource_collection_enabled: Option<bool> = None;
                let mut excluded_regions: Option<Vec<String>> = None;
                let mut extended_resource_collection_enabled: Option<bool> = None;
                let mut filter_tags: Option<Vec<String>> = None;
                let mut host_tags: Option<Vec<String>> = None;
                let mut metrics_collection_enabled: Option<bool> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut role_name: Option<String> = None;
                let mut secret_access_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            access_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_specific_namespace_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            account_specific_namespace_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "excluded_regions" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_regions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extended_resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            extended_resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            host_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role_name" => {
                            if v.is_null() {
                                continue;
                            }
                            role_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret_access_key" => {
                            if v.is_null() {
                                continue;
                            }
                            secret_access_key =
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
                let content = AWSAccount {
                    access_key_id,
                    account_id,
                    account_specific_namespace_rules,
                    cspm_resource_collection_enabled,
                    excluded_regions,
                    extended_resource_collection_enabled,
                    filter_tags,
                    host_tags,
                    metrics_collection_enabled,
                    resource_collection_enabled,
                    role_name,
                    secret_access_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountVisitor)
    }
}
