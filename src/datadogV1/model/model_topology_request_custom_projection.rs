// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mapping from result columns to node fields. Each result row feeds one node.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyRequestCustomProjection {
    /// List of mappings from a result column to a node field. Each `dimension` must be unique.
    #[serde(rename = "dimensions")]
    pub dimensions: Vec<crate::datadogV1::model::TopologyRequestCustomProjectionDimension>,
    /// Projection type. Each result row is a node.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TopologyRequestCustomProjectionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyRequestCustomProjection {
    pub fn new(
        dimensions: Vec<crate::datadogV1::model::TopologyRequestCustomProjectionDimension>,
        type_: crate::datadogV1::model::TopologyRequestCustomProjectionType,
    ) -> TopologyRequestCustomProjection {
        TopologyRequestCustomProjection {
            dimensions,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for TopologyRequestCustomProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyRequestCustomProjectionVisitor;
        impl<'a> Visitor<'a> for TopologyRequestCustomProjectionVisitor {
            type Value = TopologyRequestCustomProjection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dimensions: Option<
                    Vec<crate::datadogV1::model::TopologyRequestCustomProjectionDimension>,
                > = None;
                let mut type_: Option<
                    crate::datadogV1::model::TopologyRequestCustomProjectionType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dimensions" => {
                            dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::TopologyRequestCustomProjectionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let dimensions = dimensions.ok_or_else(|| M::Error::missing_field("dimensions"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TopologyRequestCustomProjection {
                    dimensions,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyRequestCustomProjectionVisitor)
    }
}
