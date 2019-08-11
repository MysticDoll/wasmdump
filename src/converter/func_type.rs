use crate::conventions::types::*;
use crate::converter::Converter;
use std::io::Read;

impl Converter<FuncType> for &[u8] {
    fn convert(&mut self) -> Result<FuncType, String> {
        let mut prefix: [u8; 1] = [0; 1];
        self.read_exact(&mut prefix)
            .map_err(|_| "I/O Error occured".to_owned())?;
        if Some(0x60u8) != prefix.first().map(|i| i.clone()) {
            return Err("Invalid type declare. Type definition must have prefix 0x60".to_owned());
        }
        let param_count: usize = leb128::read::unsigned(self)
            .map_err(|_| "Error occured when read leb128 functype params".to_owned())?
            as usize;

        let mut params_vec: Vec<u8> = vec![0; param_count];
        self.read_exact(&mut params_vec)
            .map_err(|_| "Read the count of params failed".to_owned())?;
        let params: Vec<ValType> = params_vec.iter().map(|&i| i.into()).collect();
        let result_count: usize = leb128::read::unsigned(self)
            .map_err(|_| "Error occured when read leb128 functype params".to_owned())?
            as usize;

        if result_count > 1 {
            return Err("Current WebAssembly vesion does not allows multi-value return".to_owned());
        }

        let mut results_vec: Vec<u8> = vec![0; result_count];
        self.read_exact(&mut results_vec)
            .map_err(|_| "Read the count of results failed".to_owned())?;
        let results: Vec<ValType> = results_vec.iter().map(|&i| i.into()).collect();

        Ok(FuncType::new(params, results))
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::{FuncType, ValType};
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_nil_to_nil() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x00u8, 0x00u8];
        let result: FuncType = (&src[..]).convert()?;
        let expected: FuncType = FuncType::new(Vec::new(), Vec::new());

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i32_i32_to_i32() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x02u8, 0x7fu8, 0x7fu8, 0x01u8, 0x7fu8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::I32(None), ValType::I32(None)];
        let results = vec![ValType::I32(None)];
        let expected: FuncType = FuncType::new(params, results);

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_incollectly_i32_to_i32_i32() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7fu8, 0x02u8, 0x7fu8, 0x7fu8];
        let expected = "Current WebAssembly vesion does not allows multi-value return".to_owned();
        let result: Result<FuncType, String> = (&src[..]).convert().map_err(|e| e.to_owned());
        if let Err(msg) = result {
            assert_eq!(expected, msg);
            return Ok(());
        } else {
            panic!("Invalid behavior: Current WebAssembly cannot use multi-value return")
        };
    }

    #[test]
    fn convert_collectly_i32_to_i32() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7fu8, 0x01u8, 0x7fu8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::I32(None)];
        let results = vec![ValType::I32(None)];
        let expected: FuncType = FuncType::new(params, results);

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i64_to_i64() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7eu8, 0x01u8, 0x7eu8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::I64(None)];
        let results = vec![ValType::I64(None)];
        let expected: FuncType = FuncType::new(params, results);

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f32_to_f32() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7du8, 0x01u8, 0x7du8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::F32(None)];
        let results = vec![ValType::F32(None)];
        let expected: FuncType = FuncType::new(params, results);

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_f64_to_f64() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7cu8, 0x01u8, 0x7cu8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::F64(None)];
        let results = vec![ValType::F64(None)];
        let expected: FuncType = FuncType::new(params, results);

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_i32_to_nil() -> Result<(), String> {
        let src: Vec<u8> = vec![0x60u8, 0x01u8, 0x7fu8, 0x00u8];
        let result: FuncType = (&src[..]).convert()?;
        let params = vec![ValType::I32(None)];
        let expected: FuncType = FuncType::new(params, Vec::new());

        assert_eq!(expected, result);
        Ok(())
    }

}
