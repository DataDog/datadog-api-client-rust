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
pub struct GeomapView20270101OneOf4457650336 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_extent")]
    pub custom_extent: Vec<serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapView20270101OneOf4457650336 {
    pub fn new(custom_extent: Vec<serde_json::Value>) -> GeomapView20270101OneOf4457650336 {
        GeomapView20270101OneOf4457650336 {
            custom_extent,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for GeomapView20270101OneOf4457650336 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapView20270101OneOf4457650336Visitor;
        impl<'a> Visitor<'a> for GeomapView20270101OneOf4457650336Visitor {
            type Value = GeomapView20270101OneOf4457650336;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_extent: Option<Vec<serde_json::Value>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_extent" => {
                            custom_extent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let custom_extent = custom_extent.ok_or_else(|| M::Error::missing_field("custom_extent"))?;

                let content = GeomapView20270101OneOf4457650336 {
                    custom_extent,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapView20270101OneOf4457650336Visitor)
    }
}
