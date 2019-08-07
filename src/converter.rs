pub mod func_type;
pub mod valtype;

pub trait Converter<T> {
    fn convert(&mut self) -> Result<T, &str>;
}
