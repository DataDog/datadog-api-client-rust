// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object of a Fastly account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FastlyAccounResponseAttributes {
    /// The FastlyAccounResponseAttributes api_key.
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    /// The name of the Fastly account.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of services belonging to the parent account.
    #[serde(rename = "services")]
    pub services: Option<Vec<crate::datadogV2::model::FastlyService>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FastlyAccounResponseAttributes {
    pub fn new(name: String) -> FastlyAccounResponseAttributes {
        FastlyAccounResponseAttributes {
            api_key: None,
            name,
            services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_key(mut self, value: String) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<crate::datadogV2::model::FastlyService>) -> Self {
        self.services = Some(value);
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

impl<'de> Deserialize<'de> for FastlyAccounResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FastlyAccounResponseAttributesVisitor;
        impl<'a> Visitor<'a> for FastlyAccounResponseAttributesVisitor {
            type Value = FastlyAccounResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut services: Option<Vec<crate::datadogV2::model::FastlyService>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FastlyAccounResponseAttributes {
                    api_key,
                    name,
                    services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FastlyAccounResponseAttributesVisitor)
    }
}
