// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Teams response metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsResponseMeta {
    /// Teams response metadata.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::TeamsResponseMetaPagination>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsResponseMeta {
    pub fn new() -> TeamsResponseMeta {
        TeamsResponseMeta {
            pagination: None,
            _unparsed: false,
        }
    }

    pub fn pagination(
        &mut self,
        value: crate::datadogV2::model::TeamsResponseMetaPagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for TeamsResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamsResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsResponseMetaVisitor;
        impl<'a> Visitor<'a> for TeamsResponseMetaVisitor {
            type Value = TeamsResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pagination: Option<crate::datadogV2::model::TeamsResponseMetaPagination> =
                    None;
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

                let content = TeamsResponseMeta {
                    pagination,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsResponseMetaVisitor)
    }
}
