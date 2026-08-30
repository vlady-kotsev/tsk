#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DirectionX {
    Right,
    Left,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DirectionY {
    Up,
    Down,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Message {
    Up,
    Down,
    Quit,
    Direction(DirectionX),
    Create,
    Input(char),
    InputDelete,
    InputMove(DirectionX),
    InputDone,
    InputCancel,
    Delete,
    MoveOrder(DirectionY),
}
