#[derive(Debug, Clone, Copy)]
pub struct DrawEvent {
    pub x:        f32,
    pub y:        f32,
    pub r:        u8,
    pub g:        u8,
    pub b:        u8,
    pub size:     u8,
    pub pen_down: bool,
    // TODO 3: add `pub clear: bool`
}

impl DrawEvent {
    pub fn to_bytes(self) -> [u8; 13] {
        let mut buf = [0u8; 13];
        buf[0..4].copy_from_slice(&self.x.to_le_bytes());
        buf[4..8].copy_from_slice(&self.y.to_le_bytes());
        buf[8]  = self.r;
        buf[9]  = self.g;
        buf[10] = self.b;
        buf[11] = self.size;
        buf[12] = self.pen_down as u8;
        // TODO 3: encode clear at buf[13]; update return type to [u8; 14]
        // TODO 4: insert version byte 1 at buf[0]; shift all other fields by one;
        //         encode clear at buf[14]; update return type to [u8; 15]
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 13 { return None; }
        // TODO 4: if buf[0] != 1 { return None; }  — reject wrong version
        // TODO 4: update length check to buf.len() < 15
        // TODO 4: shift field reads by one (x from buf[1..5], y from buf[5..9], etc.)
        Some(DrawEvent {
            x:        f32::from_le_bytes(buf[0..4].try_into().ok()?),
            y:        f32::from_le_bytes(buf[4..8].try_into().ok()?),
            r:        buf[8],
            g:        buf[9],
            b:        buf[10],
            size:     buf[11],
            pen_down: buf[12] != 0,
            // TODO 3: add clear: buf[13] != 0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let ev = DrawEvent {
            x: 100.5, y: 200.0, r: 255, g: 128, b: 0, size: 10, pen_down: true,
            // TODO 3: add clear: false
        };
        let decoded = DrawEvent::from_bytes(&ev.to_bytes()).unwrap();
        assert_eq!(ev.x, decoded.x);
        assert_eq!(ev.y, decoded.y);
        assert_eq!(ev.r, decoded.r);
        assert_eq!(ev.pen_down, decoded.pen_down);
    }

    // TODO 4: add test `rejects_wrong_version`:
    //   encode a valid event, set bytes[0] = 99, assert from_bytes returns None
}
