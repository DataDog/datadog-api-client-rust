// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a notebook `distribution` cell.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookDistributionCellAttributes {
    /// The Distribution visualization is another way of showing metrics
    /// aggregated across one or several tags, such as hosts.
    /// Unlike the heat map, a distribution graph’s x-axis is quantity rather than time.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::DistributionWidgetDefinition,
    /// The size of the graph.
    #[serde(rename = "graph_size")]
    pub graph_size: Option<crate::datadogV1::model::NotebookGraphSize>,
    /// Object describing how to split the graph to display multiple visualizations per request.
    #[serde(rename = "split_by")]
    pub split_by: Option<crate::datadogV1::model::NotebookSplitBy>,
    /// Timeframe for the notebook cell. When 'null', the notebook global time is used.
    #[serde(rename = "time", default, with = "::serde_with::rust::double_option")]
    pub time: Option<Option<crate::datadogV1::model::NotebookCellTime>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookDistributionCellAttributes {
    pub fn new(
        definition: crate::datadogV1::model::DistributionWidgetDefinition,
    ) -> NotebookDistributionCellAttributes {
        NotebookDistributionCellAttributes {
            definition,
            graph_size: None,
            split_by: None,
            time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn graph_size(mut self, value: crate::datadogV1::model::NotebookGraphSize) -> Self {
        self.graph_size = Some(value);
        self
    }

    pub fn split_by(mut self, value: crate::datadogV1::model::NotebookSplitBy) -> Self {
        self.split_by = Some(value);
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

impl<'de> Deserialize<'de> for NotebookDistributionCellAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookDistributionCellAttributesVisitor;
        impl<'a> Visitor<'a> for NotebookDistributionCellAttributesVisitor {
            type Value = NotebookDistributionCellAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<crate::datadogV1::model::DistributionWidgetDefinition> =
                    None;
                let mut graph_size: Option<crate::datadogV1::model::NotebookGraphSize> = None;
                let mut split_by: Option<crate::datadogV1::model::NotebookSplitBy> = None;
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
                        "split_by" => {
                            if v.is_null() {
                                continue;
                            }
                            split_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = NotebookDistributionCellAttributes {
                    definition,
                    graph_size,
                    split_by,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookDistributionCellAttributesVisitor)
    }
}
