// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Logs config
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogs {
    /// AWS Lambda forwarder
    #[serde(rename = "lambda_forwarder")]
    pub lambda_forwarder: Option<crate::datadogV2::model::AWSLambdaForwarder>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogs {
    pub fn new() -> AWSLogs {
        AWSLogs {
            lambda_forwarder: None,
            _unparsed: false,
        }
    }

    pub fn lambda_forwarder(mut self, value: crate::datadogV2::model::AWSLambdaForwarder) -> Self {
        self.lambda_forwarder = Some(value);
        self
    }
}

impl Default for AWSLogs {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLogs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsVisitor;
        impl<'a> Visitor<'a> for AWSLogsVisitor {
            type Value = AWSLogs;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut lambda_forwarder: Option<crate::datadogV2::model::AWSLambdaForwarder> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "lambda_forwarder" => {
                            if v.is_null() {
                                continue;
                            }
                            lambda_forwarder =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSLogs {
                    lambda_forwarder,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsVisitor)
    }
}
