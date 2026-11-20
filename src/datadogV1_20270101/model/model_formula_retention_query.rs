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
pub struct FormulaRetentionQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "computation_scope")]
    pub computation_scope: Option<crate::datadogV1_20270101::model::ComputationScope>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1_20270101::model::RetentionCompute,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaRetentionQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "exclude_anonymous_traffic")]
    pub exclude_anonymous_traffic: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::RetentionGroupByItems>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sampling")]
    pub sampling: Option<crate::datadogV1_20270101::model::FormulaRetentionQuerySampling>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: crate::datadogV1_20270101::model::RetentionSearch,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaRetentionQuery {
    pub fn new(compute: crate::datadogV1_20270101::model::RetentionCompute, data_source: crate::datadogV1_20270101::model::FormulaRetentionQueryDataSource, name: String, search: crate::datadogV1_20270101::model::RetentionSearch) -> FormulaRetentionQuery {
        FormulaRetentionQuery {
            computation_scope: None,
            compute,
            data_source,
            exclude_anonymous_traffic: None,
            group_by: None,
            name,
            sampling: None,
            search,
            _unparsed: false,
        }
    }
    
    pub fn computation_scope(mut self, value: crate::datadogV1_20270101::model::ComputationScope) -> Self {
        self.computation_scope = Some(value);
        self
    }
    
    pub fn exclude_anonymous_traffic(mut self, value: bool) -> Self {
        self.exclude_anonymous_traffic = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::RetentionGroupByItems>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn sampling(mut self, value: crate::datadogV1_20270101::model::FormulaRetentionQuerySampling) -> Self {
        self.sampling = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaRetentionQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaRetentionQueryVisitor;
        impl<'a> Visitor<'a> for FormulaRetentionQueryVisitor {
            type Value = FormulaRetentionQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut computation_scope: Option<crate::datadogV1_20270101::model::ComputationScope> = None;
                let mut compute: Option<crate::datadogV1_20270101::model::RetentionCompute> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaRetentionQueryDataSource> = None;
                let mut exclude_anonymous_traffic: Option<bool> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::RetentionGroupByItems>> = None;
                let mut name: Option<String> = None;
                let mut sampling: Option<crate::datadogV1_20270101::model::FormulaRetentionQuerySampling> = None;
                let mut search: Option<crate::datadogV1_20270101::model::RetentionSearch> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "computation_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            computation_scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _computation_scope) = computation_scope {
                                match _computation_scope {
                                    crate::datadogV1_20270101::model::ComputationScope::UnparsedObject(_computation_scope) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaRetentionQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "exclude_anonymous_traffic" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_anonymous_traffic = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sampling" => {
                            if v.is_null() {
                                continue;
                            }
                            sampling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = FormulaRetentionQuery {
                    computation_scope,
                    compute,
                    data_source,
                    exclude_anonymous_traffic,
                    group_by,
                    name,
                    sampling,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaRetentionQueryVisitor)
    }
}
