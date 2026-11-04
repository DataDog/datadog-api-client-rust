// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for a commitments list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsListMeta {
    /// Unit metadata for a numeric metric.
    #[serde(rename = "committed_spend_unit")]
    pub committed_spend_unit: Option<crate::datadogV2::model::CommitmentsUnit>,
    /// Whether Azure cost data lacks the unused cost of some commitments, for example when an export is
    /// scoped below the billing profile. When `true`, utilization is overstated.
    #[serde(rename = "unused_cost_unavailable")]
    pub unused_cost_unavailable: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsListMeta {
    pub fn new() -> CommitmentsListMeta {
        CommitmentsListMeta {
            committed_spend_unit: None,
            unused_cost_unavailable: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn committed_spend_unit(mut self, value: crate::datadogV2::model::CommitmentsUnit) -> Self {
        self.committed_spend_unit = Some(value);
        self
    }

    pub fn unused_cost_unavailable(mut self, value: bool) -> Self {
        self.unused_cost_unavailable = Some(value);
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

impl Default for CommitmentsListMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CommitmentsListMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsListMetaVisitor;
        impl<'a> Visitor<'a> for CommitmentsListMetaVisitor {
            type Value = CommitmentsListMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut committed_spend_unit: Option<crate::datadogV2::model::CommitmentsUnit> =
                    None;
                let mut unused_cost_unavailable: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "committed_spend_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            committed_spend_unit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unused_cost_unavailable" => {
                            if v.is_null() {
                                continue;
                            }
                            unused_cost_unavailable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CommitmentsListMeta {
                    committed_spend_unit,
                    unused_cost_unavailable,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsListMetaVisitor)
    }
}
