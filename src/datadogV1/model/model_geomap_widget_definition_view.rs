// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The view of the world that the map should render.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapWidgetDefinitionView {
    /// The 2-letter ISO code of a country to focus the map on. Or `WORLD`.
    #[serde(rename = "focus")]
    pub focus: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeomapWidgetDefinitionView {
    pub fn new(focus: String) -> GeomapWidgetDefinitionView {
        GeomapWidgetDefinitionView {
            focus,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for GeomapWidgetDefinitionView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapWidgetDefinitionViewVisitor;
        impl<'a> Visitor<'a> for GeomapWidgetDefinitionViewVisitor {
            type Value = GeomapWidgetDefinitionView;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut focus: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "focus" => {
                            focus = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let focus = focus.ok_or_else(|| M::Error::missing_field("focus"))?;

                let content = GeomapWidgetDefinitionView { focus, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapWidgetDefinitionViewVisitor)
    }
}
