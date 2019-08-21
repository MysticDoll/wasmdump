pub mod elemtype;
pub mod func_type;
pub mod global;
pub mod limits;
pub mod memtype;
pub mod table;
pub mod valtype;
pub mod vectors;

pub trait Converter<T> {
    fn convert(&mut self) -> Result<T, String>;
}
