// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response object containing an incident's attachments.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAttachmentsResponse {
    /// An array of incident attachments.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentAttachmentData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentAttachmentsResponseIncludedItem>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAttachmentsResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::IncidentAttachmentData>,
    ) -> IncidentAttachmentsResponse {
        IncidentAttachmentsResponse {
            data,
            included: None,
            _unparsed: false,
        }
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentAttachmentsResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentAttachmentsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAttachmentsResponseVisitor;
        impl<'a> Visitor<'a> for IncidentAttachmentsResponseVisitor {
            type Value = IncidentAttachmentsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::IncidentAttachmentData>> = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::IncidentAttachmentsResponseIncludedItem>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = IncidentAttachmentsResponse {
                    data,
                    included,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAttachmentsResponseVisitor)
    }
}
