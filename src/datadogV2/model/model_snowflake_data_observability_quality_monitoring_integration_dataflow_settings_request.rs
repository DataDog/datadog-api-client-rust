// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Data Observability dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest {
    /// Cron expression setting how often Datadog crawls your Snowflake table metadata. It takes the five standard fields, with the restriction that the month must be `*` and that the day of the month and the day of the week cannot both be constrained. The Datadog UI offers hourly (`0 * * * *`) and daily (`0 0 * * *`). Defaults to hourly.
    #[serde(rename = "do_table_crawler_cron")]
    pub do_table_crawler_cron: Option<String>,
    /// Whether metadata from the Snowflake `SNOWFLAKE` system database is included in Data Observability alongside your own databases. Defaults to `true`.
    #[serde(rename = "sync_snowflake_system_database")]
    pub sync_snowflake_system_database: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest {
        SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest {
            do_table_crawler_cron: None,
            sync_snowflake_system_database: None,
            _unparsed: false,
        }
    }

    pub fn do_table_crawler_cron(mut self, value: String) -> Self {
        self.do_table_crawler_cron = Some(value);
        self
    }

    pub fn sync_snowflake_system_database(mut self, value: bool) -> Self {
        self.sync_snowflake_system_database = Some(value);
        self
    }
}

impl Default for SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a>
            for SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequestVisitor
        {
            type Value =
                SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut do_table_crawler_cron: Option<String> = None;
                let mut sync_snowflake_system_database: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "do_table_crawler_cron" => {
                            if v.is_null() {
                                continue;
                            }
                            do_table_crawler_cron =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_snowflake_system_database" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_snowflake_system_database =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content =
                    SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest {
                        do_table_crawler_cron,
                        sync_snowflake_system_database,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequestVisitor,
        )
    }
}
