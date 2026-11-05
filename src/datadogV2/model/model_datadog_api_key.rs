// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `DatadogAPIKey` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatadogAPIKey {
    /// The `DatadogAPIKey` `api_key`.
    #[serde(rename = "api_key")]
    pub api_key: crate::datadogV2::model::DatadogAPIKeyApiKey,
    /// The `DatadogAPIKey` `app_key`.
    #[serde(rename = "app_key")]
    pub app_key: crate::datadogV2::model::DatadogAPIKeyAppKey,
    /// The `DatadogAPIKey` `datacenter`.
    #[serde(rename = "datacenter")]
    pub datacenter: crate::datadogV2::model::DatadogAPIKeyDatacenter,
    /// Custom subdomain used for Datadog URLs generated with this Connection. For example, if this org uses `<https://acme.datadoghq.com`> to access Datadog, set this field to `acme`. If this field is omitted, generated URLs will use the default site URL for its datacenter (see [<https://docs.datadoghq.com/getting_started/site]>(<https://docs.datadoghq.com/getting_started/site>)).
    #[serde(rename = "subdomain")]
    pub subdomain: Option<crate::datadogV2::model::DatadogAPIKeySubdomain>,
    /// The definition of the `DatadogAPIKey` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DatadogAPIKeyType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatadogAPIKey {
    pub fn new(
        api_key: crate::datadogV2::model::DatadogAPIKeyApiKey,
        app_key: crate::datadogV2::model::DatadogAPIKeyAppKey,
        datacenter: crate::datadogV2::model::DatadogAPIKeyDatacenter,
        type_: crate::datadogV2::model::DatadogAPIKeyType,
    ) -> DatadogAPIKey {
        DatadogAPIKey {
            api_key,
            app_key,
            datacenter,
            subdomain: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn subdomain(mut self, value: crate::datadogV2::model::DatadogAPIKeySubdomain) -> Self {
        self.subdomain = Some(value);
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

impl<'de> Deserialize<'de> for DatadogAPIKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatadogAPIKeyVisitor;
        impl<'a> Visitor<'a> for DatadogAPIKeyVisitor {
            type Value = DatadogAPIKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<crate::datadogV2::model::DatadogAPIKeyApiKey> = None;
                let mut app_key: Option<crate::datadogV2::model::DatadogAPIKeyAppKey> = None;
                let mut datacenter: Option<crate::datadogV2::model::DatadogAPIKeyDatacenter> = None;
                let mut subdomain: Option<crate::datadogV2::model::DatadogAPIKeySubdomain> = None;
                let mut type_: Option<crate::datadogV2::model::DatadogAPIKeyType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _api_key) = api_key {
                                match _api_key {
                                    crate::datadogV2::model::DatadogAPIKeyApiKey::UnparsedObject(_api_key) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "app_key" => {
                            app_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _app_key) = app_key {
                                match _app_key {
                                    crate::datadogV2::model::DatadogAPIKeyAppKey::UnparsedObject(_app_key) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "datacenter" => {
                            datacenter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _datacenter) = datacenter {
                                match _datacenter {
                                    crate::datadogV2::model::DatadogAPIKeyDatacenter::UnparsedObject(_datacenter) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "subdomain" => {
                            if v.is_null() {
                                continue;
                            }
                            subdomain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _subdomain) = subdomain {
                                match _subdomain {
                                    crate::datadogV2::model::DatadogAPIKeySubdomain::UnparsedObject(_subdomain) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DatadogAPIKeyType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let api_key = api_key.ok_or_else(|| M::Error::missing_field("api_key"))?;
                let app_key = app_key.ok_or_else(|| M::Error::missing_field("app_key"))?;
                let datacenter = datacenter.ok_or_else(|| M::Error::missing_field("datacenter"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = DatadogAPIKey {
                    api_key,
                    app_key,
                    datacenter,
                    subdomain,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatadogAPIKeyVisitor)
    }
}
