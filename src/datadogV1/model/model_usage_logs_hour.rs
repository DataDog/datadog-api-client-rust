// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Hour usage for logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageLogsHour {
    /// Contains the number of billable log bytes ingested.
    #[serde(
        rename = "billable_ingested_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub billable_ingested_bytes: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// Contains the number of log events indexed.
    #[serde(
        rename = "indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub indexed_events_count: Option<Option<i64>>,
    /// Contains the number of log bytes ingested.
    #[serde(
        rename = "ingested_events_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ingested_events_bytes: Option<Option<i64>>,
    /// Contains the number of logs forwarded bytes (data available as of April 1st 2023)
    #[serde(
        rename = "logs_forwarding_events_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_forwarding_events_bytes: Option<Option<i64>>,
    /// Contains the number of live log events indexed (data available as of December 1, 2020).
    #[serde(
        rename = "logs_live_indexed_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_live_indexed_count: Option<Option<i64>>,
    /// Contains the number of live log bytes ingested (data available as of December 1, 2020).
    #[serde(
        rename = "logs_live_ingested_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_live_ingested_bytes: Option<Option<i64>>,
    /// Contains the number of rehydrated log events indexed (data available as of December 1, 2020).
    #[serde(
        rename = "logs_rehydrated_indexed_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_rehydrated_indexed_count: Option<Option<i64>>,
    /// Contains the number of rehydrated log bytes ingested (data available as of December 1, 2020).
    #[serde(
        rename = "logs_rehydrated_ingested_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_rehydrated_ingested_bytes: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageLogsHour {
    pub fn new() -> UsageLogsHour {
        UsageLogsHour {
            billable_ingested_bytes: None,
            hour: None,
            indexed_events_count: None,
            ingested_events_bytes: None,
            logs_forwarding_events_bytes: None,
            logs_live_indexed_count: None,
            logs_live_ingested_bytes: None,
            logs_rehydrated_indexed_count: None,
            logs_rehydrated_ingested_bytes: None,
            org_name: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn billable_ingested_bytes(mut self, value: Option<i64>) -> Self {
        self.billable_ingested_bytes = Some(value);
        self
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn indexed_events_count(mut self, value: Option<i64>) -> Self {
        self.indexed_events_count = Some(value);
        self
    }

    pub fn ingested_events_bytes(mut self, value: Option<i64>) -> Self {
        self.ingested_events_bytes = Some(value);
        self
    }

    pub fn logs_forwarding_events_bytes(mut self, value: Option<i64>) -> Self {
        self.logs_forwarding_events_bytes = Some(value);
        self
    }

    pub fn logs_live_indexed_count(mut self, value: Option<i64>) -> Self {
        self.logs_live_indexed_count = Some(value);
        self
    }

    pub fn logs_live_ingested_bytes(mut self, value: Option<i64>) -> Self {
        self.logs_live_ingested_bytes = Some(value);
        self
    }

    pub fn logs_rehydrated_indexed_count(mut self, value: Option<i64>) -> Self {
        self.logs_rehydrated_indexed_count = Some(value);
        self
    }

    pub fn logs_rehydrated_ingested_bytes(mut self, value: Option<i64>) -> Self {
        self.logs_rehydrated_ingested_bytes = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageLogsHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageLogsHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageLogsHourVisitor;
        impl<'a> Visitor<'a> for UsageLogsHourVisitor {
            type Value = UsageLogsHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billable_ingested_bytes: Option<Option<i64>> = None;
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut indexed_events_count: Option<Option<i64>> = None;
                let mut ingested_events_bytes: Option<Option<i64>> = None;
                let mut logs_forwarding_events_bytes: Option<Option<i64>> = None;
                let mut logs_live_indexed_count: Option<Option<i64>> = None;
                let mut logs_live_ingested_bytes: Option<Option<i64>> = None;
                let mut logs_rehydrated_indexed_count: Option<Option<i64>> = None;
                let mut logs_rehydrated_ingested_bytes: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billable_ingested_bytes" => {
                            billable_ingested_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexed_events_count" => {
                            indexed_events_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested_events_bytes" => {
                            ingested_events_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_forwarding_events_bytes" => {
                            logs_forwarding_events_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_live_indexed_count" => {
                            logs_live_indexed_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_live_ingested_bytes" => {
                            logs_live_ingested_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_rehydrated_indexed_count" => {
                            logs_rehydrated_indexed_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_rehydrated_ingested_bytes" => {
                            logs_rehydrated_ingested_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageLogsHour {
                    billable_ingested_bytes,
                    hour,
                    indexed_events_count,
                    ingested_events_bytes,
                    logs_forwarding_events_bytes,
                    logs_live_indexed_count,
                    logs_live_ingested_bytes,
                    logs_rehydrated_indexed_count,
                    logs_rehydrated_ingested_bytes,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageLogsHourVisitor)
    }
}
