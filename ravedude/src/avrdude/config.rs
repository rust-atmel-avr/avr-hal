use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

fn serialize_baudrate<S>(val: &Option<Option<NonZeroU32>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let baudrate = val.as_ref().map(|val| val.map_or(-1, |x| x.get() as i32));

    baudrate.serialize(serializer)
}

fn deserialize_baudrate<'de, D>(deserializer: D) -> Result<Option<Option<NonZeroU32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(match Option::<i32>::deserialize(deserializer)? {
        None => None,
        Some(-1) => Some(None),
        Some(baudrate) => Some(Some(NonZeroU32::new(baudrate as _).ok_or_else(|| {
            serde::de::Error::custom(format!("invalid baudrate: {baudrate}"))
        })?)),
    })
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardAvrdudeOptions {
    pub programmer: Option<String>,
    pub partno: Option<String>,
    #[serde(
        serialize_with = "serialize_baudrate",
        deserialize_with = "deserialize_baudrate"
    )]
    // Inner option to represent whether the baudrate exists, outer option to allow for overriding.
    // Option<if baudrate == -1 { None } else { NonZeroU32(baudrate) }>
    pub baudrate: Option<Option<NonZeroU32>>,
    pub do_chip_erase: Option<bool>,
}
impl BoardAvrdudeOptions {
    pub fn merge(self, base: Self) -> Self {
        Self {
            programmer: self.programmer.or(base.programmer),
            partno: self.partno.or(base.partno),
            baudrate: self.baudrate.or(base.baudrate),
            do_chip_erase: self.do_chip_erase.or(base.do_chip_erase),
        }
    }
}

