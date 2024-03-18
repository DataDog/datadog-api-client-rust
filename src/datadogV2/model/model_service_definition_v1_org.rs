// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Org related information about the service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV1Org {
    /// App feature this service supports.
    #[serde(rename = "application")]
    pub application: Option<String>,
    /// Team that owns the service.
    #[serde(rename = "team")]
    pub team: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV1Org {
    pub fn new() -> ServiceDefinitionV1Org {
        ServiceDefinitionV1Org {
            application: None,
            team: None,
            _unparsed: false,
        }
    }

    pub fn application(mut self, value: String) -> Self {
        self.application = Some(value);
        self
    }

    pub fn team(mut self, value: String) -> Self {
        self.team = Some(value);
        self
    }
}

impl Default for ServiceDefinitionV1Org {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionV1Org {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV1OrgVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV1OrgVisitor {
            type Value = ServiceDefinitionV1Org;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application: Option<String> = None;
                let mut team: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application" => {
                            if v.is_null() {
                                continue;
                            }
                            application =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionV1Org {
                    application,
                    team,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV1OrgVisitor)
    }
}
