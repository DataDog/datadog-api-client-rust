// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of LeakedKeyAttributes object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LeakedKeyAttributes {
    /// The LeakedKeyAttributes date.
    #[serde(rename = "date")]
    pub date: chrono::DateTime<chrono::Utc>,
    /// The LeakedKeyAttributes leak_source.
    #[serde(rename = "leak_source")]
    pub leak_source: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LeakedKeyAttributes {
    pub fn new(date: chrono::DateTime<chrono::Utc>) -> LeakedKeyAttributes {
        LeakedKeyAttributes {
            date,
            leak_source: None,
            _unparsed: false,
        }
    }

    pub fn leak_source(mut self, value: String) -> Self {
        self.leak_source = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LeakedKeyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LeakedKeyAttributesVisitor;
        impl<'a> Visitor<'a> for LeakedKeyAttributesVisitor {
            type Value = LeakedKeyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut leak_source: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "date" => {
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "leak_source" => {
                            if v.is_null() {
                                continue;
                            }
                            leak_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let date = date.ok_or_else(|| M::Error::missing_field("date"))?;

                let content = LeakedKeyAttributes {
                    date,
                    leak_source,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LeakedKeyAttributesVisitor)
    }
}
