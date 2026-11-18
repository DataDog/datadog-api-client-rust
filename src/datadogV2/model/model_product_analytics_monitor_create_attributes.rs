// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Product Analytics monitor to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsMonitorCreateAttributes {
    /// Notification message sent when the monitor alerts. It can include `@` notification handles.
    #[serde(rename = "message")]
    pub message: String,
    /// Name of the monitor.
    #[serde(rename = "name")]
    pub name: String,
    /// Options of a Product Analytics monitor.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::ProductAnalyticsMonitorOptions>,
    /// Priority of the monitor, from 1 (highest) to 5 (lowest).
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    /// Monitor query, for example `formula("query").rollup("sum").last("1h") < 1`.
    /// Define the variables it references in `options.variables`.
    #[serde(rename = "query")]
    pub query: String,
    /// Monitor tags as key and value pairs. Each pair is stored as a `key:value` tag.
    /// `chart_id` and `dashboard_id` link the monitor to the chart it was created from. `dashboard_id` requires
    /// `chart_id`.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsMonitorCreateAttributes {
    pub fn new(
        message: String,
        name: String,
        query: String,
    ) -> ProductAnalyticsMonitorCreateAttributes {
        ProductAnalyticsMonitorCreateAttributes {
            message,
            name,
            options: None,
            priority: None,
            query,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn options(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsMonitorOptions,
    ) -> Self {
        self.options = Some(value);
        self
    }

    pub fn priority(mut self, value: i32) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsMonitorCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsMonitorCreateAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsMonitorCreateAttributesVisitor {
            type Value = ProductAnalyticsMonitorCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::ProductAnalyticsMonitorOptions> =
                    None;
                let mut priority: Option<i32> = None;
                let mut query: Option<String> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = ProductAnalyticsMonitorCreateAttributes {
                    message,
                    name,
                    options,
                    priority,
                    query,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsMonitorCreateAttributesVisitor)
    }
}
