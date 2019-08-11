use crate::converter::Converter;
use std::io::Read;

impl<T, S> Converter<Vec<T>> for S
where
    T: Clone,
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
        let result: Result<Vec<ValType>, String> = (&src[..]).convert();
        let expected = vec![
            ValType::I32(None),
            ValType::I64(None),
            ValType::F32(None),
            ValType::F64(None),
        ];
        assert_eq!(expected, result.unwrap());
        Ok(())
    }
}
