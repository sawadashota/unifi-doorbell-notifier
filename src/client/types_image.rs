use bytes::Bytes;

#[derive(Default, Debug, Clone)]
pub struct ImageResponse {
    pub content: Bytes,
    pub content_type: String,
}
