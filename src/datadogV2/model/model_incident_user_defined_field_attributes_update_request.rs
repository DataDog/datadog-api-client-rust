// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an incident user-defined field. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserDefinedFieldAttributesUpdateRequest {
    /// The section in which the field appears: "what_happened" or "why_it_happened". When null, the field appears in the Attributes section.
    #[serde(
        rename = "category",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub category: Option<Option<crate::datadogV2::model::IncidentUserDefinedFieldCategory>>,
    /// The lifecycle stage at which the app prompts users to fill out this field. Cannot be set on required fields.
    #[serde(
        rename = "collected",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub collected: Option<Option<crate::datadogV2::model::IncidentUserDefinedFieldCollected>>,
    /// The default value for the field. Must be one of the valid values when valid_values is set.
    #[serde(
        rename = "default_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_value: Option<Option<String>>,
    /// The human-readable name shown in the UI.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// A decimal string representing the field's display order in the UI.
    #[serde(
        rename = "ordinal",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ordinal: Option<Option<String>>,
    /// When true, users must fill out this field on incidents.
    #[serde(
        rename = "required",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub required: Option<Option<bool>>,
    /// The list of allowed values for dropdown and multiselect fields. Limited to 1000 values.
    #[serde(
        rename = "valid_values",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub valid_values:
        Option<Option<Vec<crate::datadogV2::model::IncidentUserDefinedFieldValidValue>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserDefinedFieldAttributesUpdateRequest {
    pub fn new() -> IncidentUserDefinedFieldAttributesUpdateRequest {
        IncidentUserDefinedFieldAttributesUpdateRequest {
            category: None,
            collected: None,
            default_value: None,
            display_name: None,
            ordinal: None,
            required: None,
            valid_values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(
        mut self,
        value: Option<crate::datadogV2::model::IncidentUserDefinedFieldCategory>,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn collected(
        mut self,
        value: Option<crate::datadogV2::model::IncidentUserDefinedFieldCollected>,
    ) -> Self {
        self.collected = Some(value);
        self
    }

    pub fn default_value(mut self, value: Option<String>) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn ordinal(mut self, value: Option<String>) -> Self {
        self.ordinal = Some(value);
        self
    }

    pub fn required(mut self, value: Option<bool>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn valid_values(
        mut self,
        value: Option<Vec<crate::datadogV2::model::IncidentUserDefinedFieldValidValue>>,
    ) -> Self {
        self.valid_values = Some(value);
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

impl Default for IncidentUserDefinedFieldAttributesUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldAttributesUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserDefinedFieldAttributesUpdateRequestVisitor;
        impl<'a> Visitor<'a> for IncidentUserDefinedFieldAttributesUpdateRequestVisitor {
            type Value = IncidentUserDefinedFieldAttributesUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<
                    Option<crate::datadogV2::model::IncidentUserDefinedFieldCategory>,
                > = None;
                let mut collected: Option<
                    Option<crate::datadogV2::model::IncidentUserDefinedFieldCollected>,
                > = None;
                let mut default_value: Option<Option<String>> = None;
                let mut display_name: Option<String> = None;
                let mut ordinal: Option<Option<String>> = None;
                let mut required: Option<Option<bool>> = None;
                let mut valid_values: Option<
                    Option<Vec<crate::datadogV2::model::IncidentUserDefinedFieldValidValue>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    Some(crate::datadogV2::model::IncidentUserDefinedFieldCategory::UnparsedObject(_category)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "collected" => {
                            collected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _collected) = collected {
                                match _collected {
                                    Some(crate::datadogV2::model::IncidentUserDefinedFieldCollected::UnparsedObject(_collected)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "default_value" => {
                            default_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ordinal" => {
                            ordinal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_values" => {
                            valid_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentUserDefinedFieldAttributesUpdateRequest {
                    category,
                    collected,
                    default_value,
                    display_name,
                    ordinal,
                    required,
                    valid_values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserDefinedFieldAttributesUpdateRequestVisitor)
    }
}
