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
pub struct HourlyUsageAttributionMetadata {
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV1::model::HourlyUsageAttributionPagination>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HourlyUsageAttributionMetadata {
    pub fn new() -> HourlyUsageAttributionMetadata {
        HourlyUsageAttributionMetadata {
            pagination: None,
            _unparsed: false,
        }
    }

    pub fn pagination(
        mut self,
        value: crate::datadogV1::model::HourlyUsageAttributionPagination,
    ) -> Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for HourlyUsageAttributionMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HourlyUsageAttributionMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HourlyUsageAttributionMetadataVisitor;
        impl<'a> Visitor<'a> for HourlyUsageAttributionMetadataVisitor {
            type Value = HourlyUsageAttributionMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pagination: Option<
                    crate::datadogV1::model::HourlyUsageAttributionPagination,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pagination" => {
                            if v.is_null() {
                                continue;
                            }
                            pagination = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HourlyUsageAttributionMetadata {
                    pagination,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HourlyUsageAttributionMetadataVisitor)
    }
}
