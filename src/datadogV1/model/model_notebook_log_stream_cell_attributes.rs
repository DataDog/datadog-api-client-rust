// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a notebook `log_stream` cell.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookLogStreamCellAttributes {
    /// The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::LogStreamWidgetDefinition,
    /// The size of the graph.
    #[serde(rename = "graph_size")]
    pub graph_size: Option<crate::datadogV1::model::NotebookGraphSize>,
    /// Timeframe for the notebook cell. When 'null', the notebook global time is used.
    #[serde(rename = "time", default, with = "::serde_with::rust::double_option")]
    pub time: Option<Option<crate::datadogV1::model::NotebookCellTime>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookLogStreamCellAttributes {
    pub fn new(
        definition: crate::datadogV1::model::LogStreamWidgetDefinition,
    ) -> NotebookLogStreamCellAttributes {
        NotebookLogStreamCellAttributes {
            definition,
            graph_size: None,
            time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn graph_size(mut self, value: crate::datadogV1::model::NotebookGraphSize) -> Self {
        self.graph_size = Some(value);
        self
    }

    pub fn time(mut self, value: Option<crate::datadogV1::model::NotebookCellTime>) -> Self {
        self.time = Some(value);
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

impl<'de> Deserialize<'de> for NotebookLogStreamCellAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookLogStreamCellAttributesVisitor;
        impl<'a> Visitor<'a> for NotebookLogStreamCellAttributesVisitor {
            type Value = NotebookLogStreamCellAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<crate::datadogV1::model::LogStreamWidgetDefinition> =
                    None;
                let mut graph_size: Option<crate::datadogV1::model::NotebookGraphSize> = None;
                let mut time: Option<Option<crate::datadogV1::model::NotebookCellTime>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "graph_size" => {
                            if v.is_null() {
                                continue;
                            }
                            graph_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _graph_size) = graph_size {
                                match _graph_size {
                                    crate::datadogV1::model::NotebookGraphSize::UnparsedObject(
                                        _graph_size,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "time" => {
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time) = time {
                                match _time {
                                    Some(
                                        crate::datadogV1::model::NotebookCellTime::UnparsedObject(
                                            _time,
                                        ),
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;

                let content = NotebookLogStreamCellAttributes {
                    definition,
                    graph_size,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookLogStreamCellAttributesVisitor)
    }
}
