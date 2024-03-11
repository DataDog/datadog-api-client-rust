// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A column containing the tag keys and values in a group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GroupScalarColumn {
    /// The name of the tag key or group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The type of column present for groups.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ScalarColumnTypeGroup>,
    /// The array of tag values for each group found for the results of the formulas or queries.
    #[serde(rename = "values")]
    pub values: Option<Vec<Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GroupScalarColumn {
    pub fn new() -> GroupScalarColumn {
        GroupScalarColumn {
            name: None,
            type_: None,
            values: None,
            _unparsed: false,
        }
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::ScalarColumnTypeGroup) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn values(&mut self, value: Vec<Vec<String>>) -> &mut Self {
        self.values = Some(value);
        self
    }
}

impl Default for GroupScalarColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GroupScalarColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GroupScalarColumnVisitor;
        impl<'a> Visitor<'a> for GroupScalarColumnVisitor {
            type Value = GroupScalarColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ScalarColumnTypeGroup> = None;
                let mut values: Option<Vec<Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ScalarColumnTypeGroup::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = GroupScalarColumn {
                    name,
                    type_,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GroupScalarColumnVisitor)
    }
}
