// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The supported attributes for creating a degradation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateDegradationRequestDataAttributes {
    /// The components affected by the degradation.
    #[serde(rename = "components_affected")]
    pub components_affected:
        Vec<crate::datadogV2::model::CreateDegradationRequestDataAttributesComponentsAffectedItems>,
    /// The description of the degradation.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The status of the degradation.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
    /// The title of the degradation.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateDegradationRequestDataAttributes {
    pub fn new(
        components_affected: Vec<
            crate::datadogV2::model::CreateDegradationRequestDataAttributesComponentsAffectedItems,
        >,
        status: crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
        title: String,
    ) -> CreateDegradationRequestDataAttributes {
        CreateDegradationRequestDataAttributes {
            components_affected,
            description: None,
            status,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for CreateDegradationRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateDegradationRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateDegradationRequestDataAttributesVisitor {
            type Value = CreateDegradationRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components_affected: Option<Vec<crate::datadogV2::model::CreateDegradationRequestDataAttributesComponentsAffectedItems>> = None;
                let mut description: Option<String> = None;
                let mut status: Option<
                    crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
                > = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components_affected" => {
                            components_affected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let components_affected = components_affected
                    .ok_or_else(|| M::Error::missing_field("components_affected"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = CreateDegradationRequestDataAttributes {
                    components_affected,
                    description,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateDegradationRequestDataAttributesVisitor)
    }
}
