// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `Connection` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Connection {
    /// The `Connection` `connectionId`.
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// The `Connection` `label`.
    #[serde(rename = "label")]
    pub label: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Connection {
    pub fn new(connection_id: String, label: String) -> Connection {
        Connection {
            connection_id,
            label,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for Connection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConnectionVisitor;
        impl<'a> Visitor<'a> for ConnectionVisitor {
            type Value = Connection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connection_id: Option<String> = None;
                let mut label: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connectionId" => {
                            connection_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let connection_id =
                    connection_id.ok_or_else(|| M::Error::missing_field("connection_id"))?;
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;

                let content = Connection {
                    connection_id,
                    label,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConnectionVisitor)
    }
}
