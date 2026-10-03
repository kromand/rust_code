const BUF_SIZE: usize = 1500;
pub const MPSC_CHANNEL_SIZE: usize = 16;

pub struct Message {
    pub size: usize,
    conId: usize,
    pub buf: Box<[u8; BUF_SIZE]>,
}
impl Message {
    pub const WIRE_HEADER_SIZE: usize = 8;
    pub const MAX_PAYLOAD_SIZE: usize = BUF_SIZE;

    pub fn new(cid: usize) -> Message {
        Message {
            size: 0,
            conId: cid,
            buf: Box::new([0; BUF_SIZE]),
        }
    }

    pub fn connection_id(&self) -> usize {
        self.conId
    }

    pub fn decode_wire_header(header: &[u8; Self::WIRE_HEADER_SIZE]) -> (usize, usize) {
        let connection_id = u32::from_be_bytes(header[0..4].try_into().unwrap()) as usize;
        let payload_size = u32::from_be_bytes(header[4..8].try_into().unwrap()) as usize;
        (connection_id, payload_size)
    }

    pub fn encode_wire_header(&self) -> Option<[u8; Self::WIRE_HEADER_SIZE]> {
        let connection_id = u32::try_from(self.conId).ok()?;
        let payload_size = u32::try_from(self.size).ok()?;
        if self.size > self.buf.len() {
            return None;
        }

        let mut header = [0; Self::WIRE_HEADER_SIZE];
        header[0..4].copy_from_slice(&connection_id.to_be_bytes());
        header[4..8].copy_from_slice(&payload_size.to_be_bytes());
        Some(header)
    }
}

pub struct MpscChannel {
    pub id: usize,
    assignedThread: usize,
    pub tx: tokio::sync::mpsc::Sender<Message>,
    pub rx: tokio::sync::mpsc::Receiver<Message>,
}
impl MpscChannel {
    pub fn new(
        i: usize,
        a: usize,
        t: tokio::sync::mpsc::Sender<Message>,
        r: tokio::sync::mpsc::Receiver<Message>,
    ) -> MpscChannel {
        MpscChannel {
            id: i,
            assignedThread: a,
            tx: t,
            rx: r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn wire_header_encodes_connection_id_and_payload_size() {
        let mut message = Message::new(0x0102_0304);
        message.size = 0x0102;

        let header = message.encode_wire_header().unwrap();

        assert_eq!(header, [1, 2, 3, 4, 0, 0, 1, 2]);
        assert_eq!(Message::decode_wire_header(&header), (0x0102_0304, 0x0102));
    }
}
