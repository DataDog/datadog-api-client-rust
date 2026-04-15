// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a group of instrumented pods targeted for SSI injection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetInstrumentedPodGroupAttributes {
    /// The SSI injection target configuration applied to the pod group.
    #[serde(rename = "applied_target")]
    pub applied_target: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The name of the applied SSI injection target.
    #[serde(rename = "applied_target_name")]
    pub applied_target_name: Option<String>,
    /// Tags injected into the pods by the Admission Controller.
    #[serde(rename = "injected_tags")]
    pub injected_tags: Option<Vec<String>>,
    /// The kind of the Kubernetes owner reference.
    #[serde(rename = "kube_ownerref_kind")]
    pub kube_ownerref_kind: Option<String>,
    /// The name of the Kubernetes owner reference (deployment, statefulset, etc.).
    #[serde(rename = "kube_ownerref_name")]
    pub kube_ownerref_name: Option<String>,
    /// Library injection annotations on the pod group.
    #[serde(rename = "lib_injection_annotations")]
    pub lib_injection_annotations: Option<Vec<String>>,
    /// The Kubernetes namespace of the pod group.
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    /// Total number of pods in the group.
    #[serde(rename = "pod_count")]
    pub pod_count: Option<i64>,
    /// Names of the individual pods in the group.
    #[serde(rename = "pod_names")]
    pub pod_names: Option<Vec<String>>,
    /// Additional tags associated with the pod group.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetInstrumentedPodGroupAttributes {
    pub fn new() -> FleetInstrumentedPodGroupAttributes {
        FleetInstrumentedPodGroupAttributes {
            applied_target: None,
            applied_target_name: None,
            injected_tags: None,
            kube_ownerref_kind: None,
            kube_ownerref_name: None,
            lib_injection_annotations: None,
            namespace: None,
            pod_count: None,
            pod_names: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn applied_target(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.applied_target = Some(value);
        self
    }

    pub fn applied_target_name(mut self, value: String) -> Self {
        self.applied_target_name = Some(value);
        self
    }

    pub fn injected_tags(mut self, value: Vec<String>) -> Self {
        self.injected_tags = Some(value);
        self
    }

    pub fn kube_ownerref_kind(mut self, value: String) -> Self {
        self.kube_ownerref_kind = Some(value);
        self
    }

    pub fn kube_ownerref_name(mut self, value: String) -> Self {
        self.kube_ownerref_name = Some(value);
        self
    }

    pub fn lib_injection_annotations(mut self, value: Vec<String>) -> Self {
        self.lib_injection_annotations = Some(value);
        self
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn pod_count(mut self, value: i64) -> Self {
        self.pod_count = Some(value);
        self
    }

    pub fn pod_names(mut self, value: Vec<String>) -> Self {
        self.pod_names = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
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

impl Default for FleetInstrumentedPodGroupAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetInstrumentedPodGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetInstrumentedPodGroupAttributesVisitor;
        impl<'a> Visitor<'a> for FleetInstrumentedPodGroupAttributesVisitor {
            type Value = FleetInstrumentedPodGroupAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut applied_target: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut applied_target_name: Option<String> = None;
                let mut injected_tags: Option<Vec<String>> = None;
                let mut kube_ownerref_kind: Option<String> = None;
                let mut kube_ownerref_name: Option<String> = None;
                let mut lib_injection_annotations: Option<Vec<String>> = None;
                let mut namespace: Option<String> = None;
                let mut pod_count: Option<i64> = None;
                let mut pod_names: Option<Vec<String>> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "applied_target" => {
                            if v.is_null() {
                                continue;
                            }
                            applied_target =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "applied_target_name" => {
                            if v.is_null() {
                                continue;
                            }
                            applied_target_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "injected_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            injected_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kube_ownerref_kind" => {
                            if v.is_null() {
                                continue;
                            }
                            kube_ownerref_kind =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kube_ownerref_name" => {
                            if v.is_null() {
                                continue;
                            }
                            kube_ownerref_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lib_injection_annotations" => {
                            if v.is_null() {
                                continue;
                            }
                            lib_injection_annotations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pod_count" => {
                            if v.is_null() {
                                continue;
                            }
                            pod_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pod_names" => {
                            if v.is_null() {
                                continue;
                            }
                            pod_names = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = FleetInstrumentedPodGroupAttributes {
                    applied_target,
                    applied_target_name,
                    injected_tags,
                    kube_ownerref_kind,
                    kube_ownerref_name,
                    lib_injection_annotations,
                    namespace,
                    pod_count,
                    pod_names,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetInstrumentedPodGroupAttributesVisitor)
    }
}
