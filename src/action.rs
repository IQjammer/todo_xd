#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Up,
    Down,
    Done,
    EnterNormalMode,
    EnterEditingMode,
    InsertChar(char),
    DeleteChar,
    SubmitInput,
    Delete,
    Tick,
    None,
}
