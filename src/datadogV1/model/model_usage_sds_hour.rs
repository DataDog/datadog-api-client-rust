// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sensitive Data Scanner usage for a given organization for a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSDSHour {
    /// The total number of bytes scanned of APM usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "apm_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub apm_scanned_bytes: Option<Option<i64>>,
    /// The total number of bytes scanned of Events usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "events_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub events_scanned_bytes: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<DateTime<Utc>>,
    /// The total number of bytes scanned of logs usage by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "logs_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub logs_scanned_bytes: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The total number of bytes scanned of RUM usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "rum_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rum_scanned_bytes: Option<Option<i64>>,
    /// The total number of bytes scanned across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "total_scanned_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_scanned_bytes: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSDSHour {
    pub fn new() -> UsageSDSHour {
        UsageSDSHour {
            apm_scanned_bytes: None,
            events_scanned_bytes: None,
            hour: None,
            logs_scanned_bytes: None,
            org_name: None,
            public_id: None,
            rum_scanned_bytes: None,
            total_scanned_bytes: None,
            _unparsed: false,
        }
    }

    pub fn apm_scanned_bytes(mut self, value: Option<i64>) -> Self {
        self.apm_scanned_bytes = Some(value);
        self
    }

    pub fn events_scanned_bytes(mut self, value: Option<i64>) -> Self {
        self.events_scanned_bytes = Some(value);
        self
    }

    pub fn hour(mut self, value: DateTime<Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn logs_scanned_bytes(mut self, value: Option<i64>) -> Self {
        self.logs_scanned_bytes = Some(value);
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

    pub fn rum_scanned_bytes(mut self, value: Option<i64>) -> Self {
        self.rum_scanned_bytes = Some(value);
        self
    }

    pub fn total_scanned_bytes(mut self, value: Option<i64>) -> Self {
        self.total_scanned_bytes = Some(value);
        self
    }
}

impl Default for UsageSDSHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSDSHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSDSHourVisitor;
        impl<'a> Visitor<'a> for UsageSDSHourVisitor {
            type Value = UsageSDSHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_scanned_bytes: Option<Option<i64>> = None;
                let mut events_scanned_bytes: Option<Option<i64>> = None;
                let mut hour: Option<DateTime<Utc>> = None;
                let mut logs_scanned_bytes: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut rum_scanned_bytes: Option<Option<i64>> = None;
                let mut total_scanned_bytes: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_scanned_bytes" => {
                            apm_scanned_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events_scanned_bytes" => {
                            events_scanned_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs_scanned_bytes" => {
                            logs_scanned_bytes =
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
                        "rum_scanned_bytes" => {
                            rum_scanned_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_scanned_bytes" => {
                            total_scanned_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageSDSHour {
                    apm_scanned_bytes,
                    events_scanned_bytes,
                    hour,
                    logs_scanned_bytes,
                    org_name,
                    public_id,
                    rum_scanned_bytes,
                    total_scanned_bytes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSDSHourVisitor)
    }
}
