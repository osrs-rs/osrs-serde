use serde::{ser::SerializeSeq, Deserialize, Serialize};

/// Test U16Smart
///
/// # Examples
///
/// Serialize a struct with a U16Smart
///
/// ```
/// use osrs_serde::U16Smart;
/// use serde::{Serialize};
///
/// #[derive(Serialize)]
/// struct TestU16Smart {
///    a: U16Smart
/// }
///
/// let packet = TestU16Smart { a: U16Smart(234) };
/// assert_eq!(bincode::serialize(&packet).unwrap(), [234,128]);
/// ```
#[derive(Debug, Deserialize)]
pub struct U16Smart(pub u16);

impl Serialize for U16Smart {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            0..=127 => serializer.serialize_u8(self.0 as u8),
            128..=32767 => serializer.serialize_u16(self.0 + 32768),
            _ => Err(serde::ser::Error::custom("U16Smart value out of range")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct U8Add(pub u8);

impl Serialize for U8Add {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.0.wrapping_add(128))
    }
}

#[derive(Debug, Deserialize)]
pub struct U16Le(u16);

// TODO: Swap whole impl to use big endian and then use "to_le_bytes" here instead
// Serialize the u16 as big endian
impl Serialize for U16Le {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(self.0.to_be())
    }
}

#[derive(Debug, Deserialize)]
pub struct I32IME(i32);

// TODO
// Serialize the u16 as big endian
impl Serialize for I32IME {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Serialize one at a time with bitshifts
        serializer.serialize_i32(self.0)
    }
}

/// Test StringCp1252
///
/// # Examples
///
/// Serialize a struct with a StringCp1252
///
/// ```
/// use osrs_serde::StringCp1252;
/// use serde::{Serialize};
///
/// #[derive(Serialize)]
/// struct TestStringCp1252 {
///   a: StringCp1252
/// }
///
/// let packet = TestStringCp1252 { a: StringCp1252("Hello World!".to_string()) };
/// assert_eq!(bincode::serialize(&packet).unwrap(), [12, 0, 0, 0, 0, 0, 0, 0,72,101,108,108,111,32,87,111,114,108,100,33]);
/// ```
#[derive(Debug, Deserialize)]
pub struct StringCp1252(pub String);

impl Serialize for StringCp1252 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for b in self.0.bytes() {
            seq.serialize_element(&b)?;
        }
        seq.end()
    }
}
