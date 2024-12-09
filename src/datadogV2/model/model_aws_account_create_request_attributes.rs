// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The AWS Account Integration Config to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccountCreateRequestAttributes {
    /// Tags to apply to all hosts and metrics reporting for this account. Defaults to `[]`.
    #[serde(
        rename = "account_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub account_tags: Option<Option<Vec<String>>>,
    /// AWS Authentication config.
    #[serde(rename = "auth_config")]
    pub auth_config: crate::datadogV2::model::AWSAuthConfig,
    /// AWS Account ID.
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: String,
    /// AWS partition your AWS account is scoped to. Defaults to `aws`.
    /// See [Partitions](<https://docs.aws.amazon.com/whitepapers/latest/aws-fault-isolation-boundaries/partitions.html>) in the AWS documentation for more information.
    #[serde(rename = "aws_partition")]
    pub aws_partition: crate::datadogV2::model::AWSAccountPartition,
    /// AWS Regions to collect data from. Defaults to `include_all`.
    #[serde(rename = "aws_regions")]
    pub aws_regions: Option<crate::datadogV2::model::AWSRegions>,
    /// AWS Logs Collection config.
    #[serde(rename = "logs_config")]
    pub logs_config: Option<crate::datadogV2::model::AWSLogsConfig>,
    /// AWS Metrics Collection config.
    #[serde(rename = "metrics_config")]
    pub metrics_config: Option<crate::datadogV2::model::AWSMetricsConfig>,
    /// AWS Resources Collection config.
    #[serde(rename = "resources_config")]
    pub resources_config: Option<crate::datadogV2::model::AWSResourcesConfig>,
    /// AWS Traces Collection config.
    #[serde(rename = "traces_config")]
    pub traces_config: Option<crate::datadogV2::model::AWSTracesConfig>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAccountCreateRequestAttributes {
    pub fn new(
        auth_config: crate::datadogV2::model::AWSAuthConfig,
        aws_account_id: String,
        aws_partition: crate::datadogV2::model::AWSAccountPartition,
    ) -> AWSAccountCreateRequestAttributes {
        AWSAccountCreateRequestAttributes {
            account_tags: None,
            auth_config,
            aws_account_id,
            aws_partition,
            aws_regions: None,
            logs_config: None,
            metrics_config: None,
            resources_config: None,
            traces_config: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_tags(mut self, value: Option<Vec<String>>) -> Self {
        self.account_tags = Some(value);
        self
    }

    pub fn aws_regions(mut self, value: crate::datadogV2::model::AWSRegions) -> Self {
        self.aws_regions = Some(value);
        self
    }

    pub fn logs_config(mut self, value: crate::datadogV2::model::AWSLogsConfig) -> Self {
        self.logs_config = Some(value);
        self
    }

    pub fn metrics_config(mut self, value: crate::datadogV2::model::AWSMetricsConfig) -> Self {
        self.metrics_config = Some(value);
        self
    }

    pub fn resources_config(mut self, value: crate::datadogV2::model::AWSResourcesConfig) -> Self {
        self.resources_config = Some(value);
        self
    }

    pub fn traces_config(mut self, value: crate::datadogV2::model::AWSTracesConfig) -> Self {
        self.traces_config = Some(value);
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

impl<'de> Deserialize<'de> for AWSAccountCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AWSAccountCreateRequestAttributesVisitor {
            type Value = AWSAccountCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Option<Vec<String>>> = None;
                let mut auth_config: Option<crate::datadogV2::model::AWSAuthConfig> = None;
                let mut aws_account_id: Option<String> = None;
                let mut aws_partition: Option<crate::datadogV2::model::AWSAccountPartition> = None;
                let mut aws_regions: Option<crate::datadogV2::model::AWSRegions> = None;
                let mut logs_config: Option<crate::datadogV2::model::AWSLogsConfig> = None;
                let mut metrics_config: Option<crate::datadogV2::model::AWSMetricsConfig> = None;
                let mut resources_config: Option<crate::datadogV2::model::AWSResourcesConfig> =
                    None;
                let mut traces_config: Option<crate::datadogV2::model::AWSTracesConfig> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_tags" => {
                            account_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auth_config" => {
                            auth_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_config) = auth_config {
                                match _auth_config {
                                    crate::datadogV2::model::AWSAuthConfig::UnparsedObject(
                                        _auth_config,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "aws_account_id" => {
                            aws_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_partition" => {
                            aws_partition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aws_partition) = aws_partition {
                                match _aws_partition {
                                    crate::datadogV2::model::AWSAccountPartition::UnparsedObject(_aws_partition) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "aws_regions" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_regions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aws_regions) = aws_regions {
                                match _aws_regions {
                                    crate::datadogV2::model::AWSRegions::UnparsedObject(
                                        _aws_regions,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                        "resources_config" => {
                            if v.is_null() {
                                continue;
                            }
                            resources_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traces_config" => {
                            if v.is_null() {
                                continue;
                            }
                            traces_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_config =
                    auth_config.ok_or_else(|| M::Error::missing_field("auth_config"))?;
                let aws_account_id =
                    aws_account_id.ok_or_else(|| M::Error::missing_field("aws_account_id"))?;
                let aws_partition =
                    aws_partition.ok_or_else(|| M::Error::missing_field("aws_partition"))?;

                let content = AWSAccountCreateRequestAttributes {
                    account_tags,
                    auth_config,
                    aws_account_id,
                    aws_partition,
                    aws_regions,
                    logs_config,
                    metrics_config,
                    resources_config,
                    traces_config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountCreateRequestAttributesVisitor)
    }
}
