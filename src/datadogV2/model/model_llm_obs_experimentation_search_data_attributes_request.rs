// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an experimentation search request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationSearchDataAttributesRequest {
    /// Options to control content preview truncation.
    #[serde(rename = "content_preview")]
    pub content_preview: Option<crate::datadogV2::model::LLMObsExperimentationContentPreview>,
    /// Filter criteria for an experimentation search request.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV2::model::LLMObsExperimentationFilter,
    /// Additional data to include in the response.
    #[serde(rename = "include")]
    pub include: Option<crate::datadogV2::model::LLMObsExperimentationInclude>,
    /// Cursor-based pagination parameters.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::LLMObsExperimentationCursorPage>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationSearchDataAttributesRequest {
    pub fn new(
        filter: crate::datadogV2::model::LLMObsExperimentationFilter,
    ) -> LLMObsExperimentationSearchDataAttributesRequest {
        LLMObsExperimentationSearchDataAttributesRequest {
            content_preview: None,
            filter,
            include: None,
            page: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content_preview(
        mut self,
        value: crate::datadogV2::model::LLMObsExperimentationContentPreview,
    ) -> Self {
        self.content_preview = Some(value);
        self
    }

    pub fn include(mut self, value: crate::datadogV2::model::LLMObsExperimentationInclude) -> Self {
        self.include = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::LLMObsExperimentationCursorPage) -> Self {
        self.page = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsExperimentationSearchDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationSearchDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationSearchDataAttributesRequestVisitor {
            type Value = LLMObsExperimentationSearchDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content_preview: Option<
                    crate::datadogV2::model::LLMObsExperimentationContentPreview,
                > = None;
                let mut filter: Option<crate::datadogV2::model::LLMObsExperimentationFilter> = None;
                let mut include: Option<crate::datadogV2::model::LLMObsExperimentationInclude> =
                    None;
                let mut page: Option<crate::datadogV2::model::LLMObsExperimentationCursorPage> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content_preview" => {
                            if v.is_null() {
                                continue;
                            }
                            content_preview =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            if v.is_null() {
                                continue;
                            }
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;

                let content = LLMObsExperimentationSearchDataAttributesRequest {
                    content_preview,
                    filter,
                    include,
                    page,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationSearchDataAttributesRequestVisitor)
    }
}
