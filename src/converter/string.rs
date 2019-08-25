use crate::converter::Converter;
use std::io::Read;

impl Converter<String> for &[u8] {
    fn convert(&mut self) -> Result<String, String> {
        let length = leb128::read::unsigned(self)
            .map_err(|_| "Error occured in reading leb128 value: string length".to_owned())?
            as usize;
        let mut string_buffer: Vec<u8> = vec![0; length];
        self.read_exact(&mut string_buffer)
            .map_err(|_| "I/O Error occured in read string".to_owned())?;
        String::from_utf8(string_buffer).map_err(|_| "Convert string from utf8 failed".to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_otaku_string() -> Result<(), String> {
        let src = vec![0x05u8, 0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8];
        let result: String = (&src[..]).convert()?;
        let expected = "otaku".to_owned();

        assert_eq!(expected, result);
        Ok(())
    }

}
