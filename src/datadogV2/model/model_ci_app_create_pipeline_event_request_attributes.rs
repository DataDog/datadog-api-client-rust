// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the pipeline event to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppCreatePipelineEventRequestAttributes {
    /// The Datadog environment.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// The name of the CI provider. By default, this is "custom".
    #[serde(rename = "provider_name")]
    pub provider_name: Option<String>,
    /// Details of the CI pipeline event.
    #[serde(rename = "resource")]
    pub resource: crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource,
    /// If the CI provider is SaaS, use this to differentiate between instances.
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppCreatePipelineEventRequestAttributes {
    pub fn new(
        resource: crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource,
    ) -> CIAppCreatePipelineEventRequestAttributes {
        CIAppCreatePipelineEventRequestAttributes {
            env: None,
            provider_name: None,
            resource,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn provider_name(mut self, value: String) -> Self {
        self.provider_name = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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

impl<'de> Deserialize<'de> for CIAppCreatePipelineEventRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppCreatePipelineEventRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CIAppCreatePipelineEventRequestAttributesVisitor {
            type Value = CIAppCreatePipelineEventRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut env: Option<String> = None;
                let mut provider_name: Option<String> = None;
                let mut resource: Option<
                    crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource,
                > = None;
                let mut service: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_name" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource" => {
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _resource) = resource {
                                match _resource {
                                    crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource::UnparsedObject(_resource) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let resource = resource.ok_or_else(|| M::Error::missing_field("resource"))?;

                let content = CIAppCreatePipelineEventRequestAttributes {
                    env,
                    provider_name,
                    resource,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppCreatePipelineEventRequestAttributesVisitor)
    }
}
