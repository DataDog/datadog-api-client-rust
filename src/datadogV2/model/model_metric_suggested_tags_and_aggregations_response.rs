// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object that includes a single metric's actively queried tags and aggregations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricSuggestedTagsAndAggregationsResponse {
    /// Object for a single metric's actively queried tags and aggregations.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::MetricSuggestedTagsAndAggregations>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricSuggestedTagsAndAggregationsResponse {
    pub fn new() -> MetricSuggestedTagsAndAggregationsResponse {
        MetricSuggestedTagsAndAggregationsResponse {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: crate::datadogV2::model::MetricSuggestedTagsAndAggregations,
    ) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for MetricSuggestedTagsAndAggregationsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricSuggestedTagsAndAggregationsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricSuggestedTagsAndAggregationsResponseVisitor;
        impl<'a> Visitor<'a> for MetricSuggestedTagsAndAggregationsResponseVisitor {
            type Value = MetricSuggestedTagsAndAggregationsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::MetricSuggestedTagsAndAggregations> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricSuggestedTagsAndAggregationsResponse { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricSuggestedTagsAndAggregationsResponseVisitor)
    }
}
