// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Return criteria for retention queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionReturnCriteria {
    /// Product Analytics event query.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1::model::ProductAnalyticsBaseQuery,
    /// Time interval for return criteria.
    #[serde(rename = "time_interval")]
    pub time_interval: Option<crate::datadogV1::model::RetentionReturnCriteriaTimeInterval>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionReturnCriteria {
    pub fn new(
        base_query: crate::datadogV1::model::ProductAnalyticsBaseQuery,
    ) -> RetentionReturnCriteria {
        RetentionReturnCriteria {
            base_query,
            time_interval: None,
            _unparsed: false,
        }
    }

    pub fn time_interval(
        mut self,
        value: crate::datadogV1::model::RetentionReturnCriteriaTimeInterval,
    ) -> Self {
        self.time_interval = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RetentionReturnCriteria {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionReturnCriteriaVisitor;
        impl<'a> Visitor<'a> for RetentionReturnCriteriaVisitor {
            type Value = RetentionReturnCriteria;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_query: Option<crate::datadogV1::model::ProductAnalyticsBaseQuery> =
                    None;
                let mut time_interval: Option<
                    crate::datadogV1::model::RetentionReturnCriteriaTimeInterval,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            time_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;

                let content = RetentionReturnCriteria {
                    base_query,
                    time_interval,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionReturnCriteriaVisitor)
    }
}
