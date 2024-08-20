// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Elasticsearch destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationForwardDestinationElasticsearch {
    /// Basic access authentication.
    #[serde(rename = "auth")]
    pub auth: crate::datadogV2::model::CustomDestinationElasticsearchDestinationAuth,
    /// The destination for which logs will be forwarded to.
    /// Must have HTTPS scheme and forwarding back to Datadog is not allowed.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// Name of the Elasticsearch index (must follow [Elasticsearch's criteria](<https://www.elastic.co/guide/en/elasticsearch/reference/8.11/indices-create-index.html#indices-create-api-path-params>)).
    #[serde(rename = "index_name")]
    pub index_name: String,
    /// Date pattern with US locale and UTC timezone to be appended to the index name after adding `-`
    /// (that is, `${index_name}-${indexPattern}`).
    /// You can customize the index rotation naming pattern by choosing one of these options:
    /// - Hourly: `yyyy-MM-dd-HH` (as an example, it would render: `2022-10-19-09`)
    /// - Daily: `yyyy-MM-dd` (as an example, it would render: `2022-10-19`)
    /// - Weekly: `yyyy-'W'ww` (as an example, it would render: `2022-W42`)
    /// - Monthly: `yyyy-MM` (as an example, it would render: `2022-10`)
    ///
    /// If this field is missing or is blank, it means that the index name will always be the same
    /// (that is, no rotation).
    #[serde(rename = "index_rotation")]
    pub index_rotation: Option<String>,
    /// Type of the Elasticsearch destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CustomDestinationForwardDestinationElasticsearchType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationForwardDestinationElasticsearch {
    pub fn new(
        auth: crate::datadogV2::model::CustomDestinationElasticsearchDestinationAuth,
        endpoint: String,
        index_name: String,
        type_: crate::datadogV2::model::CustomDestinationForwardDestinationElasticsearchType,
    ) -> CustomDestinationForwardDestinationElasticsearch {
        CustomDestinationForwardDestinationElasticsearch {
            auth,
            endpoint,
            index_name,
            index_rotation: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn index_rotation(mut self, value: String) -> Self {
        self.index_rotation = Some(value);
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

impl<'de> Deserialize<'de> for CustomDestinationForwardDestinationElasticsearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationForwardDestinationElasticsearchVisitor;
        impl<'a> Visitor<'a> for CustomDestinationForwardDestinationElasticsearchVisitor {
            type Value = CustomDestinationForwardDestinationElasticsearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::CustomDestinationElasticsearchDestinationAuth,
                > = None;
                let mut endpoint: Option<String> = None;
                let mut index_name: Option<String> = None;
                let mut index_rotation: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CustomDestinationForwardDestinationElasticsearchType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "endpoint" => {
                            endpoint = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index_name" => {
                            index_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index_rotation" => {
                            if v.is_null() {
                                continue;
                            }
                            index_rotation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomDestinationForwardDestinationElasticsearchType::UnparsedObject(_type_) => {
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
                let auth = auth.ok_or_else(|| M::Error::missing_field("auth"))?;
                let endpoint = endpoint.ok_or_else(|| M::Error::missing_field("endpoint"))?;
                let index_name = index_name.ok_or_else(|| M::Error::missing_field("index_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomDestinationForwardDestinationElasticsearch {
                    auth,
                    endpoint,
                    index_name,
                    index_rotation,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationForwardDestinationElasticsearchVisitor)
    }
}
