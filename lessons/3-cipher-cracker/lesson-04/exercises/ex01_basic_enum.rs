// Exercise 3.1 — define an enum and match on it
//
// Define an enum `Direction` with four unit variants:
//   North, South, East, West
//
// Then implement `fn opposite(d: Direction) -> Direction`
// that returns the opposite direction:
//   North ↔ South, East ↔ West
//
// All tests must pass.

// TODO: define Direction enum

// TODO: implement opposite

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north_opposite_is_south() {
        assert!(matches!(opposite(Direction::North), Direction::South));
    }

    #[test]
    fn south_opposite_is_north() {
        assert!(matches!(opposite(Direction::South), Direction::North));
    }

    #[test]
    fn east_opposite_is_west() {
        assert!(matches!(opposite(Direction::East), Direction::West));
    }

    #[test]
    fn west_opposite_is_east() {
        assert!(matches!(opposite(Direction::West), Direction::East));
    }
}

fn main() {
    // No assertions — if it compiles and tests pass, you're done.
}
