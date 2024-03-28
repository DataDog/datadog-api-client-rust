// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object for API data in an `OpenAPI` format as a file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAPIFile {
    /// Binary `OpenAPI` spec file
    #[serde(rename = "openapi_spec_file")]
    pub openapi_spec_file: Option<Vec<u8>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAPIFile {
    pub fn new() -> OpenAPIFile {
        OpenAPIFile {
            openapi_spec_file: None,
            _unparsed: false,
        }
    }

    pub fn openapi_spec_file(mut self, value: Vec<u8>) -> Self {
        self.openapi_spec_file = Some(value);
        self
    }
}

impl Default for OpenAPIFile {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpenAPIFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAPIFileVisitor;
        impl<'a> Visitor<'a> for OpenAPIFileVisitor {
            type Value = OpenAPIFile;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openapi_spec_file: Option<Vec<u8>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "openapi_spec_file" => {
                            if v.is_null() {
                                continue;
                            }
                            openapi_spec_file =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OpenAPIFile {
                    openapi_spec_file,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAPIFileVisitor)
    }
}
