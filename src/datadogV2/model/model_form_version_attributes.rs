// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormVersionAttributes {
    /// The data definition for the form.
    #[serde(rename = "data_definition")]
    pub data_definition: std::collections::BTreeMap<String, serde_json::Value>,
    /// The state of the form version.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::FormVersionState>,
    /// The UI definition for the form.
    #[serde(rename = "ui_definition")]
    pub ui_definition: std::collections::BTreeMap<String, serde_json::Value>,
    /// Parameters for upserting a form version.
    #[serde(rename = "upsert_params")]
    pub upsert_params: crate::datadogV2::model::FormVersionUpsertParams,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormVersionAttributes {
    pub fn new(
        data_definition: std::collections::BTreeMap<String, serde_json::Value>,
        ui_definition: std::collections::BTreeMap<String, serde_json::Value>,
        upsert_params: crate::datadogV2::model::FormVersionUpsertParams,
    ) -> FormVersionAttributes {
        FormVersionAttributes {
            data_definition,
            state: None,
            ui_definition,
            upsert_params,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn state(mut self, value: crate::datadogV2::model::FormVersionState) -> Self {
        self.state = Some(value);
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

impl<'de> Deserialize<'de> for FormVersionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormVersionAttributesVisitor;
        impl<'a> Visitor<'a> for FormVersionAttributesVisitor {
            type Value = FormVersionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_definition: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut state: Option<crate::datadogV2::model::FormVersionState> = None;
                let mut ui_definition: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut upsert_params: Option<crate::datadogV2::model::FormVersionUpsertParams> =
                    None;
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
                            if v.is_null() {
                                continue;
                            }
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
                let ui_definition =
                    ui_definition.ok_or_else(|| M::Error::missing_field("ui_definition"))?;
                let upsert_params =
                    upsert_params.ok_or_else(|| M::Error::missing_field("upsert_params"))?;

                let content = FormVersionAttributes {
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

        deserializer.deserialize_any(FormVersionAttributesVisitor)
    }
}
