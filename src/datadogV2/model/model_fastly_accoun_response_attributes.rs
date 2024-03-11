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
    /// The name of the Fastly account.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of services belonging to the parent account.
    #[serde(rename = "services")]
    pub services: Option<Vec<crate::datadogV2::model::FastlyService>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FastlyAccounResponseAttributes {
    pub fn new(name: String) -> FastlyAccounResponseAttributes {
        FastlyAccounResponseAttributes {
            name,
            services: None,
            _unparsed: false,
        }
    }

    pub fn services(&mut self, value: Vec<crate::datadogV2::model::FastlyService>) -> &mut Self {
        self.services = Some(value);
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
                let mut name: Option<String> = None;
                let mut services: Option<Vec<crate::datadogV2::model::FastlyService>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FastlyAccounResponseAttributes {
                    name,
                    services,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FastlyAccounResponseAttributesVisitor)
    }
}
