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
pub struct HostmapProjection {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dimensions")]
    pub dimensions: Vec<crate::datadogV1_20270101::model::HostmapProjectionDimensionsItems>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::HostmapProjectionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapProjection {
    pub fn new(dimensions: Vec<crate::datadogV1_20270101::model::HostmapProjectionDimensionsItems>, type_: crate::datadogV1_20270101::model::HostmapProjectionType) -> HostmapProjection {
        HostmapProjection {
            dimensions,
            type_,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for HostmapProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapProjectionVisitor;
        impl<'a> Visitor<'a> for HostmapProjectionVisitor {
            type Value = HostmapProjection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dimensions: Option<Vec<crate::datadogV1_20270101::model::HostmapProjectionDimensionsItems>> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::HostmapProjectionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dimensions" => {
                            dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::HostmapProjectionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let dimensions = dimensions.ok_or_else(|| M::Error::missing_field("dimensions"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = HostmapProjection {
                    dimensions,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostmapProjectionVisitor)
    }
}
