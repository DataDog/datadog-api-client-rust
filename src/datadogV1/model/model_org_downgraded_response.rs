// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Status of downgrade
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgDowngradedResponse {
    /// Information pertaining to the downgraded child organization.
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgDowngradedResponse {
    pub fn new() -> OrgDowngradedResponse {
        OrgDowngradedResponse {
            message: None,
            _unparsed: false,
        }
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}

impl Default for OrgDowngradedResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrgDowngradedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgDowngradedResponseVisitor;
        impl<'a> Visitor<'a> for OrgDowngradedResponseVisitor {
            type Value = OrgDowngradedResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrgDowngradedResponse { message, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgDowngradedResponseVisitor)
    }
}
