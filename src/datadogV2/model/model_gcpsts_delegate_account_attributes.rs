// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Your delegate account attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPSTSDelegateAccountAttributes {
    /// Your organization's Datadog principal email address.
    #[serde(rename = "delegate_account_email")]
    pub delegate_account_email: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPSTSDelegateAccountAttributes {
    pub fn new() -> GCPSTSDelegateAccountAttributes {
        GCPSTSDelegateAccountAttributes {
            delegate_account_email: None,
            _unparsed: false,
        }
    }

    pub fn delegate_account_email(mut self, value: String) -> Self {
        self.delegate_account_email = Some(value);
        self
    }
}

impl Default for GCPSTSDelegateAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPSTSDelegateAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPSTSDelegateAccountAttributesVisitor;
        impl<'a> Visitor<'a> for GCPSTSDelegateAccountAttributesVisitor {
            type Value = GCPSTSDelegateAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut delegate_account_email: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "delegate_account_email" => {
                            if v.is_null() {
                                continue;
                            }
                            delegate_account_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = GCPSTSDelegateAccountAttributes {
                    delegate_account_email,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPSTSDelegateAccountAttributesVisitor)
    }
}
