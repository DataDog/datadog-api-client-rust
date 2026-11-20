// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostmapTileDefSizeRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "_query_options")]
    pub _query_options: Option<std::collections::BTreeMap<String, Option<crate::datadogV1_20270101::model::DashboardInlineSchema42>>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "q")]
    pub q: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapTileDefSizeRequest {
    pub fn new(q: String) -> HostmapTileDefSizeRequest {
        HostmapTileDefSizeRequest {
            _query_options: None,
            q,
            _unparsed: false,
        }
    }
    
    pub fn _query_options(mut self, value: std::collections::BTreeMap<String, Option<crate::datadogV1_20270101::model::DashboardInlineSchema42>>) -> Self {
        self._query_options = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HostmapTileDefSizeRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapTileDefSizeRequestVisitor;
        impl<'a> Visitor<'a> for HostmapTileDefSizeRequestVisitor {
            type Value = HostmapTileDefSizeRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut _query_options: Option<std::collections::BTreeMap<String, Option<crate::datadogV1_20270101::model::DashboardInlineSchema42>>> = None;
                let mut q: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "_query_options" => {
                            if v.is_null() {
                                continue;
                            }
                            _query_options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "q" => {
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let q = q.ok_or_else(|| M::Error::missing_field("q"))?;

                let content = HostmapTileDefSizeRequest {
                    _query_options,
                    q,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostmapTileDefSizeRequestVisitor)
    }
}
