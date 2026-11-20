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
pub struct CompositeAudienceFiltersQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryAccountsItems>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "segments")]
    pub segments: Option<Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQuerySegmentsItems>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "users")]
    pub users: Option<Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryUsersItems>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CompositeAudienceFiltersQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryDataSource, name: String) -> CompositeAudienceFiltersQuery {
        CompositeAudienceFiltersQuery {
            accounts: None,
            data_source,
            name,
            segments: None,
            users: None,
            _unparsed: false,
        }
    }
    
    pub fn accounts(mut self, value: Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryAccountsItems>) -> Self {
        self.accounts = Some(value);
        self
    }
    
    pub fn segments(mut self, value: Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQuerySegmentsItems>) -> Self {
        self.segments = Some(value);
        self
    }
    
    pub fn users(mut self, value: Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryUsersItems>) -> Self {
        self.users = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for CompositeAudienceFiltersQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompositeAudienceFiltersQueryVisitor;
        impl<'a> Visitor<'a> for CompositeAudienceFiltersQueryVisitor {
            type Value = CompositeAudienceFiltersQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryAccountsItems>> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryDataSource> = None;
                let mut name: Option<String> = None;
                let mut segments: Option<Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQuerySegmentsItems>> = None;
                let mut users: Option<Vec<crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryUsersItems>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::CompositeAudienceFiltersQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "segments" => {
                            if v.is_null() {
                                continue;
                            }
                            segments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CompositeAudienceFiltersQuery {
                    accounts,
                    data_source,
                    name,
                    segments,
                    users,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CompositeAudienceFiltersQueryVisitor)
    }
}
