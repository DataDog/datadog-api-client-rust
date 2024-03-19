// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define an expression alias.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesWidgetExpressionAlias {
    /// Expression alias.
    #[serde(rename = "alias_name")]
    pub alias_name: Option<String>,
    /// Expression name.
    #[serde(rename = "expression")]
    pub expression: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesWidgetExpressionAlias {
    pub fn new(expression: String) -> TimeseriesWidgetExpressionAlias {
        TimeseriesWidgetExpressionAlias {
            alias_name: None,
            expression,
            _unparsed: false,
        }
    }

    pub fn alias_name(mut self, value: String) -> Self {
        self.alias_name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TimeseriesWidgetExpressionAlias {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetExpressionAliasVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetExpressionAliasVisitor {
            type Value = TimeseriesWidgetExpressionAlias;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias_name: Option<String> = None;
                let mut expression: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias_name" => {
                            if v.is_null() {
                                continue;
                            }
                            alias_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;

                let content = TimeseriesWidgetExpressionAlias {
                    alias_name,
                    expression,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetExpressionAliasVisitor)
    }
}
