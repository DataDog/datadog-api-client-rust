// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An insight of the case.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseInsightsItems {
    /// Reference of the insight.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Unique identifier of the resource. For example, the unique identifier of a security finding.
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    /// Type of the resource. For example, the type of a security finding is "SECURITY_FINDING".
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseInsightsItems {
    pub fn new() -> CaseInsightsItems {
        CaseInsightsItems {
            ref_: None,
            resource_id: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ref_(mut self, value: String) -> Self {
        self.ref_ = Some(value);
        self
    }

    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for CaseInsightsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseInsightsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseInsightsItemsVisitor;
        impl<'a> Visitor<'a> for CaseInsightsItemsVisitor {
            type Value = CaseInsightsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ref_: Option<String> = None;
                let mut resource_id: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ref" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CaseInsightsItems {
                    ref_,
                    resource_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseInsightsItemsVisitor)
    }
}
