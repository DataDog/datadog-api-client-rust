// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema for queue entities
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3Queue {
    /// The schema version of entity type. The field is known as schema-version in the previous version
    #[serde(rename = "apiVersion")]
    pub api_version: crate::datadogV2::model::EntityV3APIVersion,
    /// Datadog product integrations for the datastore entity
    #[serde(rename = "datadog")]
    pub datadog: Option<crate::datadogV2::model::EntityV3QueueDatadog>,
    /// Custom extensions. This is the free-formed field to send client side metadata. No Datadog features are affected by this field.
    #[serde(rename = "extensions")]
    pub extensions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A base schema for defining third party integrations
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::EntityV3Integrations>,
    /// The definition of Entity V3 Queue Kind object.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV2::model::EntityV3QueueKind,
    /// The definition of Entity V3 Metadata object.
    #[serde(rename = "metadata")]
    pub metadata: crate::datadogV2::model::EntityV3Metadata,
    /// The definition of Entity V3 Queue Spec object.
    #[serde(rename = "spec")]
    pub spec: Option<crate::datadogV2::model::EntityV3QueueSpec>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3Queue {
    pub fn new(
        api_version: crate::datadogV2::model::EntityV3APIVersion,
        kind: crate::datadogV2::model::EntityV3QueueKind,
        metadata: crate::datadogV2::model::EntityV3Metadata,
    ) -> EntityV3Queue {
        EntityV3Queue {
            api_version,
            datadog: None,
            extensions: None,
            integrations: None,
            kind,
            metadata,
            spec: None,
            _unparsed: false,
        }
    }

    pub fn datadog(mut self, value: crate::datadogV2::model::EntityV3QueueDatadog) -> Self {
        self.datadog = Some(value);
        self
    }

    pub fn extensions(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn integrations(mut self, value: crate::datadogV2::model::EntityV3Integrations) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn spec(mut self, value: crate::datadogV2::model::EntityV3QueueSpec) -> Self {
        self.spec = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EntityV3Queue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3QueueVisitor;
        impl<'a> Visitor<'a> for EntityV3QueueVisitor {
            type Value = EntityV3Queue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_version: Option<crate::datadogV2::model::EntityV3APIVersion> = None;
                let mut datadog: Option<crate::datadogV2::model::EntityV3QueueDatadog> = None;
                let mut extensions: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut integrations: Option<crate::datadogV2::model::EntityV3Integrations> = None;
                let mut kind: Option<crate::datadogV2::model::EntityV3QueueKind> = None;
                let mut metadata: Option<crate::datadogV2::model::EntityV3Metadata> = None;
                let mut spec: Option<crate::datadogV2::model::EntityV3QueueSpec> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apiVersion" => {
                            api_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _api_version) = api_version {
                                match _api_version {
                                    crate::datadogV2::model::EntityV3APIVersion::UnparsedObject(
                                        _api_version,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "datadog" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extensions" => {
                            if v.is_null() {
                                continue;
                            }
                            extensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::EntityV3QueueKind::UnparsedObject(
                                        _kind,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spec" => {
                            if v.is_null() {
                                continue;
                            }
                            spec = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let api_version =
                    api_version.ok_or_else(|| M::Error::missing_field("api_version"))?;
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;

                let content = EntityV3Queue {
                    api_version,
                    datadog,
                    extensions,
                    integrations,
                    kind,
                    metadata,
                    spec,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3QueueVisitor)
    }
}
