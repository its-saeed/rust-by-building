#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Aiming,
    InFlight,
    BallLost,
    LevelComplete,
    GameOver,
}
