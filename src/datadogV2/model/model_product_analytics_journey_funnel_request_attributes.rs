// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a journey funnel request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyFunnelRequestAttributes {
    /// Whether to exclude sessions that are not tied to an identified user.
    #[serde(rename = "exclude_anonymous_traffic")]
    pub exclude_anonymous_traffic: Option<bool>,
    /// Start of the query window, in epoch milliseconds.
    #[serde(rename = "from")]
    pub from: i64,
    /// Query definition for a journey funnel request.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::ProductAnalyticsJourneyFunnelQuery,
    /// End of the query window, in epoch milliseconds.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyFunnelRequestAttributes {
    pub fn new(
        from: i64,
        query: crate::datadogV2::model::ProductAnalyticsJourneyFunnelQuery,
        to: i64,
    ) -> ProductAnalyticsJourneyFunnelRequestAttributes {
        ProductAnalyticsJourneyFunnelRequestAttributes {
            exclude_anonymous_traffic: None,
            from,
            query,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclude_anonymous_traffic(mut self, value: bool) -> Self {
        self.exclude_anonymous_traffic = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyFunnelRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyFunnelRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyFunnelRequestAttributesVisitor {
            type Value = ProductAnalyticsJourneyFunnelRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclude_anonymous_traffic: Option<bool> = None;
                let mut from: Option<i64> = None;
                let mut query: Option<crate::datadogV2::model::ProductAnalyticsJourneyFunnelQuery> =
                    None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclude_anonymous_traffic" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_anonymous_traffic =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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

                let content = ProductAnalyticsJourneyFunnelRequestAttributes {
                    exclude_anonymous_traffic,
                    from,
                    query,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyFunnelRequestAttributesVisitor)
    }
}
