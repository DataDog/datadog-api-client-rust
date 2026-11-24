// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the RUM SDK configuration to update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMSdkConfigUpdateAttributes {
    /// The RUM SDK settings to apply when updating a configuration.
    #[serde(rename = "rum")]
    pub rum: crate::datadogV2::model::RUMSdkConfigRUMUpdateAttributes,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMSdkConfigUpdateAttributes {
    pub fn new(
        rum: crate::datadogV2::model::RUMSdkConfigRUMUpdateAttributes,
    ) -> RUMSdkConfigUpdateAttributes {
        RUMSdkConfigUpdateAttributes {
            rum,
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

impl<'de> Deserialize<'de> for RUMSdkConfigUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMSdkConfigUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for RUMSdkConfigUpdateAttributesVisitor {
            type Value = RUMSdkConfigUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rum: Option<crate::datadogV2::model::RUMSdkConfigRUMUpdateAttributes> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rum" => {
                            rum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rum = rum.ok_or_else(|| M::Error::missing_field("rum"))?;

                let content = RUMSdkConfigUpdateAttributes {
                    rum,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMSdkConfigUpdateAttributesVisitor)
    }
}
