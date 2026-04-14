// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Group by configuration for retention queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionGroupBy {
    /// Facet to group by.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Maximum number of groups.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Whether to exclude missing values.
    #[serde(rename = "should_exclude_missing")]
    pub should_exclude_missing: Option<bool>,
    /// Sort configuration for retention group by.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::RetentionGroupBySort>,
    /// Source field.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Target for retention group by.
    #[serde(rename = "target")]
    pub target: crate::datadogV1::model::RetentionGroupByTarget,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionGroupBy {
    pub fn new(
        facet: String,
        target: crate::datadogV1::model::RetentionGroupByTarget,
    ) -> RetentionGroupBy {
        RetentionGroupBy {
            facet,
            limit: None,
            should_exclude_missing: None,
            sort: None,
            source: None,
            target,
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn should_exclude_missing(mut self, value: bool) -> Self {
        self.should_exclude_missing = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV1::model::RetentionGroupBySort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RetentionGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionGroupByVisitor;
        impl<'a> Visitor<'a> for RetentionGroupByVisitor {
            type Value = RetentionGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut limit: Option<i64> = None;
                let mut should_exclude_missing: Option<bool> = None;
                let mut sort: Option<crate::datadogV1::model::RetentionGroupBySort> = None;
                let mut source: Option<String> = None;
                let mut target: Option<crate::datadogV1::model::RetentionGroupByTarget> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_exclude_missing" => {
                            if v.is_null() {
                                continue;
                            }
                            should_exclude_missing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV1::model::RetentionGroupByTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = RetentionGroupBy {
                    facet,
                    limit,
                    should_exclude_missing,
                    sort,
                    source,
                    target,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionGroupByVisitor)
    }
}
