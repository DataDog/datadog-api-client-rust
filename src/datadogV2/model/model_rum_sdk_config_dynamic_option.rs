// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A dynamic configuration option that extracts a value at runtime using a specified strategy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSdkConfigDynamicOption {
    /// The element attribute to read. Used when `strategy` is `dom`.
    #[serde(rename = "attribute")]
    pub attribute: Option<String>,
    /// A serialized regex used as an extractor in dynamic options.
    #[serde(rename = "extractor")]
    pub extractor: Option<crate::datadogV2::model::RumSdkConfigSerializedRegex>,
    /// The `localStorage` key to read. Required when `strategy` is `localStorage`.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The cookie name to read. Required when `strategy` is `cookie`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The JavaScript path used to extract the value. Required when `strategy` is `js`.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// The type identifier for a dynamic option. Always `dynamic`.
    #[serde(rename = "rc_serialized_type")]
    pub rc_serialized_type: crate::datadogV2::model::RumSdkConfigDynamicOptionSerializedType,
    /// The CSS selector to read from the page. Required when `strategy` is `dom`.
    #[serde(rename = "selector")]
    pub selector: Option<String>,
    /// The strategy used to extract the dynamic value.
    #[serde(rename = "strategy")]
    pub strategy: crate::datadogV2::model::RumSdkConfigDynamicOptionStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSdkConfigDynamicOption {
    pub fn new(
        rc_serialized_type: crate::datadogV2::model::RumSdkConfigDynamicOptionSerializedType,
        strategy: crate::datadogV2::model::RumSdkConfigDynamicOptionStrategy,
    ) -> RumSdkConfigDynamicOption {
        RumSdkConfigDynamicOption {
            attribute: None,
            extractor: None,
            key: None,
            name: None,
            path: None,
            rc_serialized_type,
            selector: None,
            strategy,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attribute(mut self, value: String) -> Self {
        self.attribute = Some(value);
        self
    }

    pub fn extractor(
        mut self,
        value: crate::datadogV2::model::RumSdkConfigSerializedRegex,
    ) -> Self {
        self.extractor = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    pub fn selector(mut self, value: String) -> Self {
        self.selector = Some(value);
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

impl<'de> Deserialize<'de> for RumSdkConfigDynamicOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSdkConfigDynamicOptionVisitor;
        impl<'a> Visitor<'a> for RumSdkConfigDynamicOptionVisitor {
            type Value = RumSdkConfigDynamicOption;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute: Option<String> = None;
                let mut extractor: Option<crate::datadogV2::model::RumSdkConfigSerializedRegex> =
                    None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut path: Option<String> = None;
                let mut rc_serialized_type: Option<
                    crate::datadogV2::model::RumSdkConfigDynamicOptionSerializedType,
                > = None;
                let mut selector: Option<String> = None;
                let mut strategy: Option<
                    crate::datadogV2::model::RumSdkConfigDynamicOptionStrategy,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extractor" => {
                            if v.is_null() {
                                continue;
                            }
                            extractor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            if v.is_null() {
                                continue;
                            }
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rc_serialized_type" => {
                            rc_serialized_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rc_serialized_type) = rc_serialized_type {
                                match _rc_serialized_type {
                                    crate::datadogV2::model::RumSdkConfigDynamicOptionSerializedType::UnparsedObject(_rc_serialized_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "selector" => {
                            if v.is_null() {
                                continue;
                            }
                            selector = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::RumSdkConfigDynamicOptionStrategy::UnparsedObject(_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rc_serialized_type = rc_serialized_type
                    .ok_or_else(|| M::Error::missing_field("rc_serialized_type"))?;
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = RumSdkConfigDynamicOption {
                    attribute,
                    extractor,
                    key,
                    name,
                    path,
                    rc_serialized_type,
                    selector,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSdkConfigDynamicOptionVisitor)
    }
}
