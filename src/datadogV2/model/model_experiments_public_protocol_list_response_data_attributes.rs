// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary of the protocol and its selected subject type and primary metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolListResponseDataAttributes {
    /// Text that explains the protocol.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Display name of the protocol.
    #[serde(rename = "name")]
    pub name: String,
    /// Subject type selected by the protocol.
    #[serde(rename = "primary_metric")]
    pub primary_metric:
        Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType>,
    /// ID of the primary metric supplied by the protocol.
    #[serde(rename = "primary_metric_id")]
    pub primary_metric_id: Option<String>,
    /// Publication status of the protocol.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus,
    /// Subject type selected by the protocol.
    #[serde(rename = "subject_type")]
    pub subject_type:
        Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType>,
    /// ID of the subject type used by this configuration.
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: Option<String>,
    /// RFC3339 update time. Preserve all fractional seconds when passing this value as expected_updated_at.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPublicProtocolListResponseDataAttributes {
    pub fn new(
        name: String,
        status: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus,
        updated_at: String,
    ) -> ExperimentsPublicProtocolListResponseDataAttributes {
        ExperimentsPublicProtocolListResponseDataAttributes {
            description: None,
            name,
            primary_metric: None,
            primary_metric_id: None,
            status,
            subject_type: None,
            subject_type_id: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn primary_metric(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType,
    ) -> Self {
        self.primary_metric = Some(value);
        self
    }

    pub fn primary_metric_id(mut self, value: String) -> Self {
        self.primary_metric_id = Some(value);
        self
    }

    pub fn subject_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType,
    ) -> Self {
        self.subject_type = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: String) -> Self {
        self.subject_type_id = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsPublicProtocolListResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolListResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolListResponseDataAttributesVisitor {
            type Value = ExperimentsPublicProtocolListResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut primary_metric: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType> = None;
                let mut primary_metric_id: Option<String> = None;
                let mut status: Option<
                    crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus,
                > = None;
                let mut subject_type: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType> = None;
                let mut subject_type_id: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_metric =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric_id" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_metric_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "subject_type" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = ExperimentsPublicProtocolListResponseDataAttributes {
                    description,
                    name,
                    primary_metric,
                    primary_metric_id,
                    status,
                    subject_type,
                    subject_type_id,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPublicProtocolListResponseDataAttributesVisitor)
    }
}
