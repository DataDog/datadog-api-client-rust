// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A specific monitor and the groups to evaluate for it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleOptionsMonitorId {
    /// The exact monitor group names to evaluate. An empty array evaluates all groups.
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
    /// The monitor's decimal ID.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleOptionsMonitorId {
    pub fn new(groups: Vec<String>, id: String) -> DeploymentRuleOptionsMonitorId {
        DeploymentRuleOptionsMonitorId {
            groups,
            id,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DeploymentRuleOptionsMonitorId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleOptionsMonitorIdVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleOptionsMonitorIdVisitor {
            type Value = DeploymentRuleOptionsMonitorId;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut groups: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "groups" => {
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let groups = groups.ok_or_else(|| M::Error::missing_field("groups"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = DeploymentRuleOptionsMonitorId {
                    groups,
                    id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleOptionsMonitorIdVisitor)
    }
}
