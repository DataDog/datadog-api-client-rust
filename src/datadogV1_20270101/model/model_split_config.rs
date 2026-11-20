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
pub struct SplitConfig {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "calendar_shift")]
    pub calendar_shift: Option<crate::datadogV1_20270101::model::CalendarShiftSplitConfig>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: crate::datadogV1_20270101::model::SplitConfigSort,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "split_dimensions")]
    pub split_dimensions: Vec<crate::datadogV1_20270101::model::SplitDimension>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "static_splits")]
    pub static_splits: Option<Vec<Vec<crate::datadogV1_20270101::model::SplitVectorTagFilter>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SplitConfig {
    pub fn new(limit: f64, sort: crate::datadogV1_20270101::model::SplitConfigSort, split_dimensions: Vec<crate::datadogV1_20270101::model::SplitDimension>) -> SplitConfig {
        SplitConfig {
            calendar_shift: None,
            limit,
            sort,
            split_dimensions,
            static_splits: None,
            _unparsed: false,
        }
    }
    
    pub fn calendar_shift(mut self, value: crate::datadogV1_20270101::model::CalendarShiftSplitConfig) -> Self {
        self.calendar_shift = Some(value);
        self
    }
    
    pub fn static_splits(mut self, value: Vec<Vec<crate::datadogV1_20270101::model::SplitVectorTagFilter>>) -> Self {
        self.static_splits = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for SplitConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitConfigVisitor;
        impl<'a> Visitor<'a> for SplitConfigVisitor {
            type Value = SplitConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calendar_shift: Option<crate::datadogV1_20270101::model::CalendarShiftSplitConfig> = None;
                let mut limit: Option<f64> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::SplitConfigSort> = None;
                let mut split_dimensions: Option<Vec<crate::datadogV1_20270101::model::SplitDimension>> = None;
                let mut static_splits: Option<Vec<Vec<crate::datadogV1_20270101::model::SplitVectorTagFilter>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "calendar_shift" => {
                            if v.is_null() {
                                continue;
                            }
                            calendar_shift = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "split_dimensions" => {
                            split_dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "static_splits" => {
                            if v.is_null() {
                                continue;
                            }
                            static_splits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let sort = sort.ok_or_else(|| M::Error::missing_field("sort"))?;
                let split_dimensions = split_dimensions.ok_or_else(|| M::Error::missing_field("split_dimensions"))?;

                let content = SplitConfig {
                    calendar_shift,
                    limit,
                    sort,
                    split_dimensions,
                    static_splits,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitConfigVisitor)
    }
}
