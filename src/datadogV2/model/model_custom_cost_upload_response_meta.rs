// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Meta for the response from the Upload Custom Costs endpoints.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostUploadResponseMeta {
    /// Version of Custom Costs file
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostUploadResponseMeta {
    pub fn new() -> CustomCostUploadResponseMeta {
        CustomCostUploadResponseMeta {
            version: None,
            _unparsed: false,
        }
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for CustomCostUploadResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostUploadResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostUploadResponseMetaVisitor;
        impl<'a> Visitor<'a> for CustomCostUploadResponseMetaVisitor {
            type Value = CustomCostUploadResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut version: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CustomCostUploadResponseMeta { version, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostUploadResponseMetaVisitor)
    }
}
