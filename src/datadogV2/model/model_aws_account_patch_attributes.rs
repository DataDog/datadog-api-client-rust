// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Account attributes for patching
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccountPatchAttributes {
    /// Tags to apply to all metrics in the account
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// AWS Authentication config
    #[serde(rename = "auth_config")]
    pub auth_config: Option<crate::datadogV2::model::AWSAuthConfig>,
    /// AWS Account ID
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: Option<String>,
    /// AWS Account name
    #[serde(rename = "aws_account_name")]
    pub aws_account_name: Option<String>,
    /// AWS Regions to collect data from
    #[serde(rename = "aws_regions")]
    pub aws_regions: Option<crate::datadogV2::model::AWSRegionsList>,
    /// Creation date of the account
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// AWS Logs config
    #[serde(rename = "logs_config")]
    pub logs_config: Option<crate::datadogV2::model::AWSLogs>,
    /// AWS Metrics config
    #[serde(rename = "metrics_config")]
    pub metrics_config: Option<crate::datadogV2::model::AWSMetrics>,
    /// Last modification date of the account
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
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

impl AWSAccountPatchAttributes {
    pub fn new() -> AWSAccountPatchAttributes {
        AWSAccountPatchAttributes {
            account_tags: None,
            auth_config: None,
            aws_account_id: None,
            aws_account_name: None,
            aws_regions: None,
            created_at: None,
            logs_config: None,
            metrics_config: None,
            modified_at: None,
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

    pub fn aws_account_id(mut self, value: String) -> Self {
        self.aws_account_id = Some(value);
        self
    }

    pub fn aws_account_name(mut self, value: String) -> Self {
        self.aws_account_name = Some(value);
        self
    }

    pub fn aws_regions(mut self, value: crate::datadogV2::model::AWSRegionsList) -> Self {
        self.aws_regions = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
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

    pub fn modified_at(mut self, value: String) -> Self {
        self.modified_at = Some(value);
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

impl Default for AWSAccountPatchAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSAccountPatchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountPatchAttributesVisitor;
        impl<'a> Visitor<'a> for AWSAccountPatchAttributesVisitor {
            type Value = AWSAccountPatchAttributes;

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
                let mut aws_account_name: Option<String> = None;
                let mut aws_regions: Option<crate::datadogV2::model::AWSRegionsList> = None;
                let mut created_at: Option<String> = None;
                let mut logs_config: Option<crate::datadogV2::model::AWSLogs> = None;
                let mut metrics_config: Option<crate::datadogV2::model::AWSMetrics> = None;
                let mut modified_at: Option<String> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            aws_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_account_name" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_account_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_regions" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_regions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
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

                let content = AWSAccountPatchAttributes {
                    account_tags,
                    auth_config,
                    aws_account_id,
                    aws_account_name,
                    aws_regions,
                    created_at,
                    logs_config,
                    metrics_config,
                    modified_at,
                    resources_config,
                    traces_config,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountPatchAttributesVisitor)
    }
}
