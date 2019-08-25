use crate::conventions::types::ImportDesc;
use crate::converter::Converter;
use std::io::Read;

impl Converter<ImportDesc> for &[u8] {
    fn convert(&mut self) -> Result<ImportDesc, String> {
        let mut type_buf: [u8; 1] = [0; 1];
        self.read_exact(&mut type_buf)
            .map_err(|_| "I/O error occured".to_owned())?;
        let index = leb128::read::unsigned(self)
            .map_err(|_| "Error occured in reading leb128 value: importdesc index")?
            as u32;

        match type_buf {
            [0x00u8] => Ok(ImportDesc::Func(index)),
            [0x01u8] => Ok(ImportDesc::Table(index)),
            [0x02u8] => Ok(ImportDesc::Memory(index)),
            [0x03u8] => Ok(ImportDesc::Global(index)),
            _ => Err("Invalid import type index".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::ImportDesc;
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_importdesc_func() -> Result<(), String> {
        let src = vec![0x00u8, 0x00u8];
        let result: ImportDesc = (&src[..]).convert()?;
        let expected = ImportDesc::Func(0);

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_importdesc_table() -> Result<(), String> {
        let src = vec![0x01u8, 0x00u8];
        let result: ImportDesc = (&src[..]).convert()?;
        let expected = ImportDesc::Table(0);

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_importdesc_memory() -> Result<(), String> {
        let src = vec![0x02u8, 0x00u8];
        let result: ImportDesc = (&src[..]).convert()?;
        let expected = ImportDesc::Memory(0);

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_importdesc_global() -> Result<(), String> {
        let src = vec![0x03u8, 0x00u8];
        let result: ImportDesc = (&src[..]).convert()?;
        let expected = ImportDesc::Global(0);

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_incollectly_importdesc_undefined() -> Result<(), String> {
        let src = vec![0x04u8, 0x00u8];
        let result: Result<ImportDesc, String> = (&src[..]).convert();

        assert!(result.is_err());
        Ok(())
    }
}
