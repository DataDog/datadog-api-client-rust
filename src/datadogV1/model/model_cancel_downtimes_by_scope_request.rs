// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cancel downtimes according to scope.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CancelDowntimesByScopeRequest {
    /// The scope(s) to which the downtime applies and must be in `key:value` format. For example, `host:app2`.
    /// Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
    /// The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CancelDowntimesByScopeRequest {
    pub fn new(scope: String) -> CancelDowntimesByScopeRequest {
        CancelDowntimesByScopeRequest {
            scope,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CancelDowntimesByScopeRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CancelDowntimesByScopeRequestVisitor;
        impl<'a> Visitor<'a> for CancelDowntimesByScopeRequestVisitor {
            type Value = CancelDowntimesByScopeRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut scope: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = CancelDowntimesByScopeRequest { scope, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CancelDowntimesByScopeRequestVisitor)
    }
}
