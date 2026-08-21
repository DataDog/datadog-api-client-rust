// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an analytics list request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsAnalyticsListRequestAttributes {
    /// Start time in epoch milliseconds. Must be less than `to`.
    #[serde(rename = "from")]
    pub from: i64,
    /// The analytics list query definition. It selects the events to return with `query`, then
    /// chooses the columns on each event row, the sort applied to those rows, and a row limit.
    /// Unlike the scalar and timeseries queries, a list query returns raw event rows rather than
    /// aggregates, so it takes no compute or group-by rule.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::ProductAnalyticsAnalyticsListQuery,
    /// End time in epoch milliseconds.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsAnalyticsListRequestAttributes {
    pub fn new(
        from: i64,
        query: crate::datadogV2::model::ProductAnalyticsAnalyticsListQuery,
        to: i64,
    ) -> ProductAnalyticsAnalyticsListRequestAttributes {
        ProductAnalyticsAnalyticsListRequestAttributes {
            from,
            query,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsAnalyticsListRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsAnalyticsListRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsAnalyticsListRequestAttributesVisitor {
            type Value = ProductAnalyticsAnalyticsListRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<i64> = None;
                let mut query: Option<crate::datadogV2::model::ProductAnalyticsAnalyticsListQuery> =
                    None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ProductAnalyticsAnalyticsListRequestAttributes {
                    from,
                    query,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsAnalyticsListRequestAttributesVisitor)
    }
}
