// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response metadata object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImageMeta {
    /// Paging attributes.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::ContainerImageMetaPage>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImageMeta {
    pub fn new() -> ContainerImageMeta {
        ContainerImageMeta {
            pagination: None,
            _unparsed: false,
        }
    }

    pub fn pagination(mut self, value: crate::datadogV2::model::ContainerImageMetaPage) -> Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for ContainerImageMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageMetaVisitor;
        impl<'a> Visitor<'a> for ContainerImageMetaVisitor {
            type Value = ContainerImageMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pagination: Option<crate::datadogV2::model::ContainerImageMetaPage> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pagination" => {
                            if v.is_null() {
                                continue;
                            }
                            pagination = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImageMeta {
                    pagination,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageMetaVisitor)
    }
}
