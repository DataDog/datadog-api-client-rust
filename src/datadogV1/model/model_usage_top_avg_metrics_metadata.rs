// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object containing document metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageTopAvgMetricsMetadata {
    /// The day value from the user request that contains the returned usage data. (If day was used the request)
    #[serde(rename = "day")]
    pub day: Option<chrono::DateTime<chrono::Utc>>,
    /// The month value from the user request that contains the returned usage data. (If month was used the request)
    #[serde(rename = "month")]
    pub month: Option<chrono::DateTime<chrono::Utc>>,
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV1::model::UsageTopAvgMetricsPagination>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageTopAvgMetricsMetadata {
    pub fn new() -> UsageTopAvgMetricsMetadata {
        UsageTopAvgMetricsMetadata {
            day: None,
            month: None,
            pagination: None,
            _unparsed: false,
        }
    }

    pub fn day(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.day = Some(value);
        self
    }

    pub fn month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.month = Some(value);
        self
    }

    pub fn pagination(
        mut self,
        value: crate::datadogV1::model::UsageTopAvgMetricsPagination,
    ) -> Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for UsageTopAvgMetricsMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageTopAvgMetricsMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageTopAvgMetricsMetadataVisitor;
        impl<'a> Visitor<'a> for UsageTopAvgMetricsMetadataVisitor {
            type Value = UsageTopAvgMetricsMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut day: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut month: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut pagination: Option<crate::datadogV1::model::UsageTopAvgMetricsPagination> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "day" => {
                            if v.is_null() {
                                continue;
                            }
                            day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "month" => {
                            if v.is_null() {
                                continue;
                            }
                            month = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pagination" => {
                            if v.is_null() {
                                continue;
                            }
                            pagination = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageTopAvgMetricsMetadata {
                    day,
                    month,
                    pagination,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageTopAvgMetricsMetadataVisitor)
    }
}
