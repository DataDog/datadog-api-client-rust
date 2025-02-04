// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `RetryStrategy` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetryStrategy {
    /// The definition of `RetryStrategyKind` object.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV2::model::RetryStrategyKind,
    /// The definition of `RetryStrategyLinear` object.
    #[serde(rename = "linear")]
    pub linear: Option<crate::datadogV2::model::RetryStrategyLinear>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetryStrategy {
    pub fn new(kind: crate::datadogV2::model::RetryStrategyKind) -> RetryStrategy {
        RetryStrategy {
            kind,
            linear: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn linear(mut self, value: crate::datadogV2::model::RetryStrategyLinear) -> Self {
        self.linear = Some(value);
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

impl<'de> Deserialize<'de> for RetryStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetryStrategyVisitor;
        impl<'a> Visitor<'a> for RetryStrategyVisitor {
            type Value = RetryStrategy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut kind: Option<crate::datadogV2::model::RetryStrategyKind> = None;
                let mut linear: Option<crate::datadogV2::model::RetryStrategyLinear> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::RetryStrategyKind::UnparsedObject(
                                        _kind,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "linear" => {
                            if v.is_null() {
                                continue;
                            }
                            linear = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;

                let content = RetryStrategy {
                    kind,
                    linear,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetryStrategyVisitor)
    }
}
