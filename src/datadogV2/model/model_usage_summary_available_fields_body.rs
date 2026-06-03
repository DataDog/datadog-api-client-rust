// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Available-fields data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSummaryAvailableFieldsBody {
    /// The lists of field names returned by `GET /api/v1/usage/summary` at each
    /// of its three response levels. Each list contains every key the data endpoint
    /// emits—both typed fields declared in the OpenAPI spec and untyped keys
    /// exposed through `additionalProperties`.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::UsageSummaryAvailableFieldsAttributes>,
    /// The identifier for the discovery scope. Always `"all"`.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of available-fields data.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::UsageSummaryAvailableFieldsType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSummaryAvailableFieldsBody {
    pub fn new() -> UsageSummaryAvailableFieldsBody {
        UsageSummaryAvailableFieldsBody {
            attributes: None,
            id: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::UsageSummaryAvailableFieldsAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::UsageSummaryAvailableFieldsType,
    ) -> Self {
        self.type_ = Some(value);
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

impl Default for UsageSummaryAvailableFieldsBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSummaryAvailableFieldsBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSummaryAvailableFieldsBodyVisitor;
        impl<'a> Visitor<'a> for UsageSummaryAvailableFieldsBodyVisitor {
            type Value = UsageSummaryAvailableFieldsBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::UsageSummaryAvailableFieldsAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::UsageSummaryAvailableFieldsType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::UsageSummaryAvailableFieldsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageSummaryAvailableFieldsBody {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSummaryAvailableFieldsBodyVisitor)
    }
}
