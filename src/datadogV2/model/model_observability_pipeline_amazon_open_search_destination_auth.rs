// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authentication settings for the Amazon OpenSearch destination.
/// The `strategy` field determines whether basic or AWS-based authentication is used.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAmazonOpenSearchDestinationAuth {
    /// The ARN of the role to assume (used with `aws` strategy).
    #[serde(rename = "assume_role")]
    pub assume_role: Option<String>,
    /// AWS region
    #[serde(rename = "aws_region")]
    pub aws_region: Option<String>,
    /// External ID for the assumed role (used with `aws` strategy).
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Session name for the assumed role (used with `aws` strategy).
    #[serde(rename = "session_name")]
    pub session_name: Option<String>,
    /// The authentication strategy to use.
    #[serde(rename = "strategy")]
    pub strategy:
        crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAmazonOpenSearchDestinationAuth {
    pub fn new(
        strategy: crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy,
    ) -> ObservabilityPipelineAmazonOpenSearchDestinationAuth {
        ObservabilityPipelineAmazonOpenSearchDestinationAuth {
            assume_role: None,
            aws_region: None,
            external_id: None,
            session_name: None,
            strategy,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assume_role(mut self, value: String) -> Self {
        self.assume_role = Some(value);
        self
    }

    pub fn aws_region(mut self, value: String) -> Self {
        self.aws_region = Some(value);
        self
    }

    pub fn external_id(mut self, value: String) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn session_name(mut self, value: String) -> Self {
        self.session_name = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonOpenSearchDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAmazonOpenSearchDestinationAuthVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAmazonOpenSearchDestinationAuthVisitor {
            type Value = ObservabilityPipelineAmazonOpenSearchDestinationAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assume_role: Option<String> = None;
                let mut aws_region: Option<String> = None;
                let mut external_id: Option<String> = None;
                let mut session_name: Option<String> = None;
                let mut strategy: Option<crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assume_role" => {
                            if v.is_null() {
                                continue;
                            }
                            assume_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_region" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_id" => {
                            if v.is_null() {
                                continue;
                            }
                            external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_name" => {
                            if v.is_null() {
                                continue;
                            }
                            session_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy::UnparsedObject(_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = ObservabilityPipelineAmazonOpenSearchDestinationAuth {
                    assume_role,
                    aws_region,
                    external_id,
                    session_name,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAmazonOpenSearchDestinationAuthVisitor)
    }
}
