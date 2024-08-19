// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Meta object containing the findings to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BulkMuteFindingsRequestMeta {
    /// Array of findings.
    #[serde(rename = "findings")]
    pub findings: Option<Vec<crate::datadogV2::model::BulkMuteFindingsRequestMetaFindings>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BulkMuteFindingsRequestMeta {
    pub fn new() -> BulkMuteFindingsRequestMeta {
        BulkMuteFindingsRequestMeta {
            findings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn findings(
        mut self,
        value: Vec<crate::datadogV2::model::BulkMuteFindingsRequestMetaFindings>,
    ) -> Self {
        self.findings = Some(value);
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

impl Default for BulkMuteFindingsRequestMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BulkMuteFindingsRequestMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BulkMuteFindingsRequestMetaVisitor;
        impl<'a> Visitor<'a> for BulkMuteFindingsRequestMetaVisitor {
            type Value = BulkMuteFindingsRequestMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut findings: Option<
                    Vec<crate::datadogV2::model::BulkMuteFindingsRequestMetaFindings>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "findings" => {
                            if v.is_null() {
                                continue;
                            }
                            findings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BulkMuteFindingsRequestMeta {
                    findings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BulkMuteFindingsRequestMetaVisitor)
    }
}
