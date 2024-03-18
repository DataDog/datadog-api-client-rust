// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The update request for an incident's attachments.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAttachmentUpdateRequest {
    /// An array of incident attachments. An attachment object without an "id" key indicates that you want to
    /// create that attachment. An attachment object without an "attributes" key indicates that you want to
    /// delete that attachment. An attachment object with both the "id" key and a populated "attributes" object
    /// indicates that you want to update that attachment.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentAttachmentUpdateData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAttachmentUpdateRequest {
    pub fn new(
        data: Vec<crate::datadogV2::model::IncidentAttachmentUpdateData>,
    ) -> IncidentAttachmentUpdateRequest {
        IncidentAttachmentUpdateRequest {
            data,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IncidentAttachmentUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAttachmentUpdateRequestVisitor;
        impl<'a> Visitor<'a> for IncidentAttachmentUpdateRequestVisitor {
            type Value = IncidentAttachmentUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::IncidentAttachmentUpdateData>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = IncidentAttachmentUpdateRequest { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAttachmentUpdateRequestVisitor)
    }
}
