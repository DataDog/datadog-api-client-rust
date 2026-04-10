// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filters for retention queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionFilters {
    /// Product Analytics/RUM audience filters.
    #[serde(rename = "audience_filters")]
    pub audience_filters: Option<crate::datadogV1::model::ProductAnalyticsAudienceFilters>,
    /// String filter.
    #[serde(rename = "string_filter")]
    pub string_filter: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionFilters {
    pub fn new() -> RetentionFilters {
        RetentionFilters {
            audience_filters: None,
            string_filter: None,
            _unparsed: false,
        }
    }

    pub fn audience_filters(
        mut self,
        value: crate::datadogV1::model::ProductAnalyticsAudienceFilters,
    ) -> Self {
        self.audience_filters = Some(value);
        self
    }

    pub fn string_filter(mut self, value: String) -> Self {
        self.string_filter = Some(value);
        self
    }
}

impl Default for RetentionFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RetentionFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionFiltersVisitor;
        impl<'a> Visitor<'a> for RetentionFiltersVisitor {
            type Value = RetentionFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<
                    crate::datadogV1::model::ProductAnalyticsAudienceFilters,
                > = None;
                let mut string_filter: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "string_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            string_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = RetentionFilters {
                    audience_filters,
                    string_filter,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionFiltersVisitor)
    }
}
