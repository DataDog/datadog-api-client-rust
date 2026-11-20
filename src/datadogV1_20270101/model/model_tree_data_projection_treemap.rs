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
pub struct TreeDataProjectionTreemap {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dimensions")]
    pub dimensions: Vec<crate::datadogV1_20270101::model::TreeDataProjectionTreemapDimensionsItems>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_styles")]
    pub group_styles: Option<Vec<crate::datadogV1_20270101::model::GroupStyleRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::TreeDataProjectionTreemapType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TreeDataProjectionTreemap {
    pub fn new(dimensions: Vec<crate::datadogV1_20270101::model::TreeDataProjectionTreemapDimensionsItems>, type_: crate::datadogV1_20270101::model::TreeDataProjectionTreemapType) -> TreeDataProjectionTreemap {
        TreeDataProjectionTreemap {
            dimensions,
            group_styles: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn group_styles(mut self, value: Vec<crate::datadogV1_20270101::model::GroupStyleRule>) -> Self {
        self.group_styles = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TreeDataProjectionTreemap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreeDataProjectionTreemapVisitor;
        impl<'a> Visitor<'a> for TreeDataProjectionTreemapVisitor {
            type Value = TreeDataProjectionTreemap;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dimensions: Option<Vec<crate::datadogV1_20270101::model::TreeDataProjectionTreemapDimensionsItems>> = None;
                let mut group_styles: Option<Vec<crate::datadogV1_20270101::model::GroupStyleRule>> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::TreeDataProjectionTreemapType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dimensions" => {
                            dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_styles" => {
                            if v.is_null() {
                                continue;
                            }
                            group_styles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::TreeDataProjectionTreemapType::UnparsedObject(_type_) => {
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

                let content = TreeDataProjectionTreemap {
                    dimensions,
                    group_styles,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TreeDataProjectionTreemapVisitor)
    }
}
