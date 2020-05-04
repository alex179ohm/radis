use bytes::BytesMut;

pub struct Response(BytesMut);

impl Into<Option<i64>> for Response {
    fn into(self) -> Option<i64> {
        todo!()
    }
}

impl Into<Option<String>> for Response {
    fn into(self) -> Option<String> {
        todo!()
    }
}
