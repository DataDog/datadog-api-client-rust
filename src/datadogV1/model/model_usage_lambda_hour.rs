// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of lambda functions and sum of the invocations of all lambda functions
/// for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageLambdaHour {
    /// Contains the number of different functions for each region and AWS account.
    #[serde(
        rename = "func_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub func_count: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// Contains the sum of invocations of all functions.
    #[serde(
        rename = "invocations_sum",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub invocations_sum: Option<Option<i64>>,
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

impl UsageLambdaHour {
    pub fn new() -> UsageLambdaHour {
        UsageLambdaHour {
            func_count: None,
            hour: None,
            invocations_sum: None,
            org_name: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn func_count(mut self, value: Option<i64>) -> Self {
        self.func_count = Some(value);
        self
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn invocations_sum(mut self, value: Option<i64>) -> Self {
        self.invocations_sum = Some(value);
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

impl Default for UsageLambdaHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageLambdaHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageLambdaHourVisitor;
        impl<'a> Visitor<'a> for UsageLambdaHourVisitor {
            type Value = UsageLambdaHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut func_count: Option<Option<i64>> = None;
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut invocations_sum: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "func_count" => {
                            func_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invocations_sum" => {
                            invocations_sum =
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

                let content = UsageLambdaHour {
                    func_count,
                    hour,
                    invocations_sum,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageLambdaHourVisitor)
    }
}
