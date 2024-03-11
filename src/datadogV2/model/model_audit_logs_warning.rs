// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Warning message indicating something that went wrong with the query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuditLogsWarning {
    /// Unique code for this type of warning.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Detailed explanation of this specific warning.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Short human-readable summary of the warning.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuditLogsWarning {
    pub fn new() -> AuditLogsWarning {
        AuditLogsWarning {
            code: None,
            detail: None,
            title: None,
            _unparsed: false,
        }
    }

    pub fn code(&mut self, value: String) -> &mut Self {
        self.code = Some(value);
        self
    }

    pub fn detail(&mut self, value: String) -> &mut Self {
        self.detail = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }
}

impl Default for AuditLogsWarning {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuditLogsWarning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuditLogsWarningVisitor;
        impl<'a> Visitor<'a> for AuditLogsWarningVisitor {
            type Value = AuditLogsWarning;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut detail: Option<String> = None;
                let mut title: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detail" => {
                            if v.is_null() {
                                continue;
                            }
                            detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AuditLogsWarning {
                    code,
                    detail,
                    title,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuditLogsWarningVisitor)
    }
}
