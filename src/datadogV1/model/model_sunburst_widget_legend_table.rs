// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration of table-based legend.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SunburstWidgetLegendTable {
    /// Whether or not to show a table legend.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SunburstWidgetLegendTableType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SunburstWidgetLegendTable {
    pub fn new(
        type_: crate::datadogV1::model::SunburstWidgetLegendTableType,
    ) -> SunburstWidgetLegendTable {
        SunburstWidgetLegendTable {
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SunburstWidgetLegendTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SunburstWidgetLegendTableVisitor;
        impl<'a> Visitor<'a> for SunburstWidgetLegendTableVisitor {
            type Value = SunburstWidgetLegendTable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1::model::SunburstWidgetLegendTableType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SunburstWidgetLegendTableType::UnparsedObject(_type_) => {
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

                let content = SunburstWidgetLegendTable { type_, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SunburstWidgetLegendTableVisitor)
    }
}
