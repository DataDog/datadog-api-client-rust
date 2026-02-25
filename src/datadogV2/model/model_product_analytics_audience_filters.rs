// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Audience filter definitions for targeting specific user segments.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsAudienceFilters {
    /// Account audience queries.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV2::model::ProductAnalyticsAudienceAccountSubquery>>,
    /// Boolean formula combining audience queries by name.
    #[serde(rename = "formula")]
    pub formula: Option<String>,
    /// Segment audience queries.
    #[serde(rename = "segments")]
    pub segments: Option<Vec<crate::datadogV2::model::ProductAnalyticsAudienceSegmentSubquery>>,
    /// User audience queries.
    #[serde(rename = "users")]
    pub users: Option<Vec<crate::datadogV2::model::ProductAnalyticsAudienceUserSubquery>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsAudienceFilters {
    pub fn new() -> ProductAnalyticsAudienceFilters {
        ProductAnalyticsAudienceFilters {
            accounts: None,
            formula: None,
            segments: None,
            users: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn accounts(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsAudienceAccountSubquery>,
    ) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn formula(mut self, value: String) -> Self {
        self.formula = Some(value);
        self
    }

    pub fn segments(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsAudienceSegmentSubquery>,
    ) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn users(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsAudienceUserSubquery>,
    ) -> Self {
        self.users = Some(value);
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

impl Default for ProductAnalyticsAudienceFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsAudienceFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsAudienceFiltersVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsAudienceFiltersVisitor {
            type Value = ProductAnalyticsAudienceFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsAudienceAccountSubquery>,
                > = None;
                let mut formula: Option<String> = None;
                let mut segments: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsAudienceSegmentSubquery>,
                > = None;
                let mut users: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsAudienceUserSubquery>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "formula" => {
                            if v.is_null() {
                                continue;
                            }
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "segments" => {
                            if v.is_null() {
                                continue;
                            }
                            segments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsAudienceFilters {
                    accounts,
                    formula,
                    segments,
                    users,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsAudienceFiltersVisitor)
    }
}
