
#[derive(Clone, Debug)]
pub enum ParsedCqlStatement {
    Select,
    Insert,
    Delete,
    Update
}
