// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated uncompressed resource detection grouped by URL path.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedUncompressedResource {
    /// Average uncompressed body size in bytes.
    #[serde(rename = "avg_body_size")]
    pub avg_body_size: i64,
    /// Average resource loading duration in nanoseconds.
    #[serde(rename = "avg_duration")]
    pub avg_duration: i64,
    /// Unique fingerprint identifying this detection group.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Impact score combining view frequency and resource size.
    #[serde(rename = "impact_score")]
    pub impact_score: f64,
    /// Total number of detection instances across sampled views.
    #[serde(rename = "instance_count")]
    pub instance_count: i32,
    /// CDN or hosting provider type for the resource.
    #[serialize_always]
    #[serde(rename = "provider_type")]
    pub provider_type: Option<String>,
    /// Whether the resource is render-blocking.
    #[serialize_always]
    #[serde(rename = "render_blocking")]
    pub render_blocking: Option<String>,
    /// Type of the resource (JS, CSS, image, fetch, and so on).
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// Normalized URL path pattern for the uncompressed resource.
    #[serde(rename = "url_path_group")]
    pub url_path_group: String,
    /// Number of sampled views where this detection occurred.
    #[serde(rename = "view_occurrences")]
    pub view_occurrences: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedUncompressedResource {
    pub fn new(
        avg_body_size: i64,
        avg_duration: i64,
        fingerprint: String,
        impact_score: f64,
        instance_count: i32,
        provider_type: Option<String>,
        render_blocking: Option<String>,
        resource_type: String,
        url_path_group: String,
        view_occurrences: i32,
    ) -> AggregatedUncompressedResource {
        AggregatedUncompressedResource {
            avg_body_size,
            avg_duration,
            fingerprint,
            impact_score,
            instance_count,
            provider_type,
            render_blocking,
            resource_type,
            url_path_group,
            view_occurrences,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AggregatedUncompressedResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedUncompressedResourceVisitor;
        impl<'a> Visitor<'a> for AggregatedUncompressedResourceVisitor {
            type Value = AggregatedUncompressedResource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_body_size: Option<i64> = None;
                let mut avg_duration: Option<i64> = None;
                let mut fingerprint: Option<String> = None;
                let mut impact_score: Option<f64> = None;
                let mut instance_count: Option<i32> = None;
                let mut provider_type: Option<Option<String>> = None;
                let mut render_blocking: Option<Option<String>> = None;
                let mut resource_type: Option<String> = None;
                let mut url_path_group: Option<String> = None;
                let mut view_occurrences: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_body_size" => {
                            avg_body_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_duration" => {
                            avg_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fingerprint" => {
                            fingerprint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_score" => {
                            impact_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_count" => {
                            instance_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_type" => {
                            provider_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "render_blocking" => {
                            render_blocking =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url_path_group" => {
                            url_path_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_occurrences" => {
                            view_occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let avg_body_size =
                    avg_body_size.ok_or_else(|| M::Error::missing_field("avg_body_size"))?;
                let avg_duration =
                    avg_duration.ok_or_else(|| M::Error::missing_field("avg_duration"))?;
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let impact_score =
                    impact_score.ok_or_else(|| M::Error::missing_field("impact_score"))?;
                let instance_count =
                    instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let provider_type =
                    provider_type.ok_or_else(|| M::Error::missing_field("provider_type"))?;
                let render_blocking =
                    render_blocking.ok_or_else(|| M::Error::missing_field("render_blocking"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let url_path_group =
                    url_path_group.ok_or_else(|| M::Error::missing_field("url_path_group"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = AggregatedUncompressedResource {
                    avg_body_size,
                    avg_duration,
                    fingerprint,
                    impact_score,
                    instance_count,
                    provider_type,
                    render_blocking,
                    resource_type,
                    url_path_group,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedUncompressedResourceVisitor)
    }
}
