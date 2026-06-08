#[derive(Debug, Clone, Copy)]
pub struct DrawEvent {
    pub x:        f32,
    pub y:        f32,
    pub r:        u8,
    pub g:        u8,
    pub b:        u8,
    pub size:     u8,
    pub pen_down: bool,
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
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 13 { return None; }
        Some(DrawEvent {
            x:        f32::from_le_bytes(buf[0..4].try_into().ok()?),
            y:        f32::from_le_bytes(buf[4..8].try_into().ok()?),
            r:        buf[8],
            g:        buf[9],
            b:        buf[10],
            size:     buf[11],
            pen_down: buf[12] != 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let ev = DrawEvent { x: 100.5, y: 200.0, r: 255, g: 128, b: 0, size: 10, pen_down: true };
        let decoded = DrawEvent::from_bytes(&ev.to_bytes()).unwrap();
        assert_eq!(ev.x, decoded.x);
        assert_eq!(ev.y, decoded.y);
        assert_eq!(ev.r, decoded.r);
        assert_eq!(ev.pen_down, decoded.pen_down);
    }
}
