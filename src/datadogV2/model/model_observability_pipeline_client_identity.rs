// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Restricts which clients may connect, based on the identities in their TLS certificate. Checks run during the TLS handshake, after the client certificate chain is validated against `ca_file`, and never relax that validation.
/// Requires `verify_certificate: true` and `ca_file`. Set exactly one of `allowlist_file` or `allowlist`.
/// When set, connections are closed after a maximum age of 300 seconds so that clients removed from the allowlist are disconnected.
/// Requires Observability Pipelines Worker 2.27 or later.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineClientIdentity {
    /// An allowlist of client identities. A client is allowed when any rule matches.
    #[serde(rename = "allowlist")]
    pub allowlist: Option<crate::datadogV2::model::ObservabilityPipelineClientIdentityAllowlist>,
    /// Path to an allowlist file, relative to the Worker's data directory. The Worker reloads the file when it changes. Must not contain parent directory (`..`) references.
    #[serde(rename = "allowlist_file")]
    pub allowlist_file: Option<String>,
    /// What happens to a client that no rule allows. `enforce` rejects the TLS handshake. `audit` accepts the client and reports that it would have been rejected.
    #[serde(rename = "mode")]
    pub mode: Option<crate::datadogV2::model::ObservabilityPipelineClientIdentityMode>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineClientIdentity {
    pub fn new() -> ObservabilityPipelineClientIdentity {
        ObservabilityPipelineClientIdentity {
            allowlist: None,
            allowlist_file: None,
            mode: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allowlist(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClientIdentityAllowlist,
    ) -> Self {
        self.allowlist = Some(value);
        self
    }

    pub fn allowlist_file(mut self, value: String) -> Self {
        self.allowlist_file = Some(value);
        self
    }

    pub fn mode(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClientIdentityMode,
    ) -> Self {
        self.mode = Some(value);
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

impl Default for ObservabilityPipelineClientIdentity {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineClientIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineClientIdentityVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineClientIdentityVisitor {
            type Value = ObservabilityPipelineClientIdentity;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allowlist: Option<
                    crate::datadogV2::model::ObservabilityPipelineClientIdentityAllowlist,
                > = None;
                let mut allowlist_file: Option<String> = None;
                let mut mode: Option<
                    crate::datadogV2::model::ObservabilityPipelineClientIdentityMode,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowlist" => {
                            if v.is_null() {
                                continue;
                            }
                            allowlist = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allowlist_file" => {
                            if v.is_null() {
                                continue;
                            }
                            allowlist_file =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            if v.is_null() {
                                continue;
                            }
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineClientIdentityMode::UnparsedObject(_mode) => {
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

                let content = ObservabilityPipelineClientIdentity {
                    allowlist,
                    allowlist_file,
                    mode,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineClientIdentityVisitor)
    }
}
