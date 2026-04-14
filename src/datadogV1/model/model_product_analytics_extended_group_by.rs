// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Group by configuration for Product Analytics Extended queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsExtendedGroupBy {
    /// Facet name to group by.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Maximum number of groups to return.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// Whether to exclude events missing the group-by facet.
    #[serde(rename = "should_exclude_missing")]
    pub should_exclude_missing: Option<bool>,
    /// Options for sorting group by results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsExtendedGroupBy {
    pub fn new(facet: String) -> ProductAnalyticsExtendedGroupBy {
        ProductAnalyticsExtendedGroupBy {
            facet,
            limit: None,
            should_exclude_missing: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn should_exclude_missing(mut self, value: bool) -> Self {
        self.should_exclude_missing = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort,
    ) -> Self {
        self.sort = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsExtendedGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsExtendedGroupByVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsExtendedGroupByVisitor {
            type Value = ProductAnalyticsExtendedGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut limit: Option<i32> = None;
                let mut should_exclude_missing: Option<bool> = None;
                let mut sort: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort,
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = ProductAnalyticsExtendedGroupBy {
                    facet,
                    limit,
                    should_exclude_missing,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsExtendedGroupByVisitor)
    }
}
