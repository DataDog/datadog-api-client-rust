// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

///
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesResponseSeries {
    /// List of tags that apply to a single response value.
    #[serde(rename = "group_tags")]
    pub group_tags: Option<Vec<String>>,
    /// The index of the query in the "formulas" array (or "queries" array if no "formulas" was specified).
    #[serde(rename = "query_index")]
    pub query_index: Option<i32>,
    /// Detailed information about the unit.
    /// The first element describes the "primary unit" (for example, `bytes` in `bytes per second`).
    /// The second element describes the "per unit" (for example, `second` in `bytes per second`).
    /// If the second element is not present, the API returns null.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit: Option<Option<Vec<Option<crate::datadogV2::model::Unit>>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesResponseSeries {
    pub fn new() -> TimeseriesResponseSeries {
        TimeseriesResponseSeries {
            group_tags: None,
            query_index: None,
            unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_tags(mut self, value: Vec<String>) -> Self {
        self.group_tags = Some(value);
        self
    }

    pub fn query_index(mut self, value: i32) -> Self {
        self.query_index = Some(value);
        self
    }

    pub fn unit(mut self, value: Option<Vec<Option<crate::datadogV2::model::Unit>>>) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for TimeseriesResponseSeries {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimeseriesResponseSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesResponseSeriesVisitor;
        impl<'a> Visitor<'a> for TimeseriesResponseSeriesVisitor {
            type Value = TimeseriesResponseSeries;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_tags: Option<Vec<String>> = None;
                let mut query_index: Option<i32> = None;
                let mut unit: Option<Option<Vec<Option<crate::datadogV2::model::Unit>>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            group_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_index" => {
                            if v.is_null() {
                                continue;
                            }
                            query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TimeseriesResponseSeries {
                    group_tags,
                    query_index,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesResponseSeriesVisitor)
    }
}
