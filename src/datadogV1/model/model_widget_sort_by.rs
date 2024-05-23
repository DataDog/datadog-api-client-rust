// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The controls for sorting the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetSortBy {
    /// The number of items to limit the widget to.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// The array of items to sort the widget by in order.
    #[serde(rename = "order_by")]
    pub order_by: Option<Vec<crate::datadogV1::model::WidgetSortOrderBy>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetSortBy {
    pub fn new() -> WidgetSortBy {
        WidgetSortBy {
            count: None,
            order_by: None,
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn order_by(mut self, value: Vec<crate::datadogV1::model::WidgetSortOrderBy>) -> Self {
        self.order_by = Some(value);
        self
    }
}

impl Default for WidgetSortBy {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetSortBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetSortByVisitor;
        impl<'a> Visitor<'a> for WidgetSortByVisitor {
            type Value = WidgetSortBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut order_by: Option<Vec<crate::datadogV1::model::WidgetSortOrderBy>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order_by" => {
                            if v.is_null() {
                                continue;
                            }
                            order_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WidgetSortBy {
                    count,
                    order_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetSortByVisitor)
    }
}
