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
pub struct HeatgridSortAndOrderBy {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "nesting_display")]
    pub nesting_display: crate::datadogV1_20270101::model::HeatgridSortAndOrderByNestingDisplay,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort_by")]
    pub sort_by: crate::datadogV1_20270101::model::FlatSortBy,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HeatgridSortAndOrderBy {
    pub fn new(nesting_display: crate::datadogV1_20270101::model::HeatgridSortAndOrderByNestingDisplay, sort_by: crate::datadogV1_20270101::model::FlatSortBy) -> HeatgridSortAndOrderBy {
        HeatgridSortAndOrderBy {
            nesting_display,
            sort_by,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for HeatgridSortAndOrderBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridSortAndOrderByVisitor;
        impl<'a> Visitor<'a> for HeatgridSortAndOrderByVisitor {
            type Value = HeatgridSortAndOrderBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut nesting_display: Option<crate::datadogV1_20270101::model::HeatgridSortAndOrderByNestingDisplay> = None;
                let mut sort_by: Option<crate::datadogV1_20270101::model::FlatSortBy> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "nesting_display" => {
                            nesting_display = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _nesting_display) = nesting_display {
                                match _nesting_display {
                                    crate::datadogV1_20270101::model::HeatgridSortAndOrderByNestingDisplay::UnparsedObject(_nesting_display) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "sort_by" => {
                            sort_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort_by) = sort_by {
                                match _sort_by {
                                    crate::datadogV1_20270101::model::FlatSortBy::UnparsedObject(_sort_by) => {
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
                let nesting_display = nesting_display.ok_or_else(|| M::Error::missing_field("nesting_display"))?;
                let sort_by = sort_by.ok_or_else(|| M::Error::missing_field("sort_by"))?;

                let content = HeatgridSortAndOrderBy {
                    nesting_display,
                    sort_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridSortAndOrderByVisitor)
    }
}
