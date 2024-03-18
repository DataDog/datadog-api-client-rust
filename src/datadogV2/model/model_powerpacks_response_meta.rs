// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack response metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpacksResponseMeta {
    /// Powerpack response pagination metadata.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::PowerpacksResponseMetaPagination>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpacksResponseMeta {
    pub fn new() -> PowerpacksResponseMeta {
        PowerpacksResponseMeta {
            pagination: None,
            _unparsed: false,
        }
    }

    pub fn pagination(
        mut self,
        value: crate::datadogV2::model::PowerpacksResponseMetaPagination,
    ) -> Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for PowerpacksResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PowerpacksResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpacksResponseMetaVisitor;
        impl<'a> Visitor<'a> for PowerpacksResponseMetaVisitor {
            type Value = PowerpacksResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pagination: Option<
                    crate::datadogV2::model::PowerpacksResponseMetaPagination,
                > = None;
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

                let content = PowerpacksResponseMeta {
                    pagination,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpacksResponseMetaVisitor)
    }
}
