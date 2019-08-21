use crate::conventions::types::{GlobalType, ValType};
use crate::converter::Converter;
use std::io::Read;

impl Converter<GlobalType> for &[u8] {
    fn convert(&mut self) -> Result<GlobalType, String> {
        let valtype: ValType = self.convert()?;
        let mut is_mutable: [u8; 1] = [0; 1];
        self.read_exact(&mut is_mutable)
            .map_err(|_| "I/O error occured")?;
        Ok(GlobalType::new(valtype, is_mutable[0] == 1))
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::{GlobalType, ValType};
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_i64_mutable() -> Result<(), String> {
        let src = vec![0x7eu8, 0x01u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::I64(None), true);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i32_mutable() -> Result<(), String> {
        let src = vec![0x7fu8, 0x01u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::I32(None), true);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f64_mutable() -> Result<(), String> {
        let src = vec![0x7cu8, 0x01u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::F64(None), true);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f32_mutable() -> Result<(), String> {
        let src = vec![0x7du8, 0x01u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::F32(None), true);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i64_immutable() -> Result<(), String> {
        let src = vec![0x7eu8, 0x00u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::I64(None), false);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i32_immutable() -> Result<(), String> {
        let src = vec![0x7fu8, 0x00u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::I32(None), false);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f64_immutable() -> Result<(), String> {
        let src = vec![0x7cu8, 0x00u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::F64(None), false);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f32_immutable() -> Result<(), String> {
        let src = vec![0x7du8, 0x00u8];
        let result: GlobalType = (&src[..]).convert()?;
        let expected = GlobalType::new(ValType::F32(None), false);
        assert_eq!(expected, result);
        Ok(())
    }
}
