// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS account ID and Lambda ARN.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAccountAndLambdaRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// ARN of the Datadog Lambda created during the Datadog-Amazon Web services Log collection setup.
    #[serde(rename = "lambda_arn")]
    pub lambda_arn: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAccountAndLambdaRequest {
    pub fn new(account_id: String, lambda_arn: String) -> AWSAccountAndLambdaRequest {
        AWSAccountAndLambdaRequest {
            account_id,
            lambda_arn,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AWSAccountAndLambdaRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAccountAndLambdaRequestVisitor;
        impl<'a> Visitor<'a> for AWSAccountAndLambdaRequestVisitor {
            type Value = AWSAccountAndLambdaRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut lambda_arn: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lambda_arn" => {
                            lambda_arn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let lambda_arn = lambda_arn.ok_or_else(|| M::Error::missing_field("lambda_arn"))?;

                let content = AWSAccountAndLambdaRequest {
                    account_id,
                    lambda_arn,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAccountAndLambdaRequestVisitor)
    }
}
