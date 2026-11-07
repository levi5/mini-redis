use bytes::{Buf, Bytes, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Debug, Clone, PartialEq)]
pub enum RespFrame {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Bytes),
    Array(Vec<RespFrame>),
    Null,
}

pub struct RespParser<R> {
    reader: BufReader<R>,
    buffer: BytesMut,
}

impl<R: tokio::io::AsyncRead + Unpin> RespParser<R> {
    pub fn new(stream: R) -> Self {
        Self {
            reader: BufReader::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn parse_frame(
        &mut self,
    ) -> Result<Option<RespFrame>, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            if let Some(frame) = self.parse()? {
                return Ok(Some(frame));
            }

            let bytes_read = self.reader.read_buf(&mut self.buffer).await?;
            if bytes_read == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(
                        "Connection closed abruptly with incomplete data in the buffer".into(),
                    );
                }
            }
        }
    }

    fn parse(&mut self) -> Result<Option<RespFrame>, Box<dyn std::error::Error + Send + Sync>> {
        let mut cursor = Cursor::new(&self.buffer[..]);

        match Self::parse_type(&mut cursor)? {
            Some(frame) => {
                let len = cursor.position() as usize;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            None => Ok(None),
        }
    }

    fn parse_type(
        _cursor: &mut Cursor<&[u8]>,
    ) -> Result<Option<RespFrame>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}
