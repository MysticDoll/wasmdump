use crate::conventions::types::*;
use crate::converter::Converter;

impl Converter<TableType> for &[u8] {
    fn convert(&mut self) -> Result<TableType, String> {
        let elemtype: ElemType = self.convert()?;
        let limits: Limits = self.convert()?;

        Ok(TableType::new(elemtype, limits))
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::{ElemType, Limits, TableType};
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_tabletype() -> Result<(), String> {
        let src = vec![0x70, 0x00, 0x01];
        let result: TableType = (&src[..]).convert()?;
        let expected = TableType::new(ElemType::FuncRef, Limits::new(1, None));

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn convert_collectly_tabletype_with_max_limit() -> Result<(), String> {
        let src = vec![0x70, 0x01, 0x01, 0x02];
        let result: TableType = (&src[..]).convert()?;
        let expected = TableType::new(ElemType::FuncRef, Limits::new(1, Some(2)));

        assert_eq!(expected, result);
        Ok(())
    }
}
