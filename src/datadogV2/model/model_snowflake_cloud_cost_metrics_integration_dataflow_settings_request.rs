// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Cloud Cost Management dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
    /// Snowflake query tags to ingest, so that cost data can be broken down by them in Cloud Cost Management. Provide the tag names as a comma-separated list without spaces, using only letters, digits, underscores, dots, and hyphens. Datadog does not collect query tags by default.
    #[serde(rename = "query_tags")]
    pub query_tags: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
        SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
            query_tags: None,
            _unparsed: false,
        }
    }

    pub fn query_tags(mut self, value: String) -> Self {
        self.query_tags = Some(value);
        self
    }
}

impl Default for SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequestVisitor {
            type Value = SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query_tags: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            query_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest {
                    query_tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequestVisitor)
    }
}
