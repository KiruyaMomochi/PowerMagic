use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Equip {0} does not exist")]
    EquipNotFound(i64),
}