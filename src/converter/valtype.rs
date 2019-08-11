use crate::conventions::types::*;
use crate::converter::Converter;
use std::io::Read;

impl From<u8> for ValType {
    fn from(from: u8) -> ValType {
        match from {
            0x7f => ValType::I32(None),
            0x7e => ValType::I64(None),
            0x7d => ValType::F32(None),
            0x7c => ValType::F64(None),
            _ => panic!("Invalid valtype error"),
        }
    }
}

impl Converter<ValType> for &[u8] {
    fn convert(&mut self) -> Result<ValType, String> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf)
            .map_err(|_| "I/O Error occured".to_owned())?;
        Ok(buf[0].into())
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::ValType;
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_i32() -> Result<(), String> {
        let src: Vec<u8> = vec![0x7fu8];
        let result: ValType = (&src[..]).convert()?;

        assert_eq!(ValType::I32(None), result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i64() -> Result<(), String> {
        let src: Vec<u8> = vec![0x7eu8];
        let result: ValType = (&src[..]).convert()?;

        assert_eq!(ValType::I64(None), result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f32() -> Result<(), String> {
        let src: Vec<u8> = vec![0x7du8];
        let result: ValType = (&src[..]).convert()?;

        assert_eq!(ValType::F32(None), result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f64() -> Result<(), String> {
        let src: Vec<u8> = vec![0x7cu8];
        let result: ValType = (&src[..]).convert()?;

        assert_eq!(ValType::F64(None), result);
        Ok(())
    }
}
