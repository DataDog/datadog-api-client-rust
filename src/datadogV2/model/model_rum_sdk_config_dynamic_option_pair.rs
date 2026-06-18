// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A key-value pair where the value is a dynamic configuration option.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSdkConfigDynamicOptionPair {
    /// The key name for this dynamic configuration pair.
    #[serde(rename = "key")]
    pub key: String,
    /// A dynamic configuration option that extracts a value at runtime using a specified strategy.
    #[serde(rename = "value")]
    pub value: crate::datadogV2::model::RumSdkConfigDynamicOption,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSdkConfigDynamicOptionPair {
    pub fn new(
        key: String,
        value: crate::datadogV2::model::RumSdkConfigDynamicOption,
    ) -> RumSdkConfigDynamicOptionPair {
        RumSdkConfigDynamicOptionPair {
            key,
            value,
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

impl<'de> Deserialize<'de> for RumSdkConfigDynamicOptionPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSdkConfigDynamicOptionPairVisitor;
        impl<'a> Visitor<'a> for RumSdkConfigDynamicOptionPairVisitor {
            type Value = RumSdkConfigDynamicOptionPair;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key: Option<String> = None;
                let mut value: Option<crate::datadogV2::model::RumSdkConfigDynamicOption> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = RumSdkConfigDynamicOptionPair {
                    key,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSdkConfigDynamicOptionPairVisitor)
    }
}
