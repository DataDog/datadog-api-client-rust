// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relative timeframe.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookRelativeTime {
    /// The available timeframes depend on the widget you are using.
    #[serde(rename = "live_span")]
    pub live_span: crate::datadogV1::model::WidgetLiveSpan,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookRelativeTime {
    pub fn new(live_span: crate::datadogV1::model::WidgetLiveSpan) -> NotebookRelativeTime {
        NotebookRelativeTime {
            live_span,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookRelativeTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookRelativeTimeVisitor;
        impl<'a> Visitor<'a> for NotebookRelativeTimeVisitor {
            type Value = NotebookRelativeTime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut live_span: Option<crate::datadogV1::model::WidgetLiveSpan> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "live_span" => {
                            live_span = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _live_span) = live_span {
                                match _live_span {
                                    crate::datadogV1::model::WidgetLiveSpan::UnparsedObject(
                                        _live_span,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let live_span = live_span.ok_or_else(|| M::Error::missing_field("live_span"))?;

                let content = NotebookRelativeTime {
                    live_span,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookRelativeTimeVisitor)
    }
}
