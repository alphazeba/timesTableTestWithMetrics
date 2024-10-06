use text_io::Error;

#[derive(Debug)]
pub enum InvalidInput {
    InputMustBeInt(Error),
}