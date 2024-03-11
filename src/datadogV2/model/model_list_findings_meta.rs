// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListFindingsMeta {
    /// Pagination and findings count information.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::ListFindingsPage>,
    /// The point in time corresponding to the listed findings.
    #[serde(rename = "snapshot_timestamp")]
    pub snapshot_timestamp: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListFindingsMeta {
    pub fn new() -> ListFindingsMeta {
        ListFindingsMeta {
            page: None,
            snapshot_timestamp: None,
            _unparsed: false,
        }
    }

    pub fn page(&mut self, value: crate::datadogV2::model::ListFindingsPage) -> &mut Self {
        self.page = Some(value);
        self
    }

    pub fn snapshot_timestamp(&mut self, value: i64) -> &mut Self {
        self.snapshot_timestamp = Some(value);
        self
    }
}

impl Default for ListFindingsMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListFindingsMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListFindingsMetaVisitor;
        impl<'a> Visitor<'a> for ListFindingsMetaVisitor {
            type Value = ListFindingsMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<crate::datadogV2::model::ListFindingsPage> = None;
                let mut snapshot_timestamp: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ListFindingsMeta {
                    page,
                    snapshot_timestamp,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListFindingsMetaVisitor)
    }
}
