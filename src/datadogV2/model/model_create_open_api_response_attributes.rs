// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for `CreateOpenAPI`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateOpenAPIResponseAttributes {
    /// List of endpoints which couldn't be parsed.
    #[serde(rename = "failed_endpoints")]
    pub failed_endpoints: Option<Vec<crate::datadogV2::model::OpenAPIEndpoint>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateOpenAPIResponseAttributes {
    pub fn new() -> CreateOpenAPIResponseAttributes {
        CreateOpenAPIResponseAttributes {
            failed_endpoints: None,
            _unparsed: false,
        }
    }

    pub fn failed_endpoints(
        mut self,
        value: Vec<crate::datadogV2::model::OpenAPIEndpoint>,
    ) -> Self {
        self.failed_endpoints = Some(value);
        self
    }
}

impl Default for CreateOpenAPIResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateOpenAPIResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateOpenAPIResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CreateOpenAPIResponseAttributesVisitor {
            type Value = CreateOpenAPIResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failed_endpoints: Option<Vec<crate::datadogV2::model::OpenAPIEndpoint>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failed_endpoints" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_endpoints =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CreateOpenAPIResponseAttributes {
                    failed_endpoints,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateOpenAPIResponseAttributesVisitor)
    }
}
