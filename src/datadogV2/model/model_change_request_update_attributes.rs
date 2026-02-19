// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a change request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestUpdateAttributes {
    /// The plan associated with the change request.
    #[serde(rename = "change_request_plan")]
    pub change_request_plan: Option<String>,
    /// The risk level of the change request.
    #[serde(rename = "change_request_risk")]
    pub change_request_risk: Option<crate::datadogV2::model::ChangeRequestRiskLevel>,
    /// The type of the change request.
    #[serde(rename = "change_request_type")]
    pub change_request_type: Option<crate::datadogV2::model::ChangeRequestChangeType>,
    /// The planned end date of the change request.
    #[serde(rename = "end_date")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The identifier of the change request to update.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The planned start date of the change request.
    #[serde(rename = "start_date")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestUpdateAttributes {
    pub fn new() -> ChangeRequestUpdateAttributes {
        ChangeRequestUpdateAttributes {
            change_request_plan: None,
            change_request_risk: None,
            change_request_type: None,
            end_date: None,
            id: None,
            start_date: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_request_plan(mut self, value: String) -> Self {
        self.change_request_plan = Some(value);
        self
    }

    pub fn change_request_risk(
        mut self,
        value: crate::datadogV2::model::ChangeRequestRiskLevel,
    ) -> Self {
        self.change_request_risk = Some(value);
        self
    }

    pub fn change_request_type(
        mut self,
        value: crate::datadogV2::model::ChangeRequestChangeType,
    ) -> Self {
        self.change_request_type = Some(value);
        self
    }

    pub fn end_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn start_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(value);
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

impl Default for ChangeRequestUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChangeRequestUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for ChangeRequestUpdateAttributesVisitor {
            type Value = ChangeRequestUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_request_plan: Option<String> = None;
                let mut change_request_risk: Option<
                    crate::datadogV2::model::ChangeRequestRiskLevel,
                > = None;
                let mut change_request_type: Option<
                    crate::datadogV2::model::ChangeRequestChangeType,
                > = None;
                let mut end_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<String> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_request_plan" => {
                            if v.is_null() {
                                continue;
                            }
                            change_request_plan =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_request_risk" => {
                            if v.is_null() {
                                continue;
                            }
                            change_request_risk =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_request_risk) = change_request_risk {
                                match _change_request_risk {
                                    crate::datadogV2::model::ChangeRequestRiskLevel::UnparsedObject(_change_request_risk) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "change_request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            change_request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_request_type) = change_request_type {
                                match _change_request_type {
                                    crate::datadogV2::model::ChangeRequestChangeType::UnparsedObject(_change_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "end_date" => {
                            if v.is_null() {
                                continue;
                            }
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChangeRequestUpdateAttributes {
                    change_request_plan,
                    change_request_risk,
                    change_request_type,
                    end_date,
                    id,
                    start_date,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestUpdateAttributesVisitor)
    }
}
