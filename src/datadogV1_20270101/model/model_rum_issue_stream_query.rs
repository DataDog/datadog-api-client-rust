// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMIssueStreamQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "assignee_uuids")]
    pub assignee_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "code_gen_fix_states")]
    pub code_gen_fix_states: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::RUMIssueStreamQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "investigation")]
    pub investigation: Option<crate::datadogV1_20270101::model::RUMIssueStreamQueryInvestigation>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "muted")]
    pub muted: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "persona")]
    pub persona: Option<crate::datadogV1_20270101::model::IssueStreamPersona>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::IssueStreamSort>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "states")]
    pub states: Option<Vec<crate::datadogV1_20270101::model::IssueStreamState>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "suspected_causes")]
    pub suspected_causes: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "team_handles")]
    pub team_handles: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RUMIssueStreamQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::RUMIssueStreamQueryDataSource, query_string: String) -> RUMIssueStreamQuery {
        RUMIssueStreamQuery {
            assignee_uuids: None,
            code_gen_fix_states: None,
            cross_org_uuids: None,
            data_source,
            indexes: None,
            investigation: None,
            muted: None,
            persona: None,
            query_string,
            sort: None,
            states: None,
            suspected_causes: None,
            team_handles: None,
            _unparsed: false,
        }
    }
    
    pub fn assignee_uuids(mut self, value: Vec<String>) -> Self {
        self.assignee_uuids = Some(value);
        self
    }
    
    pub fn code_gen_fix_states(mut self, value: Vec<String>) -> Self {
        self.code_gen_fix_states = Some(value);
        self
    }
    
    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
    
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }
    
    pub fn investigation(mut self, value: crate::datadogV1_20270101::model::RUMIssueStreamQueryInvestigation) -> Self {
        self.investigation = Some(value);
        self
    }
    
    pub fn muted(mut self, value: bool) -> Self {
        self.muted = Some(value);
        self
    }
    
    pub fn persona(mut self, value: crate::datadogV1_20270101::model::IssueStreamPersona) -> Self {
        self.persona = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::IssueStreamSort) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn states(mut self, value: Vec<crate::datadogV1_20270101::model::IssueStreamState>) -> Self {
        self.states = Some(value);
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
    
}



impl<'de> Deserialize<'de> for RUMIssueStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMIssueStreamQueryVisitor;
        impl<'a> Visitor<'a> for RUMIssueStreamQueryVisitor {
            type Value = RUMIssueStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee_uuids: Option<Vec<String>> = None;
                let mut code_gen_fix_states: Option<Vec<String>> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::RUMIssueStreamQueryDataSource> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut investigation: Option<crate::datadogV1_20270101::model::RUMIssueStreamQueryInvestigation> = None;
                let mut muted: Option<bool> = None;
                let mut persona: Option<crate::datadogV1_20270101::model::IssueStreamPersona> = None;
                let mut query_string: Option<String> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::IssueStreamSort> = None;
                let mut states: Option<Vec<crate::datadogV1_20270101::model::IssueStreamState>> = None;
                let mut suspected_causes: Option<Vec<String>> = None;
                let mut team_handles: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee_uuids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "code_gen_fix_states" => {
                            if v.is_null() {
                                continue;
                            }
                            code_gen_fix_states = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cross_org_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_org_uuids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::RUMIssueStreamQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "investigation" => {
                            if v.is_null() {
                                continue;
                            }
                            investigation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "muted" => {
                            if v.is_null() {
                                continue;
                            }
                            muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "persona" => {
                            if v.is_null() {
                                continue;
                            }
                            persona = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _persona) = persona {
                                match _persona {
                                    crate::datadogV1_20270101::model::IssueStreamPersona::UnparsedObject(_persona) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "query_string" => {
                            query_string = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "states" => {
                            if v.is_null() {
                                continue;
                            }
                            states = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "suspected_causes" => {
                            if v.is_null() {
                                continue;
                            }
                            suspected_causes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "team_handles" => {
                            if v.is_null() {
                                continue;
                            }
                            team_handles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query_string = query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = RUMIssueStreamQuery {
                    assignee_uuids,
                    code_gen_fix_states,
                    cross_org_uuids,
                    data_source,
                    indexes,
                    investigation,
                    muted,
                    persona,
                    query_string,
                    sort,
                    states,
                    suspected_causes,
                    team_handles,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMIssueStreamQueryVisitor)
    }
}
