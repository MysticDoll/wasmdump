pub mod elemtype;
pub mod export_desc;
pub mod func_type;
pub mod global;
pub mod import_desc;
pub mod limits;
pub mod memtype;
pub mod string;
pub mod table;
pub mod valtype;
pub mod vectors;

pub trait Converter<T> {
    fn convert(&mut self) -> Result<T, String>;
}
