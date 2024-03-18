// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object containing document metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonthlyUsageAttributionMetadata {
    /// An array of available aggregates.
    #[serde(rename = "aggregates")]
    pub aggregates: Option<Vec<crate::datadogV1::model::UsageAttributionAggregatesBody>>,
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV1::model::MonthlyUsageAttributionPagination>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonthlyUsageAttributionMetadata {
    pub fn new() -> MonthlyUsageAttributionMetadata {
        MonthlyUsageAttributionMetadata {
            aggregates: None,
            pagination: None,
            _unparsed: false,
        }
    }

    pub fn aggregates(
        mut self,
        value: Vec<crate::datadogV1::model::UsageAttributionAggregatesBody>,
    ) -> Self {
        self.aggregates = Some(value);
        self
    }

    pub fn pagination(
        mut self,
        value: crate::datadogV1::model::MonthlyUsageAttributionPagination,
    ) -> Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for MonthlyUsageAttributionMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonthlyUsageAttributionMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonthlyUsageAttributionMetadataVisitor;
        impl<'a> Visitor<'a> for MonthlyUsageAttributionMetadataVisitor {
            type Value = MonthlyUsageAttributionMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregates: Option<
                    Vec<crate::datadogV1::model::UsageAttributionAggregatesBody>,
                > = None;
                let mut pagination: Option<
                    crate::datadogV1::model::MonthlyUsageAttributionPagination,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregates" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = MonthlyUsageAttributionMetadata {
                    aggregates,
                    pagination,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonthlyUsageAttributionMetadataVisitor)
    }
}
