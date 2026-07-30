// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a RUM operation strong link response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMOperationStrongLinkResponseAttributes {
    /// The timestamp when the strong link was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A description of the strong link.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The unique identifier of the linked feature.
    #[serde(rename = "feature_id")]
    pub feature_id: String,
    /// The unique identifier of the linked RUM operation.
    #[serde(rename = "operation_id")]
    pub operation_id: String,
    /// The status of a RUM operation strong link.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RUMOperationStrongLinkStatus,
    /// A list of tags associated with the strong link.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The timestamp when the strong link was last updated.
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updated_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMOperationStrongLinkResponseAttributes {
    pub fn new(
        feature_id: String,
        operation_id: String,
        status: crate::datadogV2::model::RUMOperationStrongLinkStatus,
    ) -> RUMOperationStrongLinkResponseAttributes {
        RUMOperationStrongLinkResponseAttributes {
            created_at: None,
            description: None,
            feature_id,
            operation_id,
            status,
            tags: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.updated_at = Some(value);
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

impl<'de> Deserialize<'de> for RUMOperationStrongLinkResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMOperationStrongLinkResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RUMOperationStrongLinkResponseAttributesVisitor {
            type Value = RUMOperationStrongLinkResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut feature_id: Option<String> = None;
                let mut operation_id: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::RUMOperationStrongLinkStatus> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut updated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_id" => {
                            feature_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_id" => {
                            operation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
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
                let feature_id = feature_id.ok_or_else(|| M::Error::missing_field("feature_id"))?;
                let operation_id =
                    operation_id.ok_or_else(|| M::Error::missing_field("operation_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = RUMOperationStrongLinkResponseAttributes {
                    created_at,
                    description,
                    feature_id,
                    operation_id,
                    status,
                    tags,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMOperationStrongLinkResponseAttributesVisitor)
    }
}
