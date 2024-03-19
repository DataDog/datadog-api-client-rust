// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the number of hourly recorded custom metrics for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageTopAvgMetricsResponse {
    /// The object containing document metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::UsageTopAvgMetricsMetadata>,
    /// Number of hourly recorded custom metrics for a given organization.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageTopAvgMetricsHour>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageTopAvgMetricsResponse {
    pub fn new() -> UsageTopAvgMetricsResponse {
        UsageTopAvgMetricsResponse {
            metadata: None,
            usage: None,
            _unparsed: false,
        }
    }

    pub fn metadata(mut self, value: crate::datadogV1::model::UsageTopAvgMetricsMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageTopAvgMetricsHour>) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageTopAvgMetricsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageTopAvgMetricsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageTopAvgMetricsResponseVisitor;
        impl<'a> Visitor<'a> for UsageTopAvgMetricsResponseVisitor {
            type Value = UsageTopAvgMetricsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metadata: Option<crate::datadogV1::model::UsageTopAvgMetricsMetadata> =
                    None;
                let mut usage: Option<Vec<crate::datadogV1::model::UsageTopAvgMetricsHour>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageTopAvgMetricsResponse {
                    metadata,
                    usage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageTopAvgMetricsResponseVisitor)
    }
}
