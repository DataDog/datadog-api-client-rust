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
pub struct AudienceFilters {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV1_20270101::model::AudienceFiltersAccountsItems>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "segments")]
    pub segments: Option<Vec<crate::datadogV1_20270101::model::AudienceFiltersSegmentsItems>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "users")]
    pub users: Option<Vec<crate::datadogV1_20270101::model::AudienceFiltersUsersItems>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AudienceFilters {
    pub fn new() -> AudienceFilters {
        AudienceFilters {
            accounts: None,
            segments: None,
            users: None,
            _unparsed: false,
        }
    }
    
    pub fn accounts(mut self, value: Vec<crate::datadogV1_20270101::model::AudienceFiltersAccountsItems>) -> Self {
        self.accounts = Some(value);
        self
    }
    
    pub fn segments(mut self, value: Vec<crate::datadogV1_20270101::model::AudienceFiltersSegmentsItems>) -> Self {
        self.segments = Some(value);
        self
    }
    
    pub fn users(mut self, value: Vec<crate::datadogV1_20270101::model::AudienceFiltersUsersItems>) -> Self {
        self.users = Some(value);
        self
    }
    
}


impl Default for AudienceFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AudienceFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AudienceFiltersVisitor;
        impl<'a> Visitor<'a> for AudienceFiltersVisitor {
            type Value = AudienceFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<Vec<crate::datadogV1_20270101::model::AudienceFiltersAccountsItems>> = None;
                let mut segments: Option<Vec<crate::datadogV1_20270101::model::AudienceFiltersSegmentsItems>> = None;
                let mut users: Option<Vec<crate::datadogV1_20270101::model::AudienceFiltersUsersItems>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = AudienceFilters {
                    accounts,
                    segments,
                    users,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AudienceFiltersVisitor)
    }
}
