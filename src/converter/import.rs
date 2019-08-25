use crate::conventions::types::{Import, ImportDesc};
use crate::converter::Converter;

impl Converter<Import> for &[u8] {
    fn convert(&mut self) -> Result<Import, String> {
        let r#mod: String = self.convert()?;
        let namespace: String = self.convert()?;
        let import_desc: ImportDesc = self.convert()?;

        Ok(Import::new(r#mod, namespace, import_desc))
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::{Import, ImportDesc};
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_otaku_otaku_func() -> Result<(), String> {
        let src = vec![
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku; import module name
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku  ; import field name
            0x00u8, // ; import kind
            0x00u8, // ; import signature index
        ];
        let result: Import = (&src[..]).convert()?;
        let expected = Import::new("otaku".to_owned(), "otaku".to_owned(), ImportDesc::Func(0));

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_otaku_otaku_table() -> Result<(), String> {
        let src = vec![
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku; import module name
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku  ; import field name
            0x01u8, // ; import kind
            0x00u8, // ; import signature index
        ];
        let result: Import = (&src[..]).convert()?;
        let expected = Import::new("otaku".to_owned(), "otaku".to_owned(), ImportDesc::Table(0));

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_otaku_otaku_memory() -> Result<(), String> {
        let src = vec![
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku; import module name
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku  ; import field name
            0x02u8, // ; import kind
            0x00u8, // ; import signature index
        ];
        let result: Import = (&src[..]).convert()?;
        let expected = Import::new(
            "otaku".to_owned(),
            "otaku".to_owned(),
            ImportDesc::Memory(0),
        );

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_otaku_otaku_global() -> Result<(), String> {
        let src = vec![
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku; import module name
            0x05u8, // ; string length
            0x6fu8, 0x74u8, 0x61u8, 0x6bu8, 0x75u8, // otaku  ; import field name
            0x03u8, // ; import kind
            0x00u8, // ; import signature index
        ];
        let result: Import = (&src[..]).convert()?;
        let expected = Import::new(
            "otaku".to_owned(),
            "otaku".to_owned(),
            ImportDesc::Global(0),
        );

        assert_eq!(expected, result);

        Ok(())
    }
}
