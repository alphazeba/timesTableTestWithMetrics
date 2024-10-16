use text_io::Error;

#[derive(Debug)]
pub enum InvalidInput {
    #[allow(dead_code)]
    InputMustBeInt(Error),
}