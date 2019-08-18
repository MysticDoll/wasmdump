use crate::conventions::types::*;
use crate::converter::Converter;
use std::io::Read;

impl From<u8> for ElemType {
    fn from(from: u8) -> ElemType {
        match from {
            0x70 => ElemType::FuncRef,
            _ => panic!("Invalid elemtype error"),
        }
    }
}

impl Converter<ElemType> for &[u8] {
    fn convert(&mut self) -> Result<ElemType, String> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf)
            .map_err(|_| "I/O Error occured".to_owned())?;
        Ok(buf[0].into())
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::ElemType;
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_funcref() -> Result<(), String> {
        let src: Vec<u8> = vec![0x70u8];
        let result: ElemType = (&src[..]).convert()?;

        assert_eq!(ElemType::FuncRef, result);
        Ok(())
    }

}
