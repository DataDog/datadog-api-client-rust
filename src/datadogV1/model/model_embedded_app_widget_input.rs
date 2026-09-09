// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An input passed to the embedded app.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EmbeddedAppWidgetInput {
    /// Name of the app input.
    #[serde(rename = "name")]
    pub name: String,
    /// Value of the app input. This can be a string, number, boolean, object, or a non-empty homogeneous array of those types.
    #[serde(rename = "value")]
    pub value: crate::datadogV1::model::EmbeddedAppWidgetInputValue,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EmbeddedAppWidgetInput {
    pub fn new(
        name: String,
        value: crate::datadogV1::model::EmbeddedAppWidgetInputValue,
    ) -> EmbeddedAppWidgetInput {
        EmbeddedAppWidgetInput {
            name,
            value,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for EmbeddedAppWidgetInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EmbeddedAppWidgetInputVisitor;
        impl<'a> Visitor<'a> for EmbeddedAppWidgetInputVisitor {
            type Value = EmbeddedAppWidgetInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut value: Option<crate::datadogV1::model::EmbeddedAppWidgetInputValue> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV1::model::EmbeddedAppWidgetInputValue::UnparsedObject(_value) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = EmbeddedAppWidgetInput {
                    name,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EmbeddedAppWidgetInputVisitor)
    }
}
