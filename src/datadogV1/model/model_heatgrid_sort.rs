// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Ordering of the heatgrid rows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridSort {
    /// Display groups as flat rows.
    #[serde(rename = "nesting_display")]
    pub nesting_display: crate::datadogV1::model::HeatgridNestingDisplay,
    /// Sort rows by aggregated value or group label.
    #[serde(rename = "sort_by")]
    pub sort_by: crate::datadogV1::model::HeatgridSortBy,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridSort {
    pub fn new(
        nesting_display: crate::datadogV1::model::HeatgridNestingDisplay,
        sort_by: crate::datadogV1::model::HeatgridSortBy,
    ) -> HeatgridSort {
        HeatgridSort {
            nesting_display,
            sort_by,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridSortVisitor;
        impl<'a> Visitor<'a> for HeatgridSortVisitor {
            type Value = HeatgridSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut nesting_display: Option<crate::datadogV1::model::HeatgridNestingDisplay> =
                    None;
                let mut sort_by: Option<crate::datadogV1::model::HeatgridSortBy> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "nesting_display" => {
                            nesting_display =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _nesting_display) = nesting_display {
                                match _nesting_display {
                                    crate::datadogV1::model::HeatgridNestingDisplay::UnparsedObject(_nesting_display) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sort_by" => {
                            sort_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort_by) = sort_by {
                                match _sort_by {
                                    crate::datadogV1::model::HeatgridSortBy::UnparsedObject(
                                        _sort_by,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let nesting_display =
                    nesting_display.ok_or_else(|| M::Error::missing_field("nesting_display"))?;
                let sort_by = sort_by.ok_or_else(|| M::Error::missing_field("sort_by"))?;

                let content = HeatgridSort {
                    nesting_display,
                    sort_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridSortVisitor)
    }
}
