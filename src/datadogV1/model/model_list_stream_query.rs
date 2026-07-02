// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated list stream widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListStreamQuery {
    /// Filter by assignee UUIDs. Usable only with `issue_stream`.
    #[serde(rename = "assignee_uuids")]
    pub assignee_uuids: Option<Vec<String>>,
    /// Specifies the field for logs pattern clustering. Usable only with logs_pattern_stream.
    #[serde(rename = "clustering_pattern_field_path")]
    pub clustering_pattern_field_path: Option<String>,
    /// Compute configuration for the List Stream Widget. Compute can be used only with the logs_transaction_stream (from 1 to 5 items) list stream source.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV1::model::ListStreamComputeItems>>,
    /// Source from which to query items to display in the stream. apm_issue_stream, rum_issue_stream, and logs_issue_stream are deprecated. Use issue_stream instead.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::ListStreamSource,
    /// Size to use to display an event.
    #[serde(rename = "event_size")]
    pub event_size: Option<crate::datadogV1::model::WidgetEventSize>,
    /// Group by configuration for the List Stream Widget. Group by can be used only with logs_pattern_stream (up to 4 items) or logs_transaction_stream (one group by item is required) list stream source.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::ListStreamGroupByItems>>,
    /// List of indexes.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Persona filter for the `issue_stream` data source.
    #[serde(rename = "persona")]
    pub persona: Option<crate::datadogV1::model::ListStreamIssuePersona>,
    /// Widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::WidgetFieldSort>,
    /// Filter by issue states. Usable only with `issue_stream`.
    #[serde(rename = "states")]
    pub states: Option<Vec<crate::datadogV1::model::ListStreamIssueState>>,
    /// Option for storage location. Feature in Private Beta.
    #[serde(rename = "storage")]
    pub storage: Option<String>,
    /// Filter by suspected causes. Usable only with `issue_stream`.
    #[serde(rename = "suspected_causes")]
    pub suspected_causes: Option<Vec<String>>,
    /// Filter by team handles. Usable only with `issue_stream`.
    #[serde(rename = "team_handles")]
    pub team_handles: Option<Vec<String>>,
    /// Version of the query for the logs transaction stream widget. When omitted, v1 query behavior is
    /// preserved. Set to `sequential_query` to use v2 behavior. **This feature is in Preview.**
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV1::model::ListStreamQueryVersion>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListStreamQuery {
    pub fn new(
        data_source: crate::datadogV1::model::ListStreamSource,
        query_string: String,
    ) -> ListStreamQuery {
        ListStreamQuery {
            assignee_uuids: None,
            clustering_pattern_field_path: None,
            compute: None,
            data_source,
            event_size: None,
            group_by: None,
            indexes: None,
            persona: None,
            query_string,
            sort: None,
            states: None,
            storage: None,
            suspected_causes: None,
            team_handles: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignee_uuids(mut self, value: Vec<String>) -> Self {
        self.assignee_uuids = Some(value);
        self
    }

    pub fn clustering_pattern_field_path(mut self, value: String) -> Self {
        self.clustering_pattern_field_path = Some(value);
        self
    }

    pub fn compute(mut self, value: Vec<crate::datadogV1::model::ListStreamComputeItems>) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn event_size(mut self, value: crate::datadogV1::model::WidgetEventSize) -> Self {
        self.event_size = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV1::model::ListStreamGroupByItems>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn persona(mut self, value: crate::datadogV1::model::ListStreamIssuePersona) -> Self {
        self.persona = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV1::model::WidgetFieldSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn states(mut self, value: Vec<crate::datadogV1::model::ListStreamIssueState>) -> Self {
        self.states = Some(value);
        self
    }

    pub fn storage(mut self, value: String) -> Self {
        self.storage = Some(value);
        self
    }

    pub fn suspected_causes(mut self, value: Vec<String>) -> Self {
        self.suspected_causes = Some(value);
        self
    }

    pub fn team_handles(mut self, value: Vec<String>) -> Self {
        self.team_handles = Some(value);
        self
    }

    pub fn version(mut self, value: crate::datadogV1::model::ListStreamQueryVersion) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for ListStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListStreamQueryVisitor;
        impl<'a> Visitor<'a> for ListStreamQueryVisitor {
            type Value = ListStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee_uuids: Option<Vec<String>> = None;
                let mut clustering_pattern_field_path: Option<String> = None;
                let mut compute: Option<Vec<crate::datadogV1::model::ListStreamComputeItems>> =
                    None;
                let mut data_source: Option<crate::datadogV1::model::ListStreamSource> = None;
                let mut event_size: Option<crate::datadogV1::model::WidgetEventSize> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::ListStreamGroupByItems>> =
                    None;
                let mut indexes: Option<Vec<String>> = None;
                let mut persona: Option<crate::datadogV1::model::ListStreamIssuePersona> = None;
                let mut query_string: Option<String> = None;
                let mut sort: Option<crate::datadogV1::model::WidgetFieldSort> = None;
                let mut states: Option<Vec<crate::datadogV1::model::ListStreamIssueState>> = None;
                let mut storage: Option<String> = None;
                let mut suspected_causes: Option<Vec<String>> = None;
                let mut team_handles: Option<Vec<String>> = None;
                let mut version: Option<crate::datadogV1::model::ListStreamQueryVersion> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee_uuids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "clustering_pattern_field_path" => {
                            if v.is_null() {
                                continue;
                            }
                            clustering_pattern_field_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::ListStreamSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "event_size" => {
                            if v.is_null() {
                                continue;
                            }
                            event_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _event_size) = event_size {
                                match _event_size {
                                    crate::datadogV1::model::WidgetEventSize::UnparsedObject(
                                        _event_size,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "persona" => {
                            if v.is_null() {
                                continue;
                            }
                            persona = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _persona) = persona {
                                match _persona {
                                    crate::datadogV1::model::ListStreamIssuePersona::UnparsedObject(_persona) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "states" => {
                            if v.is_null() {
                                continue;
                            }
                            states = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage" => {
                            if v.is_null() {
                                continue;
                            }
                            storage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suspected_causes" => {
                            if v.is_null() {
                                continue;
                            }
                            suspected_causes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_handles" => {
                            if v.is_null() {
                                continue;
                            }
                            team_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _version) = version {
                                match _version {
                                    crate::datadogV1::model::ListStreamQueryVersion::UnparsedObject(_version) => {
                                        _unparsed = true;
                                    },
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
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = ListStreamQuery {
                    assignee_uuids,
                    clustering_pattern_field_path,
                    compute,
                    data_source,
                    event_size,
                    group_by,
                    indexes,
                    persona,
                    query_string,
                    sort,
                    states,
                    storage,
                    suspected_causes,
                    team_handles,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListStreamQueryVisitor)
    }
}
