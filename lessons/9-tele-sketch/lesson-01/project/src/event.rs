// TODO: define DrawEvent with these fields:
//   x: f32, y: f32   — canvas position
//   r: u8, g: u8, b: u8  — stroke colour
//   size: u8          — brush radius in pixels
//   pen_down: bool    — true = draw, false = pen lifted
//
// Add #[derive(Debug, Clone, Copy)]

pub struct DrawEvent {
    // TODO
}

impl DrawEvent {
    // TODO: implement to_bytes(&self) -> [u8; 13]
    //   lay out fields as: [x:4][y:4][r][g][b][size][pen_down as u8]
    pub fn to_bytes(self) -> [u8; 13] {
        todo!()
    }

    // TODO: implement from_bytes(buf: &[u8]) -> Option<Self>
    //   return None if buf.len() < 13
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write a round-trip test
    // Create a DrawEvent, call to_bytes(), call from_bytes(), assert fields match
    #[test]
    fn round_trip() {
        todo!()
    }
}
