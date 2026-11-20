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
pub struct LogQueryDefinitionGroupBy {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "should_exclude_missing")]
    pub should_exclude_missing: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::LogQueryDefinitionGroupBySort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LogQueryDefinitionGroupBy {
    pub fn new(facet: String) -> LogQueryDefinitionGroupBy {
        LogQueryDefinitionGroupBy {
            facet,
            limit: None,
            should_exclude_missing: None,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn should_exclude_missing(mut self, value: bool) -> Self {
        self.should_exclude_missing = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::LogQueryDefinitionGroupBySort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for LogQueryDefinitionGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogQueryDefinitionGroupByVisitor;
        impl<'a> Visitor<'a> for LogQueryDefinitionGroupByVisitor {
            type Value = LogQueryDefinitionGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut limit: Option<f64> = None;
                let mut should_exclude_missing: Option<bool> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::LogQueryDefinitionGroupBySort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "should_exclude_missing" => {
                            if v.is_null() {
                                continue;
                            }
                            should_exclude_missing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = LogQueryDefinitionGroupBy {
                    facet,
                    limit,
                    should_exclude_missing,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogQueryDefinitionGroupByVisitor)
    }
}
