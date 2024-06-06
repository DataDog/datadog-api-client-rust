// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Account attributes for creation
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccountCreateAttributes {
    /// Tags to apply to all metrics in the account
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// AWS Authentication config
    #[serde(rename = "auth_config")]
    pub auth_config: Option<crate::datadogV2::model::AWSAuthConfig>,
    /// AWS Account ID
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: String,
    /// AWS Regions to collect data from
    #[serde(rename = "aws_regions")]
    pub aws_regions: Option<crate::datadogV2::model::AWSRegionsList>,
    /// AWS Logs config
    #[serde(rename = "logs_config")]
    pub logs_config: Option<crate::datadogV2::model::AWSLogs>,
    /// AWS Metrics config
    #[serde(rename = "metrics_config")]
    pub metrics_config: Option<crate::datadogV2::model::AWSMetrics>,
    /// AWS Resources config
    #[serde(rename = "resources_config")]
    pub resources_config: Option<crate::datadogV2::model::AWSResources>,
    /// AWS Traces config
    #[serde(rename = "traces_config")]
    pub traces_config: Option<crate::datadogV2::model::AWSTraces>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAccountCreateAttributes {
    pub fn new(aws_account_id: String) -> AWSAccountCreateAttributes {
        AWSAccountCreateAttributes {
            account_tags: None,
            auth_config: None,
            aws_account_id,
            aws_regions: None,
            logs_config: None,
            metrics_config: None,
            resources_config: None,
            traces_config: None,
            _unparsed: false,
        }
    }

    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }

    pub fn auth_config(mut self, value: crate::datadogV2::model::AWSAuthConfig) -> Self {
        self.auth_config = Some(value);
        self
    }

    pub fn aws_regions(mut self, value: crate::datadogV2::model::AWSRegionsList) -> Self {
        self.aws_regions = Some(value);
        self
    }

    pub fn logs_config(mut self, value: crate::datadogV2::model::AWSLogs) -> Self {
        self.logs_config = Some(value);
        self
    }

    pub fn metrics_config(mut self, value: crate::datadogV2::model::AWSMetrics) -> Self {
        self.metrics_config = Some(value);
        self
    }

    pub fn resources_config(mut self, value: crate::datadogV2::model::AWSResources) -> Self {
        self.resources_config = Some(value);
        self
    }

    pub fn traces_config(mut self, value: crate::datadogV2::model::AWSTraces) -> Self {
        self.traces_config = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AWSAccountCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AWSAccountCreateAttributesVisitor {
            type Value = AWSAccountCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Vec<String>> = None;
                let mut auth_config: Option<crate::datadogV2::model::AWSAuthConfig> = None;
                let mut aws_account_id: Option<String> = None;
                let mut aws_regions: Option<crate::datadogV2::model::AWSRegionsList> = None;
                let mut logs_config: Option<crate::datadogV2::model::AWSLogs> = None;
                let mut metrics_config: Option<crate::datadogV2::model::AWSMetrics> = None;
                let mut resources_config: Option<crate::datadogV2::model::AWSResources> = None;
                let mut traces_config: Option<crate::datadogV2::model::AWSTraces> = None;
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
                        "auth_config" => {
                            if v.is_null() {
                                continue;
                            }
                            auth_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_account_id" => {
                            aws_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_regions" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_regions =
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
                        &_ => {}
                    }
                }
                let aws_account_id =
                    aws_account_id.ok_or_else(|| M::Error::missing_field("aws_account_id"))?;

                let content = AWSAccountCreateAttributes {
                    account_tags,
                    auth_config,
                    aws_account_id,
                    aws_regions,
                    logs_config,
                    metrics_config,
                    resources_config,
                    traces_config,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountCreateAttributesVisitor)
    }
}
