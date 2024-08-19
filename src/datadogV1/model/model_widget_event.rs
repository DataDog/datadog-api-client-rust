// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Event overlay control options.
///
/// See the dedicated [Events JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/widget_json/#events-schema>)
/// to learn how to build the `<EVENTS_SCHEMA>`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetEvent {
    /// Query definition.
    #[serde(rename = "q")]
    pub q: String,
    /// The execution method for multi-value filters.
    #[serde(rename = "tags_execution")]
    pub tags_execution: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetEvent {
    pub fn new(q: String) -> WidgetEvent {
        WidgetEvent {
            q,
            tags_execution: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tags_execution(mut self, value: String) -> Self {
        self.tags_execution = Some(value);
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

impl<'de> Deserialize<'de> for WidgetEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetEventVisitor;
        impl<'a> Visitor<'a> for WidgetEventVisitor {
            type Value = WidgetEvent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut q: Option<String> = None;
                let mut tags_execution: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "q" => {
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags_execution" => {
                            if v.is_null() {
                                continue;
                            }
                            tags_execution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let q = q.ok_or_else(|| M::Error::missing_field("q"))?;

                let content = WidgetEvent {
                    q,
                    tags_execution,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetEventVisitor)
    }
}
