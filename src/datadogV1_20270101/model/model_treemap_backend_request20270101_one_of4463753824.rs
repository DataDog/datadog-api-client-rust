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
pub struct TreemapBackendRequest20270101OneOf4463753824 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "q")]
    pub q: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::TreemapRequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TreemapBackendRequest20270101OneOf4463753824 {
    pub fn new(q: String) -> TreemapBackendRequest20270101OneOf4463753824 {
        TreemapBackendRequest20270101OneOf4463753824 {
            q,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::TreemapRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TreemapBackendRequest20270101OneOf4463753824 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreemapBackendRequest20270101OneOf4463753824Visitor;
        impl<'a> Visitor<'a> for TreemapBackendRequest20270101OneOf4463753824Visitor {
            type Value = TreemapBackendRequest20270101OneOf4463753824;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut q: Option<String> = None;
                let mut style: Option<crate::datadogV1_20270101::model::TreemapRequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "q" => {
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let q = q.ok_or_else(|| M::Error::missing_field("q"))?;

                let content = TreemapBackendRequest20270101OneOf4463753824 {
                    q,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TreemapBackendRequest20270101OneOf4463753824Visitor)
    }
}
