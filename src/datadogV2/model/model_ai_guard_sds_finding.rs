// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A sensitive data finding detected by the SDS scanner.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardSdsFinding {
    /// The category of sensitive data detected.
    #[serde(rename = "category")]
    pub category: String,
    /// The location of a sensitive data match within the evaluated request.
    #[serde(rename = "location")]
    pub location: crate::datadogV2::model::AIGuardSdsFindingLocation,
    /// The human-readable name of the SDS rule that triggered.
    #[serde(rename = "rule_display_name")]
    pub rule_display_name: String,
    /// The tag identifier of the SDS rule that triggered.
    #[serde(rename = "rule_tag")]
    pub rule_tag: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardSdsFinding {
    pub fn new(
        category: String,
        location: crate::datadogV2::model::AIGuardSdsFindingLocation,
        rule_display_name: String,
        rule_tag: String,
    ) -> AIGuardSdsFinding {
        AIGuardSdsFinding {
            category,
            location,
            rule_display_name,
            rule_tag,
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

impl<'de> Deserialize<'de> for AIGuardSdsFinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardSdsFindingVisitor;
        impl<'a> Visitor<'a> for AIGuardSdsFindingVisitor {
            type Value = AIGuardSdsFinding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut location: Option<crate::datadogV2::model::AIGuardSdsFindingLocation> = None;
                let mut rule_display_name: Option<String> = None;
                let mut rule_tag: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_display_name" => {
                            rule_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_tag" => {
                            rule_tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let location = location.ok_or_else(|| M::Error::missing_field("location"))?;
                let rule_display_name = rule_display_name
                    .ok_or_else(|| M::Error::missing_field("rule_display_name"))?;
                let rule_tag = rule_tag.ok_or_else(|| M::Error::missing_field("rule_tag"))?;

                let content = AIGuardSdsFinding {
                    category,
                    location,
                    rule_display_name,
                    rule_tag,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardSdsFindingVisitor)
    }
}
