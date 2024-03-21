// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Description of the Lambdas.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogsLambda {
    /// Available ARN IDs.
    #[serde(rename = "arn")]
    pub arn: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogsLambda {
    pub fn new() -> AWSLogsLambda {
        AWSLogsLambda {
            arn: None,
            _unparsed: false,
        }
    }

    pub fn arn(mut self, value: String) -> Self {
        self.arn = Some(value);
        self
    }
}

impl Default for AWSLogsLambda {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLogsLambda {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsLambdaVisitor;
        impl<'a> Visitor<'a> for AWSLogsLambdaVisitor {
            type Value = AWSLogsLambda;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arn: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arn" => {
                            if v.is_null() {
                                continue;
                            }
                            arn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSLogsLambda { arn, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsLambdaVisitor)
    }
}
