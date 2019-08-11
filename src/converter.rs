pub mod func_type;
pub mod limits;
pub mod valtype;
pub mod vectors;

pub trait Converter<T> {
    fn convert(&mut self) -> Result<T, String>;
}
