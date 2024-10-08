// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a Datadog Log index.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsIndex {
    /// The number of log events you can send in this index per day before you are rate-limited.
    #[serde(rename = "daily_limit")]
    pub daily_limit: Option<i64>,
    /// Object containing options to override the default daily limit reset time.
    #[serde(rename = "daily_limit_reset")]
    pub daily_limit_reset: Option<crate::datadogV1::model::LogsDailyLimitReset>,
    /// A percentage threshold of the daily quota at which a Datadog warning event is generated.
    #[serde(rename = "daily_limit_warning_threshold_percentage")]
    pub daily_limit_warning_threshold_percentage: Option<f64>,
    /// An array of exclusion objects. The logs are tested against the query of each filter,
    /// following the order of the array. Only the first matching active exclusion matters,
    /// others (if any) are ignored.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Option<Vec<crate::datadogV1::model::LogsExclusion>>,
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV1::model::LogsFilter,
    /// A boolean stating if the index is rate limited, meaning more logs than the daily limit have been sent.
    /// Rate limit is reset every-day at 2pm UTC.
    #[serde(rename = "is_rate_limited")]
    pub is_rate_limited: Option<bool>,
    /// The name of the index.
    #[serde(rename = "name")]
    pub name: String,
    /// The total number of days logs are stored in Standard and Flex Tier before being deleted from the index.
    /// If Standard Tier is enabled on this index, logs are first retained in Standard Tier for the number of days specified through `num_retention_days`,
    /// and then stored in Flex Tier until the number of days specified in `num_flex_logs_retention_days` is reached.
    /// The available values depend on retention plans specified in your organization's contract/subscriptions.
    #[serde(rename = "num_flex_logs_retention_days")]
    pub num_flex_logs_retention_days: Option<i64>,
    /// The number of days logs are stored in Standard Tier before aging into the Flex Tier or being deleted from the index.
    /// The available values depend on retention plans specified in your organization's contract/subscriptions.
    #[serde(rename = "num_retention_days")]
    pub num_retention_days: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsIndex {
    pub fn new(filter: crate::datadogV1::model::LogsFilter, name: String) -> LogsIndex {
        LogsIndex {
            daily_limit: None,
            daily_limit_reset: None,
            daily_limit_warning_threshold_percentage: None,
            exclusion_filters: None,
            filter,
            is_rate_limited: None,
            name,
            num_flex_logs_retention_days: None,
            num_retention_days: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn daily_limit(mut self, value: i64) -> Self {
        self.daily_limit = Some(value);
        self
    }

    pub fn daily_limit_reset(
        mut self,
        value: crate::datadogV1::model::LogsDailyLimitReset,
    ) -> Self {
        self.daily_limit_reset = Some(value);
        self
    }

    pub fn daily_limit_warning_threshold_percentage(mut self, value: f64) -> Self {
        self.daily_limit_warning_threshold_percentage = Some(value);
        self
    }

    pub fn exclusion_filters(mut self, value: Vec<crate::datadogV1::model::LogsExclusion>) -> Self {
        self.exclusion_filters = Some(value);
        self
    }

    pub fn is_rate_limited(mut self, value: bool) -> Self {
        self.is_rate_limited = Some(value);
        self
    }

    pub fn num_flex_logs_retention_days(mut self, value: i64) -> Self {
        self.num_flex_logs_retention_days = Some(value);
        self
    }

    pub fn num_retention_days(mut self, value: i64) -> Self {
        self.num_retention_days = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for LogsIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsIndexVisitor;
        impl<'a> Visitor<'a> for LogsIndexVisitor {
            type Value = LogsIndex;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut daily_limit: Option<i64> = None;
                let mut daily_limit_reset: Option<crate::datadogV1::model::LogsDailyLimitReset> =
                    None;
                let mut daily_limit_warning_threshold_percentage: Option<f64> = None;
                let mut exclusion_filters: Option<Vec<crate::datadogV1::model::LogsExclusion>> =
                    None;
                let mut filter: Option<crate::datadogV1::model::LogsFilter> = None;
                let mut is_rate_limited: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut num_flex_logs_retention_days: Option<i64> = None;
                let mut num_retention_days: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "daily_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            daily_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "daily_limit_reset" => {
                            if v.is_null() {
                                continue;
                            }
                            daily_limit_reset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "daily_limit_warning_threshold_percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            daily_limit_warning_threshold_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclusion_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            exclusion_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_rate_limited" => {
                            if v.is_null() {
                                continue;
                            }
                            is_rate_limited =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_flex_logs_retention_days" => {
                            if v.is_null() {
                                continue;
                            }
                            num_flex_logs_retention_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_retention_days" => {
                            if v.is_null() {
                                continue;
                            }
                            num_retention_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = LogsIndex {
                    daily_limit,
                    daily_limit_reset,
                    daily_limit_warning_threshold_percentage,
                    exclusion_filters,
                    filter,
                    is_rate_limited,
                    name,
                    num_flex_logs_retention_days,
                    num_retention_days,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsIndexVisitor)
    }
}
