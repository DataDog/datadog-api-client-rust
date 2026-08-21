// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Splits retention results by the values of a facet.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsRetentionGroupBy {
    /// The attribute path to group by.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Maximum number of groups to return. Omit it to let the service choose.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Whether to drop entities that have no value for the facet.
    #[serde(rename = "should_exclude_missing")]
    pub should_exclude_missing: Option<bool>,
    /// Sort configuration for group-by results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::ProductAnalyticsGroupBySort>,
    /// Audience source backing the group-by, when grouping by an audience rather than a facet.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Which axis of the retention grid a group-by applies to.
    #[serde(rename = "target")]
    pub target: crate::datadogV2::model::ProductAnalyticsRetentionGroupByTarget,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsRetentionGroupBy {
    pub fn new(
        facet: String,
        target: crate::datadogV2::model::ProductAnalyticsRetentionGroupByTarget,
    ) -> ProductAnalyticsRetentionGroupBy {
        ProductAnalyticsRetentionGroupBy {
            facet,
            limit: None,
            should_exclude_missing: None,
            sort: None,
            source: None,
            target,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn sort(mut self, value: crate::datadogV2::model::ProductAnalyticsGroupBySort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsRetentionGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsRetentionGroupByVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsRetentionGroupByVisitor {
            type Value = ProductAnalyticsRetentionGroupBy;

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
                let mut sort: Option<crate::datadogV2::model::ProductAnalyticsGroupBySort> = None;
                let mut source: Option<String> = None;
                let mut target: Option<
                    crate::datadogV2::model::ProductAnalyticsRetentionGroupByTarget,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV2::model::ProductAnalyticsRetentionGroupByTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = ProductAnalyticsRetentionGroupBy {
                    facet,
                    limit,
                    should_exclude_missing,
                    sort,
                    source,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsRetentionGroupByVisitor)
    }
}
