// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about the element used for a step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileStepParamsElement {
    /// Context of the element.
    #[serde(rename = "context")]
    pub context: Option<String>,
    /// Type of the context that the element is in.
    #[serde(rename = "contextType")]
    pub context_type: Option<crate::datadogV1::model::SyntheticsMobileStepParamsElementContextType>,
    /// Description of the element.
    #[serde(rename = "elementDescription")]
    pub element_description: Option<String>,
    /// Multi-locator to find the element.
    #[serde(rename = "multiLocator")]
    pub multi_locator: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Position of the action relative to the element.
    #[serde(rename = "relativePosition")]
    pub relative_position:
        Option<crate::datadogV1::model::SyntheticsMobileStepParamsElementRelativePosition>,
    /// Text content of the element.
    #[serde(rename = "textContent")]
    pub text_content: Option<String>,
    /// User locator to find the element.
    #[serde(rename = "userLocator")]
    pub user_locator: Option<crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocator>,
    /// Name of the view of the element.
    #[serde(rename = "viewName")]
    pub view_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileStepParamsElement {
    pub fn new() -> SyntheticsMobileStepParamsElement {
        SyntheticsMobileStepParamsElement {
            context: None,
            context_type: None,
            element_description: None,
            multi_locator: None,
            relative_position: None,
            text_content: None,
            user_locator: None,
            view_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn context(mut self, value: String) -> Self {
        self.context = Some(value);
        self
    }

    pub fn context_type(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsElementContextType,
    ) -> Self {
        self.context_type = Some(value);
        self
    }

    pub fn element_description(mut self, value: String) -> Self {
        self.element_description = Some(value);
        self
    }

    pub fn multi_locator(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.multi_locator = Some(value);
        self
    }

    pub fn relative_position(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsElementRelativePosition,
    ) -> Self {
        self.relative_position = Some(value);
        self
    }

    pub fn text_content(mut self, value: String) -> Self {
        self.text_content = Some(value);
        self
    }

    pub fn user_locator(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocator,
    ) -> Self {
        self.user_locator = Some(value);
        self
    }

    pub fn view_name(mut self, value: String) -> Self {
        self.view_name = Some(value);
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

impl Default for SyntheticsMobileStepParamsElement {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepParamsElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileStepParamsElementVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileStepParamsElementVisitor {
            type Value = SyntheticsMobileStepParamsElement;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut context: Option<String> = None;
                let mut context_type: Option<
                    crate::datadogV1::model::SyntheticsMobileStepParamsElementContextType,
                > = None;
                let mut element_description: Option<String> = None;
                let mut multi_locator: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut relative_position: Option<
                    crate::datadogV1::model::SyntheticsMobileStepParamsElementRelativePosition,
                > = None;
                let mut text_content: Option<String> = None;
                let mut user_locator: Option<
                    crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocator,
                > = None;
                let mut view_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "contextType" => {
                            if v.is_null() {
                                continue;
                            }
                            context_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _context_type) = context_type {
                                match _context_type {
                                    crate::datadogV1::model::SyntheticsMobileStepParamsElementContextType::UnparsedObject(_context_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "elementDescription" => {
                            if v.is_null() {
                                continue;
                            }
                            element_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multiLocator" => {
                            if v.is_null() {
                                continue;
                            }
                            multi_locator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relativePosition" => {
                            if v.is_null() {
                                continue;
                            }
                            relative_position =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "textContent" => {
                            if v.is_null() {
                                continue;
                            }
                            text_content =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userLocator" => {
                            if v.is_null() {
                                continue;
                            }
                            user_locator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "viewName" => {
                            if v.is_null() {
                                continue;
                            }
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileStepParamsElement {
                    context,
                    context_type,
                    element_description,
                    multi_locator,
                    relative_position,
                    text_content,
                    user_locator,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileStepParamsElementVisitor)
    }
}
