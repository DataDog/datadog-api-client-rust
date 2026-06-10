// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating a form.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateFormDataAttributes {
    /// Whether the form accepts anonymous submissions.
    #[serde(rename = "anonymous")]
    pub anonymous: Option<bool>,
    /// A JSON Schema definition that describes the form's data fields.
    #[serde(rename = "data_definition")]
    pub data_definition: crate::datadogV2::model::FormDataDefinition,
    /// The description of the form.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the form is an IDP survey.
    #[serde(rename = "idp_survey")]
    pub idp_survey: Option<bool>,
    /// The name of the form.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether each user can only submit one response.
    #[serde(rename = "single_response")]
    pub single_response: Option<bool>,
    /// UI configuration for rendering form fields, including widget overrides, field ordering, and themes.
    #[serde(rename = "ui_definition")]
    pub ui_definition: crate::datadogV2::model::FormUiDefinition,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateFormDataAttributes {
    pub fn new(
        data_definition: crate::datadogV2::model::FormDataDefinition,
        name: String,
        ui_definition: crate::datadogV2::model::FormUiDefinition,
    ) -> CreateFormDataAttributes {
        CreateFormDataAttributes {
            anonymous: None,
            data_definition,
            description: None,
            idp_survey: None,
            name,
            single_response: None,
            ui_definition,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn anonymous(mut self, value: bool) -> Self {
        self.anonymous = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn idp_survey(mut self, value: bool) -> Self {
        self.idp_survey = Some(value);
        self
    }

    pub fn single_response(mut self, value: bool) -> Self {
        self.single_response = Some(value);
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

impl<'de> Deserialize<'de> for CreateFormDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateFormDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateFormDataAttributesVisitor {
            type Value = CreateFormDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anonymous: Option<bool> = None;
                let mut data_definition: Option<crate::datadogV2::model::FormDataDefinition> = None;
                let mut description: Option<String> = None;
                let mut idp_survey: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut single_response: Option<bool> = None;
                let mut ui_definition: Option<crate::datadogV2::model::FormUiDefinition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anonymous" => {
                            if v.is_null() {
                                continue;
                            }
                            anonymous = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_definition" => {
                            data_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "idp_survey" => {
                            if v.is_null() {
                                continue;
                            }
                            idp_survey = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "single_response" => {
                            if v.is_null() {
                                continue;
                            }
                            single_response =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ui_definition" => {
                            ui_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_definition =
                    data_definition.ok_or_else(|| M::Error::missing_field("data_definition"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let ui_definition =
                    ui_definition.ok_or_else(|| M::Error::missing_field("ui_definition"))?;

                let content = CreateFormDataAttributes {
                    anonymous,
                    data_definition,
                    description,
                    idp_survey,
                    name,
                    single_response,
                    ui_definition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateFormDataAttributesVisitor)
    }
}
