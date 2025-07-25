// Format du protocole : [op_code: u8][len: u16][payload: Vec<u8>]

pub const OP_MSG: u8 = 0x01;
pub const OP_QUIT: u8 = 0x02;
pub const OP_ACK: u8 = 0x03;

pub struct ProtoMsg {
    pub op: u8,
    pub payload: Vec<u8>,
}

impl ProtoMsg {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.op);
        let len = self.payload.len() as u16;
        buf.push((len >> 8) as u8);
        buf.push((len & 0xFF) as u8);
        buf.extend(&self.payload);
        buf
    }
    pub fn deserialize(buf: &[u8]) -> Option<ProtoMsg> {
        if buf.len() < 3 { return None; }
        let op = buf[0];
        let len = ((buf[1] as u16) << 8) | (buf[2] as u16);
        if buf.len() < 3 + len as usize { return None; }
        let payload = buf[3..3+len as usize].to_vec();
        Some(ProtoMsg { op, payload })
    }
} 