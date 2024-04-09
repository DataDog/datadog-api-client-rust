// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The GCS archive's integration destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveIntegrationGCS {
    /// A client email.
    #[serde(rename = "client_email")]
    pub client_email: String,
    /// A project ID.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveIntegrationGCS {
    pub fn new(client_email: String) -> LogsArchiveIntegrationGCS {
        LogsArchiveIntegrationGCS {
            client_email,
            project_id: None,
            _unparsed: false,
        }
    }

    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsArchiveIntegrationGCS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveIntegrationGCSVisitor;
        impl<'a> Visitor<'a> for LogsArchiveIntegrationGCSVisitor {
            type Value = LogsArchiveIntegrationGCS;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_email: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_email" => {
                            client_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            if v.is_null() {
                                continue;
                            }
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let client_email =
                    client_email.ok_or_else(|| M::Error::missing_field("client_email"))?;

                let content = LogsArchiveIntegrationGCS {
                    client_email,
                    project_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveIntegrationGCSVisitor)
    }
}
