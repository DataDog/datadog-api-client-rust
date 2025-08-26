// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Widget time setting with hide incomplete cost data option.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetTimeHideIncompleteData {
    /// Whether to hide incomplete cost data in the widget.
    #[serde(rename = "hide_incomplete_cost_data")]
    pub hide_incomplete_cost_data: bool,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetTimeHideIncompleteData {
    pub fn new(hide_incomplete_cost_data: bool) -> WidgetTimeHideIncompleteData {
        WidgetTimeHideIncompleteData {
            hide_incomplete_cost_data,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for WidgetTimeHideIncompleteData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetTimeHideIncompleteDataVisitor;
        impl<'a> Visitor<'a> for WidgetTimeHideIncompleteDataVisitor {
            type Value = WidgetTimeHideIncompleteData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hide_incomplete_cost_data: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hide_incomplete_cost_data" => {
                            hide_incomplete_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let hide_incomplete_cost_data = hide_incomplete_cost_data
                    .ok_or_else(|| M::Error::missing_field("hide_incomplete_cost_data"))?;

                let content = WidgetTimeHideIncompleteData {
                    hide_incomplete_cost_data,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetTimeHideIncompleteDataVisitor)
    }
}
