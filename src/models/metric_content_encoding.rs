/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricContentEncoding : HTTP header used to compress the media-type.

/// HTTP header used to compress the media-type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetricContentEncoding {
    #[serde(rename = "deflate")]
    DEFLATE,
    #[serde(rename = "zstd1")]
    ZSTD1,
    #[serde(rename = "gzip")]
    GZIP,

}

impl ToString for MetricContentEncoding {
    fn to_string(&self) -> String {
        match self {
            Self::DEFLATE => String::from("deflate"),
            Self::ZSTD1 => String::from("zstd1"),
            Self::GZIP => String::from("gzip"),
        }
    }
}

impl Default for MetricContentEncoding {
    fn default() -> MetricContentEncoding {
        Self::DEFLATE
    }
}




