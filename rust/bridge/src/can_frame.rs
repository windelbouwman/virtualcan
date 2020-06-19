use scroll::{Pread, Pwrite};

#[derive(PartialEq, Debug)]
pub struct CanFrame {
    pub id: u32,
    pub extended: bool,
    pub data: Vec<u8>,
}

impl CanFrame {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: [u8; 14] = [0; 14];
        buf.pwrite_with::<u32>(self.id, 0, scroll::BE).unwrap();
        buf[4] = if self.extended { 1 } else { 0 };
        buf[5] = self.data.len() as u8;

        for (i, b) in self.data.iter().enumerate() {
            // println!("i={}", i);
            buf[6 + i] = *b;
        }

        // id u32
        // flags u8
        // size u8
        // data [u8]

        buf.to_vec()
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        let id: u32 = buf.pread_with(0, scroll::BE).unwrap();
        let extended: bool = buf[4] & 1 == 1;
        let size = buf[5];

        let mut data = vec![];
        for i in 0..size {
            data.push(buf[6 + i as usize])
        }

        CanFrame { id, extended, data }
    }
}

#[cfg(test)]
mod tests {

    use super::CanFrame;

    #[test]
    fn roundtrips() {
        let msg1 = CanFrame {
            id: 0x1337,
            extended: true,
            data: vec![1, 2, 3, 4, 5],
        };

        let data = msg1.to_bytes();
        let msg2 = CanFrame::from_bytes(&data);

        assert_eq!(msg1, msg2);
    }

    #[test]
    fn roundtrips2() {
        let msg1 = CanFrame {
            id: 13,
            extended: false,
            data: vec![0xca, 0xbb, 0xaf],
        };

        let data = msg1.to_bytes();
        let msg2 = CanFrame::from_bytes(&data);

        assert_eq!(msg1, msg2);
    }
}
