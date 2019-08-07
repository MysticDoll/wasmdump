use crate::conventions::types::*;
use crate::converter::Converter;
use std::io::Read;

impl Converter<FuncType> for &[u8] {
    fn convert(&mut self) -> Result<FuncType, &str> {
        let mut prefix: [u8; 1] = [0; 1];
        self.read_exact(&mut prefix)
            .map_err(|_| "I/O Error occured")?;
        if Some(0x60u8) != prefix.first().map(|i| i.clone()) {
            return Err("Invalid type declare. Type definition must have prefix 0x60");
        }
        let param_count: usize = leb128::read::unsigned(self)
            .map_err(|_| "Error occured when read leb128 functype params")?
            as usize;

        let mut params_vec: Vec<u8> = vec![0; param_count];
        self.read_exact(&mut params_vec)
            .map_err(|_| "Read the count of params failed")?;
        let params: Vec<ValType> = params_vec.iter().map(|i| i.clone().into()).collect();
        let result_count: usize = leb128::read::unsigned(self)
            .map_err(|_| "Error occured when read leb128 functype params")?
            as usize;
        let mut results_vec: Vec<u8> = vec![0; result_count];
        self.read_exact(&mut results_vec)
            .map_err(|_| "Read the count of results failed")?;
        let results: Vec<ValType> = results_vec.iter().map(|i| i.clone().into()).collect();

        Ok(FuncType::new(params, results))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::*;
    use crate::conventions::types::*;
    use crate::converter::Converter;
    use std::io::Read;

    #[test]
    fn convert_collectly_nil_to_nil() -> Result<(), String> {
        let mut src: Vec<u8> = vec![0x60u8, 0x00u8, 0x00u8];
        let result: FuncType = (&src[..]).convert()?;
        let expected: FuncType = FuncType::new(Vec::new(), Vec::new());

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i32_to_i32() -> Result<(), String> {
        let mut src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7fu8, 0x01u8, 0x7fu8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::I32(None)];
        let results = vec![ValType::I32(None)];
        let expected: FuncType = FuncType::new(params, results);

        assert_eq!(expected, result);
        Ok(())
    }
    #[test]
    fn convert_collectly_i32_to_nil() -> Result<(), String> {
        let mut src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7fu8, 0x00u8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::I32(None)];
        let expected: FuncType = FuncType::new(params, Vec::new());

        assert_eq!(expected, result);
        Ok(())
    }

}
