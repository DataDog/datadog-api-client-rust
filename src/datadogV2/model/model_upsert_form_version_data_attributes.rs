// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating or updating a form version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertFormVersionDataAttributes {
    /// A JSON Schema definition that describes the form's data fields.
    #[serde(rename = "data_definition")]
    pub data_definition: crate::datadogV2::model::FormDataDefinition,
    /// The state of a form version.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::FormVersionState,
    /// UI configuration for rendering form fields, including widget overrides, field ordering, and themes.
    #[serde(rename = "ui_definition")]
    pub ui_definition: crate::datadogV2::model::FormUiDefinition,
    /// Concurrency control parameters for the form version upsert operation.
    #[serde(rename = "upsert_params")]
    pub upsert_params: crate::datadogV2::model::UpsertFormVersionUpsertParams,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertFormVersionDataAttributes {
    pub fn new(
        data_definition: crate::datadogV2::model::FormDataDefinition,
        state: crate::datadogV2::model::FormVersionState,
        ui_definition: crate::datadogV2::model::FormUiDefinition,
        upsert_params: crate::datadogV2::model::UpsertFormVersionUpsertParams,
    ) -> UpsertFormVersionDataAttributes {
        UpsertFormVersionDataAttributes {
            data_definition,
            state,
            ui_definition,
            upsert_params,
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

impl<'de> Deserialize<'de> for UpsertFormVersionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertFormVersionDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpsertFormVersionDataAttributesVisitor {
            type Value = UpsertFormVersionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_definition: Option<crate::datadogV2::model::FormDataDefinition> = None;
                let mut state: Option<crate::datadogV2::model::FormVersionState> = None;
                let mut ui_definition: Option<crate::datadogV2::model::FormUiDefinition> = None;
                let mut upsert_params: Option<
                    crate::datadogV2::model::UpsertFormVersionUpsertParams,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_definition" => {
                            data_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::FormVersionState::UnparsedObject(
                                        _state,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "ui_definition" => {
                            ui_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "upsert_params" => {
                            upsert_params =
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
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;
                let ui_definition =
                    ui_definition.ok_or_else(|| M::Error::missing_field("ui_definition"))?;
                let upsert_params =
                    upsert_params.ok_or_else(|| M::Error::missing_field("upsert_params"))?;

                let content = UpsertFormVersionDataAttributes {
                    data_definition,
                    state,
                    ui_definition,
                    upsert_params,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertFormVersionDataAttributesVisitor)
    }
}
