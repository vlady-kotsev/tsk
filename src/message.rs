#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChangeScreenDirection {
    In,
    Out,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Message {
    Up,
    Down,
    Quit,
    ChangeScreen(ChangeScreenDirection),
    Create,
}
