// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AppBuilderErrorErrorsItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AppBuilderErrorErrorsItems {
    /// The `items` `detail`.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// The definition of `AppBuilderErrorErrorsItemsSource` object.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV2::model::AppBuilderErrorErrorsItemsSource>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AppBuilderErrorErrorsItems {
    pub fn new() -> AppBuilderErrorErrorsItems {
        AppBuilderErrorErrorsItems {
            detail: None,
            source: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn detail(mut self, value: String) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn source(
        mut self,
        value: crate::datadogV2::model::AppBuilderErrorErrorsItemsSource,
    ) -> Self {
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

impl Default for AppBuilderErrorErrorsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AppBuilderErrorErrorsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AppBuilderErrorErrorsItemsVisitor;
        impl<'a> Visitor<'a> for AppBuilderErrorErrorsItemsVisitor {
            type Value = AppBuilderErrorErrorsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detail: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::AppBuilderErrorErrorsItemsSource> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "detail" => {
                            if v.is_null() {
                                continue;
                            }
                            detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AppBuilderErrorErrorsItems {
                    detail,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AppBuilderErrorErrorsItemsVisitor)
    }
}
