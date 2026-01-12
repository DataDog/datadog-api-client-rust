// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `google_pubsub` source ingests logs from a Google Cloud Pub/Sub subscription.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGooglePubSubSource {
    /// GCP credentials used to authenticate with Google Cloud Storage.
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth>,
    /// The decoding format used to interpret incoming logs.
    #[serde(rename = "decoding")]
    pub decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// The GCP project ID that owns the Pub/Sub subscription.
    #[serde(rename = "project")]
    pub project: String,
    /// The Pub/Sub subscription name from which messages are consumed.
    #[serde(rename = "subscription")]
    pub subscription: String,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The source type. The value should always be `google_pubsub`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineGooglePubSubSourceType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGooglePubSubSource {
    pub fn new(
        decoding: crate::datadogV2::model::ObservabilityPipelineDecoding,
        id: String,
        project: String,
        subscription: String,
        type_: crate::datadogV2::model::ObservabilityPipelineGooglePubSubSourceType,
    ) -> ObservabilityPipelineGooglePubSubSource {
        ObservabilityPipelineGooglePubSubSource {
            auth: None,
            decoding,
            id,
            project,
            subscription,
            tls: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(mut self, value: crate::datadogV2::model::ObservabilityPipelineGcpAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGooglePubSubSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGooglePubSubSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGooglePubSubSourceVisitor {
            type Value = ObservabilityPipelineGooglePubSubSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth> = None;
                let mut decoding: Option<crate::datadogV2::model::ObservabilityPipelineDecoding> =
                    None;
                let mut id: Option<String> = None;
                let mut project: Option<String> = None;
                let mut subscription: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineGooglePubSubSourceType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decoding" => {
                            decoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _decoding) = decoding {
                                match _decoding {
                                    crate::datadogV2::model::ObservabilityPipelineDecoding::UnparsedObject(_decoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project" => {
                            project = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscription" => {
                            subscription =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineGooglePubSubSourceType::UnparsedObject(_type_) => {
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
                let decoding = decoding.ok_or_else(|| M::Error::missing_field("decoding"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let project = project.ok_or_else(|| M::Error::missing_field("project"))?;
                let subscription =
                    subscription.ok_or_else(|| M::Error::missing_field("subscription"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineGooglePubSubSource {
                    auth,
                    decoding,
                    id,
                    project,
                    subscription,
                    tls,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGooglePubSubSourceVisitor)
    }
}
