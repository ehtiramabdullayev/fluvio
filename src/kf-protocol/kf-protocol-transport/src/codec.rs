use std::io::Cursor;
use std::io::Error as IoError;

use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use log::trace;
use futures_codec::Decoder;
use futures_codec::Encoder;

use kf_protocol::Decoder as KDecoder;

/// Implement Kafka codec as in https://kafka.apache.org/protocol#The_Messages_ListOffsets
/// First 4 bytes are size of the message.  Then total buffer = 4 + message content
///
#[derive(Debug, Default)]
pub struct KfCodec {}

impl KfCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for KfCodec {
    type Item = BytesMut;
    type Error = IoError;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<BytesMut>, Self::Error> {
    
        let len = bytes.len();
        if len == 0 {
            return Ok(None)
        }
        if len >= 4 {
            let mut src = Cursor::new(&*bytes);
            let mut packet_len: i32 = 0;
            packet_len.decode(&mut src, 0)?;
            trace!("KCodec Decoder: raw: {}, message size: {}", len, packet_len);
            if (packet_len + 4) as usize <= len {
                trace!(
                    "KCodec Decoder: all packets are in buffer len: {} ",
                    packet_len + 4
                );
                let rest = bytes.split_off(4);
                bytes.truncate(0);
                Ok(Some(rest))
            } else {
                trace!(
                    "KCodec Decoder buffer len: {} is less than packet+4: {}, waiting",
                    len,
                    packet_len + 4
                );
                Ok(None)
            }
        } else {
            trace!("KCodec Decoder received raw bytes len: {} less than 4 not enough for size", len);
            Ok(None)
        }
    }
}

/// Implement encoder for Kafka Codec
/// We don't write buffer length because of file slice.  The buffer length
/// is encoded in the
impl Encoder for KfCodec {
    type Item = Bytes;
    type Error = IoError;

    fn encode(&mut self, data: Self::Item, buf: &mut BytesMut) -> Result<(), IoError> {
        trace!("KCodec Encoder: Encoding data with {} bytes", data.len());
        buf.put(data);
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::io::Error;
    use std::net::SocketAddr;
    use std::time;

    use bytes::Bytes;
    use futures::future::join;
    use futures::sink::SinkExt;
    use futures::stream::StreamExt;
    use futures_codec::Framed;

    use flv_future_aio::net::AsyncTcpListener;
    use flv_future_aio::net::AsyncTcpStream;
    use flv_future_core::sleep;
    use flv_future_core::test_async;
    use kf_protocol::Decoder as KDecoder;
    use kf_protocol::Encoder as KEncoder;
    use log::debug;

    use super::KfCodec;

    #[test_async]
    async fn test_async_tcp() -> Result<(), Error> {
        debug!("start running test");

        let addr = "127.0.0.1:11122".parse::<SocketAddr>().expect("parse");

        let server_ft = async {
            debug!("server: binding");
            let listener = AsyncTcpListener::bind(&addr).await?;
            debug!("server: successfully binding. waiting for incoming");
            let mut incoming = listener.incoming();
            while let Some(stream) = incoming.next().await {
                debug!("server: got connection from client");
                let tcp_stream = stream?;
                let framed = Framed::new(tcp_stream, KfCodec {});
                let (mut sink, _) = framed.split();
                let data: Vec<u8> = vec![0x1, 0x02, 0x03, 0x04, 0x5];
                //  debug!("server encoding original vector with len: {}", data.len());
                let mut buf = vec![];
                data.encode(&mut buf, 0)?;
                //  debug!("server encoder buffer len: {}, should be 9 = 5 + 4(len)", buf.len());
                debug!(
                    "server: writing to client vector encoded len: {}",
                    buf.len()
                );
                assert_eq!(buf.len(), 9); //  4(array len)+ 5 bytes

                // write buffer length since encoder doesn't write
                // need to send out len
                let mut len_buf = vec![];
                let len = buf.len() as i32;
                len.encode(&mut len_buf, 0)?;
                sink.send(Bytes::from(len_buf)).await?;

                sink.send(Bytes::from(buf)).await?;
                /*
                debug!("server: sending 2nd value to client");
                let data2 = vec![0x20,0x11];
                await!(sink.send(to_bytes(data2)))?;
                // sleep for 100 ms to give client time
                debug!("wait for 50 ms to give receiver change to process");
                */
                flv_future_core::sleep(time::Duration::from_millis(50)).await;
                debug!("finishing. terminating server");
                return Ok(()) as Result<(), Error>;
            }

            Ok(()) as Result<(), Error>
        };

        let client_ft = async {
            debug!("client: sleep to give server chance to come up");
            sleep(time::Duration::from_millis(100)).await;
            debug!("client: trying to connect");
            let tcp_stream = AsyncTcpStream::connect(&addr).await?;
            debug!("client: got connection. waiting");
            let framed = Framed::new(tcp_stream, KfCodec {});
            let (_, mut stream) = framed.split();
            if let Some(value) = stream.next().await {
                debug!("client :received first value from server");
                let mut bytes = value?;
                debug!("client: received bytes len: {}", bytes.len());
                assert_eq!(bytes.len(), 9, "total bytes is 9");
                let mut decoded_values = vec![];
                decoded_values
                    .decode(&mut bytes, 0)
                    .expect("vector decoding failed");
                assert_eq!(decoded_values.len(), 5);
                assert_eq!(decoded_values[0], 1);
                assert_eq!(decoded_values[1], 2);
                debug!("all test pass");
            } else {
                assert!(false, "no first value received");
            }

            debug!("waiting for 2nd value");
            /*
            if let Some(value) = await!(stream.next()) {
                debug!("client: received 2nd value from server");
                let mut bytes = value?;
                let values = bytes.take();
                assert_eq!(values.len(),2);

            } else {
                assert!(false,"no second value received");
            }
            */

            debug!("finished client");

            Ok(()) as Result<(), Error>
        };

        let _rt = join(client_ft, server_ft).await;

        Ok(())
    }
}
