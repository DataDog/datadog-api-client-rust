// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object for updating a Datadog Log index.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsIndexUpdateRequest {
    /// The number of log events you can send in this index per day before you are rate-limited.
    #[serde(rename = "daily_limit")]
    pub daily_limit: Option<i64>,
    /// If true, sets the `daily_limit` value to null and the index is not limited on a daily basis (any
    /// specified `daily_limit` value in the request is ignored). If false or omitted, the index's current
    /// `daily_limit` is maintained.
    #[serde(rename = "disable_daily_limit")]
    pub disable_daily_limit: Option<bool>,
    /// An array of exclusion objects. The logs are tested against the query of each filter,
    /// following the order of the array. Only the first matching active exclusion matters,
    /// others (if any) are ignored.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Option<Vec<crate::datadogV1::model::LogsExclusion>>,
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV1::model::LogsFilter,
    /// The number of days before logs are deleted from this index. Available values depend on
    /// retention plans specified in your organization's contract/subscriptions.
    ///
    /// **Note:** Changing the retention for an index adjusts the length of retention for all logs
    /// already in this index. It may also affect billing.
    #[serde(rename = "num_retention_days")]
    pub num_retention_days: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsIndexUpdateRequest {
    pub fn new(filter: crate::datadogV1::model::LogsFilter) -> LogsIndexUpdateRequest {
        LogsIndexUpdateRequest {
            daily_limit: None,
            disable_daily_limit: None,
            exclusion_filters: None,
            filter,
            num_retention_days: None,
            _unparsed: false,
        }
    }

    pub fn daily_limit(&mut self, value: i64) -> &mut Self {
        self.daily_limit = Some(value);
        self
    }

    pub fn disable_daily_limit(&mut self, value: bool) -> &mut Self {
        self.disable_daily_limit = Some(value);
        self
    }

    pub fn exclusion_filters(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsExclusion>,
    ) -> &mut Self {
        self.exclusion_filters = Some(value);
        self
    }

    pub fn num_retention_days(&mut self, value: i64) -> &mut Self {
        self.num_retention_days = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsIndexUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsIndexUpdateRequestVisitor;
        impl<'a> Visitor<'a> for LogsIndexUpdateRequestVisitor {
            type Value = LogsIndexUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut daily_limit: Option<i64> = None;
                let mut disable_daily_limit: Option<bool> = None;
                let mut exclusion_filters: Option<Vec<crate::datadogV1::model::LogsExclusion>> =
                    None;
                let mut filter: Option<crate::datadogV1::model::LogsFilter> = None;
                let mut num_retention_days: Option<i64> = None;
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
                        "disable_daily_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            disable_daily_limit =
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
                        "num_retention_days" => {
                            if v.is_null() {
                                continue;
                            }
                            num_retention_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;

                let content = LogsIndexUpdateRequest {
                    daily_limit,
                    disable_daily_limit,
                    exclusion_filters,
                    filter,
                    num_retention_days,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsIndexUpdateRequestVisitor)
    }
}
