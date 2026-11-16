// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Accounts whose users are affected by the issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactAccounts {
    /// Number of distinct accounts affected by the issue in the time window.
    #[serde(rename = "distinct_count")]
    pub distinct_count: i64,
    /// Up to five accounts with the most errors, most affected first.
    #[serde(rename = "top")]
    pub top: Vec<crate::datadogV2::model::IssueImpactAccount>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueImpactAccounts {
    pub fn new(
        distinct_count: i64,
        top: Vec<crate::datadogV2::model::IssueImpactAccount>,
    ) -> IssueImpactAccounts {
        IssueImpactAccounts {
            distinct_count,
            top,
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

impl<'de> Deserialize<'de> for IssueImpactAccounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactAccountsVisitor;
        impl<'a> Visitor<'a> for IssueImpactAccountsVisitor {
            type Value = IssueImpactAccounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut distinct_count: Option<i64> = None;
                let mut top: Option<Vec<crate::datadogV2::model::IssueImpactAccount>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "distinct_count" => {
                            distinct_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top" => {
                            top = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let distinct_count =
                    distinct_count.ok_or_else(|| M::Error::missing_field("distinct_count"))?;
                let top = top.ok_or_else(|| M::Error::missing_field("top"))?;

                let content = IssueImpactAccounts {
                    distinct_count,
                    top,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactAccountsVisitor)
    }
}
