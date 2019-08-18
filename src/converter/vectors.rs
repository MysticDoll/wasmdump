use crate::converter::Converter;
use std::io::Read;

impl<T, S> Converter<Vec<T>> for S
where
    S: Converter<T> + Read,
{
    fn convert(&mut self) -> Result<Vec<T>, String> {
        let num = leb128::read::unsigned(self)
            .map_err(|_| "Error occured when read leb128 param of the nums of Vector".to_owned())?
            as usize;

        let mut values: Vec<T> = Vec::new();

        for _ in 0..num {
            values.push(self.convert()?);
        }

        Ok(values)
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::*;
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_vec_valtype() -> Result<(), String> {
        let src = vec![0x04u8, 0x7fu8, 0x7eu8, 0x7du8, 0x7cu8];
        let result: Vec<ValType> = (&src[..]).convert()?;
        let expected = vec![
            ValType::I32(None),
            ValType::I64(None),
            ValType::F32(None),
            ValType::F64(None),
        ];
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_vec_valtype_empty() -> Result<(), String> {
        let src = vec![0x00u8];
        let result: Vec<ValType> = (&src[..]).convert()?;
        let expected: Vec<ValType> = vec![];
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_vec_functype() -> Result<(), String> {
        let src = vec![
            0x02u8, // num functypes
            0x60u8, 0x02u8, 0x7fu8, 0x7fu8, 0x01u8, 0x7eu8, // functype 0
            0x60u8, 0x02u8, 0x7du8, 0x7du8, 0x01u8, 0x7cu8, // functype 1
        ];
        let result: Vec<FuncType> = (&src[..]).convert()?;
        let expected = vec![
            FuncType::new(
                vec![ValType::I32(None), ValType::I32(None)],
                vec![ValType::I64(None)],
            ),
            FuncType::new(
                vec![ValType::F32(None), ValType::F32(None)],
                vec![ValType::F64(None)],
            ),
        ];
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_vec_functype_empty() -> Result<(), String> {
        let src = vec![0x00u8];
        let result: Vec<FuncType> = (&src[..]).convert()?;
        let expected: Vec<FuncType> = vec![];
        assert_eq!(expected, result);
        Ok(())
    }
}
