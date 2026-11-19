// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The variant to serve once the bandit stops.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsStopBanditV2RequestDataAttributes {
    /// Key of the flag variant to serve to everyone once the bandit stops. It can be any of the flag's variants, and overrides the configured fallback.
    #[serde(rename = "served_variant_key")]
    pub served_variant_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsStopBanditV2RequestDataAttributes {
    pub fn new(served_variant_key: String) -> ExperimentsStopBanditV2RequestDataAttributes {
        ExperimentsStopBanditV2RequestDataAttributes {
            served_variant_key,
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

impl<'de> Deserialize<'de> for ExperimentsStopBanditV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsStopBanditV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsStopBanditV2RequestDataAttributesVisitor {
            type Value = ExperimentsStopBanditV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut served_variant_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "served_variant_key" => {
                            served_variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let served_variant_key = served_variant_key
                    .ok_or_else(|| M::Error::missing_field("served_variant_key"))?;

                let content = ExperimentsStopBanditV2RequestDataAttributes {
                    served_variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsStopBanditV2RequestDataAttributesVisitor)
    }
}
