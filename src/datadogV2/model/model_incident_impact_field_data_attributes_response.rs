// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an impact field in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentImpactFieldDataAttributesResponse {
    /// The display name of the impact field.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The choices for dropdown or multiselect fields.
    #[serde(rename = "field_choices")]
    pub field_choices: Option<Vec<crate::datadogV2::model::IncidentImpactFieldChoice>>,
    /// The type of an impact field.
    #[serde(rename = "field_type")]
    pub field_type: crate::datadogV2::model::IncidentImpactFieldValueType,
    /// The normalized name of the impact field.
    #[serde(rename = "name")]
    pub name: String,
    /// The tag key associated with the field.
    #[serde(
        rename = "tag_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub tag_key: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentImpactFieldDataAttributesResponse {
    pub fn new(
        display_name: String,
        field_type: crate::datadogV2::model::IncidentImpactFieldValueType,
        name: String,
    ) -> IncidentImpactFieldDataAttributesResponse {
        IncidentImpactFieldDataAttributesResponse {
            display_name,
            field_choices: None,
            field_type,
            name,
            tag_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn field_choices(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentImpactFieldChoice>,
    ) -> Self {
        self.field_choices = Some(value);
        self
    }

    pub fn tag_key(mut self, value: Option<String>) -> Self {
        self.tag_key = Some(value);
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

impl<'de> Deserialize<'de> for IncidentImpactFieldDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentImpactFieldDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentImpactFieldDataAttributesResponseVisitor {
            type Value = IncidentImpactFieldDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_name: Option<String> = None;
                let mut field_choices: Option<
                    Vec<crate::datadogV2::model::IncidentImpactFieldChoice>,
                > = None;
                let mut field_type: Option<crate::datadogV2::model::IncidentImpactFieldValueType> =
                    None;
                let mut name: Option<String> = None;
                let mut tag_key: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_choices" => {
                            if v.is_null() {
                                continue;
                            }
                            field_choices =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_type" => {
                            field_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _field_type) = field_type {
                                match _field_type {
                                    crate::datadogV2::model::IncidentImpactFieldValueType::UnparsedObject(_field_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let field_type = field_type.ok_or_else(|| M::Error::missing_field("field_type"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = IncidentImpactFieldDataAttributesResponse {
                    display_name,
                    field_choices,
                    field_type,
                    name,
                    tag_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentImpactFieldDataAttributesResponseVisitor)
    }
}
