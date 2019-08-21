use crate::conventions::types::MemType;
use crate::converter::Converter;

impl Converter<MemType> for &[u8] {
    fn convert(&mut self) -> Result<MemType, String> {
        Ok(MemType::new(self.convert()?))
    }
}
