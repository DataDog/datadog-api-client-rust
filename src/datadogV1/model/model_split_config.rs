// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Encapsulates all user choices about how to split a graph.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SplitConfig {
    /// Maximum number of graphs to display in the widget.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Controls the order in which graphs appear in the split.
    #[serde(rename = "sort")]
    pub sort: crate::datadogV1::model::SplitSort,
    /// The dimension(s) on which to split the graph
    #[serde(rename = "split_dimensions")]
    pub split_dimensions: Vec<crate::datadogV1::model::SplitDimension>,
    /// Manual selection of tags making split graph widget static
    #[serde(rename = "static_splits")]
    pub static_splits: Option<Vec<Vec<crate::datadogV1::model::SplitVectorEntryItem>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SplitConfig {
    pub fn new(
        limit: i64,
        sort: crate::datadogV1::model::SplitSort,
        split_dimensions: Vec<crate::datadogV1::model::SplitDimension>,
    ) -> SplitConfig {
        SplitConfig {
            limit,
            sort,
            split_dimensions,
            static_splits: None,
            _unparsed: false,
        }
    }

    pub fn static_splits(
        &mut self,
        value: Vec<Vec<crate::datadogV1::model::SplitVectorEntryItem>>,
    ) -> &mut Self {
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
                let mut limit: Option<i64> = None;
                let mut sort: Option<crate::datadogV1::model::SplitSort> = None;
                let mut split_dimensions: Option<Vec<crate::datadogV1::model::SplitDimension>> =
                    None;
                let mut static_splits: Option<
                    Vec<Vec<crate::datadogV1::model::SplitVectorEntryItem>>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "split_dimensions" => {
                            split_dimensions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "static_splits" => {
                            if v.is_null() {
                                continue;
                            }
                            static_splits =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let sort = sort.ok_or_else(|| M::Error::missing_field("sort"))?;
                let split_dimensions =
                    split_dimensions.ok_or_else(|| M::Error::missing_field("split_dimensions"))?;

                let content = SplitConfig {
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
