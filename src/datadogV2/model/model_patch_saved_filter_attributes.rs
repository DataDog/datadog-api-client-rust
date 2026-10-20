// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fields to update. At least one of `name`, `description`, or `targeting_rules` must have a non-null value. Omitted or null fields are preserved. The creation type cannot be changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchSavedFilterAttributes {
    /// The new description. Send an empty string to clear the description. If omitted or null, the existing description is preserved.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The `updated_at` value returned by a previous read, as an RFC 3339 timestamp. If it differs from the stored update time, truncated to seconds, the request returns a 409 response. For metadata-only updates, this check does not prevent all concurrent writes. This field alone does not constitute an update.
    #[serde(
        rename = "last_updated_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_updated_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The new name of the saved filter. Must not be empty.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// The complete replacement targeting rules, combined with OR. Conditions within each rule are combined with AND. Replaces all existing rules. Values are not appended. Must contain at least one rule. Saved filters cannot reference other saved filters.
    #[serde(
        rename = "targeting_rules",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub targeting_rules:
        Option<Option<Vec<crate::datadogV2::model::SavedFilterTargetingRuleRequest>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchSavedFilterAttributes {
    pub fn new() -> PatchSavedFilterAttributes {
        PatchSavedFilterAttributes {
            description: None,
            last_updated_at: None,
            name: None,
            targeting_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn last_updated_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_updated_at = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Option<Vec<crate::datadogV2::model::SavedFilterTargetingRuleRequest>>,
    ) -> Self {
        self.targeting_rules = Some(value);
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

impl Default for PatchSavedFilterAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PatchSavedFilterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchSavedFilterAttributesVisitor;
        impl<'a> Visitor<'a> for PatchSavedFilterAttributesVisitor {
            type Value = PatchSavedFilterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut last_updated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut name: Option<Option<String>> = None;
                let mut targeting_rules: Option<
                    Option<Vec<crate::datadogV2::model::SavedFilterTargetingRuleRequest>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_at" => {
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            targeting_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = PatchSavedFilterAttributes {
                    description,
                    last_updated_at,
                    name,
                    targeting_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PatchSavedFilterAttributesVisitor)
    }
}
