#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Right,
    Left,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Message {
    Up,
    Down,
    Quit,
    Direction(Direction),
    Create,
    Input(char),
    InputDelete,
    InputMove(Direction),
    InputDone,
}
