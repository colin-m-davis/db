pub enum Error {
  Parse(String),
  Io(String),
  Exec(String),
  Unknown(String),
}