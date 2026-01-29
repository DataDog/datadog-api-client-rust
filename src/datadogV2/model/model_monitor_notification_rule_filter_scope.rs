// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filters monitor notifications using a scope expression over key:value pairs with boolean logic (AND, OR, NOT).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorNotificationRuleFilterScope {
    /// A scope expression composed by key:value pairs (e.g. `service:foo`) with boolean operators (AND, OR, NOT) and parentheses for grouping.
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleFilterScope {
    pub fn new(scope: String) -> MonitorNotificationRuleFilterScope {
        MonitorNotificationRuleFilterScope {
            scope,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorNotificationRuleFilterScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleFilterScopeVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleFilterScopeVisitor {
            type Value = MonitorNotificationRuleFilterScope;

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
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = MonitorNotificationRuleFilterScope { scope, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleFilterScopeVisitor)
    }
}
