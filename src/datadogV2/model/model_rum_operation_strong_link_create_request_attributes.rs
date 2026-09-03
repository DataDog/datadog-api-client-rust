// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a RUM operation link.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMOperationStrongLinkCreateRequestAttributes {
    /// The RUM application ID used when creating a stub operation from `operation_name`.
    #[serde(rename = "application_id")]
    pub application_id: Option<uuid::Uuid>,
    /// A description of the link.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The unique identifier of the journey to link.
    #[serde(rename = "feature_id")]
    pub feature_id: String,
    /// The unique identifier of the RUM operation to link. Either `operation_id` or
    /// `operation_name` is required.
    #[serde(rename = "operation_id")]
    pub operation_id: Option<String>,
    /// The name of the RUM operation to link. Either `operation_id` or `operation_name` is
    /// required. If no operation with this name exists, a stub operation is created.
    #[serde(rename = "operation_name")]
    pub operation_name: Option<String>,
    /// The status of a RUM operation link.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::RUMOperationStrongLinkStatus>,
    /// A list of tags associated with the link.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMOperationStrongLinkCreateRequestAttributes {
    pub fn new(feature_id: String) -> RUMOperationStrongLinkCreateRequestAttributes {
        RUMOperationStrongLinkCreateRequestAttributes {
            application_id: None,
            description: None,
            feature_id,
            operation_id: None,
            operation_name: None,
            status: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: uuid::Uuid) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn operation_id(mut self, value: String) -> Self {
        self.operation_id = Some(value);
        self
    }

    pub fn operation_name(mut self, value: String) -> Self {
        self.operation_name = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::RUMOperationStrongLinkStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for RUMOperationStrongLinkCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMOperationStrongLinkCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for RUMOperationStrongLinkCreateRequestAttributesVisitor {
            type Value = RUMOperationStrongLinkCreateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<uuid::Uuid> = None;
                let mut description: Option<Option<String>> = None;
                let mut feature_id: Option<String> = None;
                let mut operation_id: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::RUMOperationStrongLinkStatus> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_id" => {
                            feature_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_id" => {
                            if v.is_null() {
                                continue;
                            }
                            operation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_name" => {
                            if v.is_null() {
                                continue;
                            }
                            operation_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::RUMOperationStrongLinkStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let feature_id = feature_id.ok_or_else(|| M::Error::missing_field("feature_id"))?;

                let content = RUMOperationStrongLinkCreateRequestAttributes {
                    application_id,
                    description,
                    feature_id,
                    operation_id,
                    operation_name,
                    status,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMOperationStrongLinkCreateRequestAttributesVisitor)
    }
}
