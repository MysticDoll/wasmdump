use crate::conventions::types::*;
use crate::converter::Converter;
use std::io::Read;

impl Converter<Limits> for &[u8] {
    fn convert(&mut self) -> Result<Limits, String> {
        let mut prefix: [u8; 1] = [0; 1];
        self.read_exact(&mut prefix)
            .map_err(|_| "I/O Error occured".to_owned())?;

        match prefix {
            [0x00u8] => {
                let min = leb128::read::unsigned(self)
                    .map(|m| m as u32)
                    .map_err(|_| "Error occured when read leb128 param of Limits.min".to_owned())?;

                Ok(Limits::new(min, None))
            }
            [0x01u8] => {
                let min: u32 = leb128::read::unsigned(self)
                    .map(|m| m as u32)
                    .map_err(|_| "Error occured when read leb128 param of Limits.min".to_owned())?;
                let max: Option<u32> = leb128::read::unsigned(self)
                    .map(|m| Some(m as u32))
                    .map_err(|_| "Error occured when read leb128 param of Limits.max".to_owned())?;

                if min > max.unwrap() {
                    return Err(
                        "Max value of Limit must be lager than the min value of them".to_owned(),
                    );
                }
                Ok(Limits::new(min, max))
            }
            _ => Err("Invalid prefix for Limit struct".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::conventions::types::Limits;
    use crate::converter::Converter;

    #[test]
    fn convert_collectly_min_1_and_max_epsilon() -> Result<(), String> {
        let src: Vec<u8> = vec![0x00u8, 0x01u8];
        let result: Limits = (&src[..]).convert()?;
        let expected: Limits = Limits::new(1, None);

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_min_1_and_max_1() -> Result<(), String> {
        let src: Vec<u8> = vec![0x01u8, 0x01u8, 0x01u8];
        let result: Limits = (&src[..]).convert()?;
        let expected: Limits = Limits::new(1, Some(1));

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_collectly_min_1_and_max_2() -> Result<(), String> {
        let src: Vec<u8> = vec![0x01u8, 0x01u8, 0x02u8];
        let result: Limits = (&src[..]).convert()?;
        let expected: Limits = Limits::new(1, Some(2));

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn convert_incollectly_min_2_and_max_1() -> Result<(), String> {
        let src: Vec<u8> = vec![0x01u8, 0x02u8, 0x01u8];
        let result: Result<Limits, String> = (&src[..]).convert().map_err(|e| e.to_owned());
        let expected = "Max value of Limit must be lager than the min value of them".to_owned();

        if !result.is_err() {
            panic!("Invalid behavior: Max value of Limit must be lager than min value")
        }

        assert_eq!(expected, result.unwrap_err());
        Ok(())
    }

}
