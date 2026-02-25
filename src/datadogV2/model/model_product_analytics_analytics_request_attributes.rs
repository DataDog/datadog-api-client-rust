// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an analytics request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsAnalyticsRequestAttributes {
    /// Override the query execution strategy.
    #[serde(rename = "enforced_execution_type")]
    pub enforced_execution_type: Option<crate::datadogV2::model::ProductAnalyticsExecutionType>,
    /// Start time in epoch milliseconds. Must be less than `to`.
    #[serde(rename = "from")]
    pub from: i64,
    /// The analytics query definition containing a base query, compute rule, and optional grouping.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::ProductAnalyticsAnalyticsQuery,
    /// Optional request ID for multi-step query continuation.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// End time in epoch milliseconds.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsAnalyticsRequestAttributes {
    pub fn new(
        from: i64,
        query: crate::datadogV2::model::ProductAnalyticsAnalyticsQuery,
        to: i64,
    ) -> ProductAnalyticsAnalyticsRequestAttributes {
        ProductAnalyticsAnalyticsRequestAttributes {
            enforced_execution_type: None,
            from,
            query,
            request_id: None,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enforced_execution_type(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsExecutionType,
    ) -> Self {
        self.enforced_execution_type = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsAnalyticsRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsAnalyticsRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsAnalyticsRequestAttributesVisitor {
            type Value = ProductAnalyticsAnalyticsRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enforced_execution_type: Option<
                    crate::datadogV2::model::ProductAnalyticsExecutionType,
                > = None;
                let mut from: Option<i64> = None;
                let mut query: Option<crate::datadogV2::model::ProductAnalyticsAnalyticsQuery> =
                    None;
                let mut request_id: Option<String> = None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enforced_execution_type" => {
                            if v.is_null() {
                                continue;
                            }
                            enforced_execution_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _enforced_execution_type) = enforced_execution_type {
                                match _enforced_execution_type {
                                    crate::datadogV2::model::ProductAnalyticsExecutionType::UnparsedObject(_enforced_execution_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = ProductAnalyticsAnalyticsRequestAttributes {
                    enforced_execution_type,
                    from,
                    query,
                    request_id,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsAnalyticsRequestAttributesVisitor)
    }
}
