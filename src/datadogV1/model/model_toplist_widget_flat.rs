// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Top list widget flat display.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ToplistWidgetFlat {
    /// Top list widget flat display type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ToplistWidgetFlatType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ToplistWidgetFlat {
    pub fn new(type_: crate::datadogV1::model::ToplistWidgetFlatType) -> ToplistWidgetFlat {
        ToplistWidgetFlat {
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for ToplistWidgetFlat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistWidgetFlatVisitor;
        impl<'a> Visitor<'a> for ToplistWidgetFlatVisitor {
            type Value = ToplistWidgetFlat;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1::model::ToplistWidgetFlatType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::ToplistWidgetFlatType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ToplistWidgetFlat { type_, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ToplistWidgetFlatVisitor)
    }
}
