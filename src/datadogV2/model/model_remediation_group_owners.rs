// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Who should fix the group: the code owners of the group's findings, from their `@git.codeowners` attribute,
/// and their Datadog teams, from their `team` tag. A group with no owners has empty `items` and a `total` of 0.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupOwners {
    /// Owners of the group's open findings. `items` lists at most 5 owners, the owners of the most findings first,
    /// then by name. `total` counts every distinct owner, so the list is cut when `total` is larger than the number
    /// of items.
    #[serde(rename = "codeowners")]
    pub codeowners: crate::datadogV2::model::RemediationGroupOwnerList,
    /// Owners of the group's open findings. `items` lists at most 5 owners, the owners of the most findings first,
    /// then by name. `total` counts every distinct owner, so the list is cut when `total` is larger than the number
    /// of items.
    #[serde(rename = "teams")]
    pub teams: crate::datadogV2::model::RemediationGroupOwnerList,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupOwners {
    pub fn new(
        codeowners: crate::datadogV2::model::RemediationGroupOwnerList,
        teams: crate::datadogV2::model::RemediationGroupOwnerList,
    ) -> RemediationGroupOwners {
        RemediationGroupOwners {
            codeowners,
            teams,
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

impl<'de> Deserialize<'de> for RemediationGroupOwners {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupOwnersVisitor;
        impl<'a> Visitor<'a> for RemediationGroupOwnersVisitor {
            type Value = RemediationGroupOwners;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut codeowners: Option<crate::datadogV2::model::RemediationGroupOwnerList> =
                    None;
                let mut teams: Option<crate::datadogV2::model::RemediationGroupOwnerList> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "codeowners" => {
                            codeowners = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let codeowners = codeowners.ok_or_else(|| M::Error::missing_field("codeowners"))?;
                let teams = teams.ok_or_else(|| M::Error::missing_field("teams"))?;

                let content = RemediationGroupOwners {
                    codeowners,
                    teams,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupOwnersVisitor)
    }
}
