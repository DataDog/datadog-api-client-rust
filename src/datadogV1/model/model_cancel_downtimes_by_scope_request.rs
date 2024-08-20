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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CancelDowntimesByScopeRequest {
    pub fn new(scope: String) -> CancelDowntimesByScopeRequest {
        CancelDowntimesByScopeRequest {
            scope,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = CancelDowntimesByScopeRequest {
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CancelDowntimesByScopeRequestVisitor)
    }
}
