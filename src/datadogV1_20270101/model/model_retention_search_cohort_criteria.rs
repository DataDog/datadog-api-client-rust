// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionSearchCohortCriteria {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "base_query")]
    pub base_query: crate::datadogV1_20270101::model::BaseProductAnalyticsQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_interval")]
    pub time_interval: crate::datadogV1_20270101::model::RetentionSearchCohortCriteriaTimeInterval,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RetentionSearchCohortCriteria {
    pub fn new(base_query: crate::datadogV1_20270101::model::BaseProductAnalyticsQuery, time_interval: crate::datadogV1_20270101::model::RetentionSearchCohortCriteriaTimeInterval) -> RetentionSearchCohortCriteria {
        RetentionSearchCohortCriteria {
            base_query,
            time_interval,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for RetentionSearchCohortCriteria {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionSearchCohortCriteriaVisitor;
        impl<'a> Visitor<'a> for RetentionSearchCohortCriteriaVisitor {
            type Value = RetentionSearchCohortCriteria;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_query: Option<crate::datadogV1_20270101::model::BaseProductAnalyticsQuery> = None;
                let mut time_interval: Option<crate::datadogV1_20270101::model::RetentionSearchCohortCriteriaTimeInterval> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_query" => {
                            base_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _base_query) = base_query {
                                match _base_query {
                                    crate::datadogV1_20270101::model::BaseProductAnalyticsQuery::UnparsedObject(_base_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "time_interval" => {
                            time_interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let base_query = base_query.ok_or_else(|| M::Error::missing_field("base_query"))?;
                let time_interval = time_interval.ok_or_else(|| M::Error::missing_field("time_interval"))?;

                let content = RetentionSearchCohortCriteria {
                    base_query,
                    time_interval,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionSearchCohortCriteriaVisitor)
    }
}
