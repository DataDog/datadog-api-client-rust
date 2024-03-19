// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Subscription definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationSubscription {
    /// The subscription type. Types available are `trial`, `free`, and `pro`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrganizationSubscription {
    pub fn new() -> OrganizationSubscription {
        OrganizationSubscription {
            type_: None,
            _unparsed: false,
        }
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for OrganizationSubscription {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrganizationSubscription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationSubscriptionVisitor;
        impl<'a> Visitor<'a> for OrganizationSubscriptionVisitor {
            type Value = OrganizationSubscription;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrganizationSubscription { type_, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationSubscriptionVisitor)
    }
}
