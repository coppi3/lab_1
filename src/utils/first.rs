use std::collections::HashMap;
use std::fmt;

//function that encodes string with given table
pub fn encode(string: &str) -> Vec<u8> {
    let dict: HashMap<char, u8> = HashMap::from([
        ('А', 0xC0),
        ('Б', 0xC1),
        ('В', 0xC2),
        ('Г', 0xC3),
        ('Д', 0xC4),
        ('Е', 0xC5),
        ('Ж', 0xC6),
        ('З', 0xC7),
        ('И', 0xC8),
        ('Й', 0xC9),
        ('К', 0xCA),
        ('Л', 0xCB),
        ('М', 0xCC),
        ('Н', 0xCD),
        ('О', 0xCE),
        ('П', 0xCF),
        ('Р', 0xD0),
        ('С', 0xD1),
        ('Т', 0xD2),
        ('У', 0xD3),
        ('Ф', 0xD4),
        ('Х', 0xD5),
        ('Ц', 0xD6),
        ('Ч', 0xD7),
        ('Ш', 0xD8),
        ('Щ', 0xD9),
        ('Ъ', 0xDA),
        ('Ы', 0xDB),
        ('Ь', 0xDC),
        ('Э', 0xDD),
        ('Ю', 0xDE),
        ('Я', 0xDF),
        ('а', 0xE0),
        ('б', 0xE1),
        ('в', 0xE2),
        ('г', 0xE3),
        ('д', 0xE4),
        ('е', 0xE5),
        ('ж', 0xE6),
        ('з', 0xE7),
        ('и', 0xE8),
        ('й', 0xE9),
        ('к', 0xEA),
        ('л', 0xEB),
        ('м', 0xEC),
        ('н', 0xED),
        ('о', 0xEE),
        ('п', 0xEF),
        ('р', 0xF0),
        ('с', 0xF1),
        ('т', 0xF2),
        ('у', 0xF3),
        ('ф', 0xF4),
        ('х', 0xF5),
        ('ц', 0xF6),
        ('ч', 0xF7),
        ('ш', 0xF8),
        ('щ', 0xF9),
        ('ъ', 0xFA),
        ('ы', 0xFB),
        ('ь', 0xFC),
        ('э', 0xFD),
        ('ю', 0xFE),
        ('я', 0xFF),
        (' ', 0x20),
        (',', 0x2C),
        ('.', 0x2E),
        ('0', 0x30),
        ('1', 0x31),
        ('2', 0x32),
        ('3', 0x33),
        ('4', 0x34),
        ('5', 0x35),
        ('6', 0x36),
        ('7', 0x37),
        ('8', 0x38),
        ('9', 0x39),
    ]);
    let chars = string.to_string();
    let mut encoded: Vec<u8> = Vec::with_capacity(string.chars().count());
    for i in chars.chars() {
        encoded.push(dict[&i]);
    }
    encoded
}

//copied from github to print arrays in binary
pub struct V<'a>(pub &'a Vec<u8>);

// custom output
impl fmt::Binary for V<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the value using tuple idexing
        // and create reference to 'vec'
        let vec = &self.0;

        // @count -> the index of the value,
        // @n     -> the value
        for (count, n) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, " ")?;
            }

            write!(f, "{:08b}", n)?;
        }

        Ok(())
    }
}

//changed binary trait to print UpperHex
impl fmt::UpperHex for V<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the value using tuple idexing
        // and create reference to 'vec'
        let vec = &self.0;

        // @count -> the index of the value,
        // @n     -> the value
        for (count, n) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, " ")?;
            }

            write!(f, "{:X}", n)?;
        }

        Ok(())
    }
}
