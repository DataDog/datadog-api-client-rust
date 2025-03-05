// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationSecurityWafCustomRuleConditionInputAddress {
    SERVER_DB_STATEMENT,
    SERVER_IO_FS_FILE,
    SERVER_IO_NET_URL,
    SERVER_SYS_SHELL_CMD,
    SERVER_REQUEST_METHOD,
    SERVER_REQUEST_URI_RAW,
    SERVER_REQUEST_PATH_PARAMS,
    SERVER_REQUEST_QUERY,
    SERVER_REQUEST_HEADERS_NO_COOKIES,
    SERVER_REQUEST_COOKIES,
    SERVER_REQUEST_TRAILERS,
    SERVER_REQUEST_BODY,
    SERVER_RESPONSE_STATUS,
    SERVER_RESPONSE_HEADERS_NO_COOKIES,
    SERVER_RESPONSE_TRAILERS,
    GRPC_SERVER_REQUEST_METADATA,
    GRPC_SERVER_REQUEST_MESSAGE,
    GRPC_SERVER_METHOD,
    GRAPHQL_SERVER_ALL_RESOLVERS,
    USR_ID,
    HTTP_CLIENT_IP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ApplicationSecurityWafCustomRuleConditionInputAddress {
    fn to_string(&self) -> String {
        match self {
            Self::SERVER_DB_STATEMENT => String::from("server.db.statement"),
            Self::SERVER_IO_FS_FILE => String::from("server.io.fs.file"),
            Self::SERVER_IO_NET_URL => String::from("server.io.net.url"),
            Self::SERVER_SYS_SHELL_CMD => String::from("server.sys.shell.cmd"),
            Self::SERVER_REQUEST_METHOD => String::from("server.request.method"),
            Self::SERVER_REQUEST_URI_RAW => String::from("server.request.uri.raw"),
            Self::SERVER_REQUEST_PATH_PARAMS => String::from("server.request.path_params"),
            Self::SERVER_REQUEST_QUERY => String::from("server.request.query"),
            Self::SERVER_REQUEST_HEADERS_NO_COOKIES => {
                String::from("server.request.headers.no_cookies")
            }
            Self::SERVER_REQUEST_COOKIES => String::from("server.request.cookies"),
            Self::SERVER_REQUEST_TRAILERS => String::from("server.request.trailers"),
            Self::SERVER_REQUEST_BODY => String::from("server.request.body"),
            Self::SERVER_RESPONSE_STATUS => String::from("server.response.status"),
            Self::SERVER_RESPONSE_HEADERS_NO_COOKIES => {
                String::from("server.response.headers.no_cookies")
            }
            Self::SERVER_RESPONSE_TRAILERS => String::from("server.response.trailers"),
            Self::GRPC_SERVER_REQUEST_METADATA => String::from("grpc.server.request.metadata"),
            Self::GRPC_SERVER_REQUEST_MESSAGE => String::from("grpc.server.request.message"),
            Self::GRPC_SERVER_METHOD => String::from("grpc.server.method"),
            Self::GRAPHQL_SERVER_ALL_RESOLVERS => String::from("graphql.server.all_resolvers"),
            Self::USR_ID => String::from("usr.id"),
            Self::HTTP_CLIENT_IP => String::from("http.client_ip"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ApplicationSecurityWafCustomRuleConditionInputAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleConditionInputAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "server.db.statement" => Self::SERVER_DB_STATEMENT,
            "server.io.fs.file" => Self::SERVER_IO_FS_FILE,
            "server.io.net.url" => Self::SERVER_IO_NET_URL,
            "server.sys.shell.cmd" => Self::SERVER_SYS_SHELL_CMD,
            "server.request.method" => Self::SERVER_REQUEST_METHOD,
            "server.request.uri.raw" => Self::SERVER_REQUEST_URI_RAW,
            "server.request.path_params" => Self::SERVER_REQUEST_PATH_PARAMS,
            "server.request.query" => Self::SERVER_REQUEST_QUERY,
            "server.request.headers.no_cookies" => Self::SERVER_REQUEST_HEADERS_NO_COOKIES,
            "server.request.cookies" => Self::SERVER_REQUEST_COOKIES,
            "server.request.trailers" => Self::SERVER_REQUEST_TRAILERS,
            "server.request.body" => Self::SERVER_REQUEST_BODY,
            "server.response.status" => Self::SERVER_RESPONSE_STATUS,
            "server.response.headers.no_cookies" => Self::SERVER_RESPONSE_HEADERS_NO_COOKIES,
            "server.response.trailers" => Self::SERVER_RESPONSE_TRAILERS,
            "grpc.server.request.metadata" => Self::GRPC_SERVER_REQUEST_METADATA,
            "grpc.server.request.message" => Self::GRPC_SERVER_REQUEST_MESSAGE,
            "grpc.server.method" => Self::GRPC_SERVER_METHOD,
            "graphql.server.all_resolvers" => Self::GRAPHQL_SERVER_ALL_RESOLVERS,
            "usr.id" => Self::USR_ID,
            "http.client_ip" => Self::HTTP_CLIENT_IP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
