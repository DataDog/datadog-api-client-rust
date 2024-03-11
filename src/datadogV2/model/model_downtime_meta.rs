// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeMeta {
    /// Object containing the total filtered count.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::DowntimeMetaPage>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeMeta {
    pub fn new() -> DowntimeMeta {
        DowntimeMeta {
            page: None,
            _unparsed: false,
        }
    }

    pub fn page(&mut self, value: crate::datadogV2::model::DowntimeMetaPage) -> &mut Self {
        self.page = Some(value);
        self
    }
}

impl Default for DowntimeMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeMetaVisitor;
        impl<'a> Visitor<'a> for DowntimeMetaVisitor {
            type Value = DowntimeMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<crate::datadogV2::model::DowntimeMetaPage> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeMeta { page, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeMetaVisitor)
    }
}
