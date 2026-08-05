// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating a degradation template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateDegradationTemplateRequestDataAttributes {
    /// The components affected by a degradation created from this template.
    #[serde(rename = "components_affected")]
    pub components_affected: Option<Vec<crate::datadogV2::model::CreateDegradationTemplateRequestDataAttributesComponentsAffectedItems>>,
    /// The title used for a degradation created from this template.
    #[serde(rename = "degradation_title")]
    pub degradation_title: Option<String>,
    /// The name of the degradation template.
    #[serde(rename = "name")]
    pub name: String,
    /// The pre-filled updates for a degradation created from this template.
    #[serde(rename = "updates")]
    pub updates: Option<Vec<crate::datadogV2::model::CreateDegradationTemplateRequestDataAttributesUpdatesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CreateDegradationTemplateRequestDataAttributes {
    pub fn new(name: String) -> CreateDegradationTemplateRequestDataAttributes {
        CreateDegradationTemplateRequestDataAttributes {
            components_affected: None,
            degradation_title: None,
            name,
            updates: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn components_affected(
        mut self,
        value: Vec<crate::datadogV2::model::CreateDegradationTemplateRequestDataAttributesComponentsAffectedItems>,
    ) -> Self {
        self.components_affected = Some(value);
        self
    }

    pub fn degradation_title(mut self, value: String) -> Self {
        self.degradation_title = Some(value);
        self
    }

    pub fn updates(
        mut self,
        value: Vec<
            crate::datadogV2::model::CreateDegradationTemplateRequestDataAttributesUpdatesItems,
        >,
    ) -> Self {
        self.updates = Some(value);
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

impl<'de> Deserialize<'de> for CreateDegradationTemplateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateDegradationTemplateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateDegradationTemplateRequestDataAttributesVisitor {
            type Value = CreateDegradationTemplateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components_affected: Option<Vec<crate::datadogV2::model::CreateDegradationTemplateRequestDataAttributesComponentsAffectedItems>> = None;
                let mut degradation_title: Option<String> = None;
                let mut name: Option<String> = None;
                let mut updates: Option<Vec<crate::datadogV2::model::CreateDegradationTemplateRequestDataAttributesUpdatesItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components_affected" => {
                            if v.is_null() {
                                continue;
                            }
                            components_affected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "degradation_title" => {
                            if v.is_null() {
                                continue;
                            }
                            degradation_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updates" => {
                            if v.is_null() {
                                continue;
                            }
                            updates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CreateDegradationTemplateRequestDataAttributes {
                    components_affected,
                    degradation_title,
                    name,
                    updates,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateDegradationTemplateRequestDataAttributesVisitor)
    }
}
