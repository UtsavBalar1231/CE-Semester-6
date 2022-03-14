pub static MAX: u16 = 3999;

#[derive(Debug, Clone)]
pub enum RomanSymbols {
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}

impl RomanSymbols {
    pub fn value(&self) -> u16 {
        match self {
            RomanSymbols::I => 1,
            RomanSymbols::IV => 4,
            RomanSymbols::V => 5,
            RomanSymbols::IX => 9,
            RomanSymbols::X => 10,
            RomanSymbols::XL => 40,
            RomanSymbols::L => 50,
            RomanSymbols::XC => 90,
            RomanSymbols::C => 100,
            RomanSymbols::CD => 400,
            RomanSymbols::D => 500,
            RomanSymbols::CM => 900,
            RomanSymbols::M => 1000,
        }
    }

    pub fn from_u16(value: u16) -> RomanSymbols {
        match value {
            1 => RomanSymbols::I,
            4 => RomanSymbols::IV,
            5 => RomanSymbols::V,
            9 => RomanSymbols::IX,
            10 => RomanSymbols::X,
            40 => RomanSymbols::XL,
            50 => RomanSymbols::L,
            90 => RomanSymbols::XC,
            100 => RomanSymbols::C,
            400 => RomanSymbols::CD,
            500 => RomanSymbols::D,
            900 => RomanSymbols::CM,
            1000 => RomanSymbols::M,
            _ => panic!("Invalid value, {:?}", value),
        }
    }

    pub fn from_str(value: &str) -> RomanSymbols {
        match value {
            "I" => RomanSymbols::I,
            "IV" => RomanSymbols::IV,
            "V" => RomanSymbols::V,
            "IX" => RomanSymbols::IX,
            "X" => RomanSymbols::X,
            "XL" => RomanSymbols::XL,
            "L" => RomanSymbols::L,
            "XC" => RomanSymbols::XC,
            "C" => RomanSymbols::C,
            "CD" => RomanSymbols::CD,
            "D" => RomanSymbols::D,
            "CM" => RomanSymbols::CM,
            "M" => RomanSymbols::M,
            _ => panic!("Invalid value, {:?}", value),
        }
    }

    pub fn symbols() -> Vec<String> {
        vec![
            "I".to_string(),
            "IV".to_string(),
            "V".to_string(),
            "IX".to_string(),
            "X".to_string(),
            "XL".to_string(),
            "L".to_string(),
            "XC".to_string(),
            "C".to_string(),
            "CD".to_string(),
            "D".to_string(),
            "CM".to_string(),
            "M".to_string(),
        ]
    }
    pub fn values() -> Vec<u16> {
        vec![1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000]
    }

    pub fn roman_to_int(value: &String) -> Option<u16> {
        let (mut n, mut result) = (0, 0);
        for i in value.chars().rev() {
            let v = RomanSymbols::from_str(&i.to_string());
            if v.value() >= n {
                result += v.value();
            } else {
                result -= v.value();
            }
            n = v.value();
        }
        if result > MAX {
            None
        } else {
            Some(result)
        }
    }

    pub fn roman_pairs() -> Vec<(String, u16)> {
        let symbols = RomanSymbols::symbols();
        let values = RomanSymbols::values();
        let mut result = Vec::new();
        for (i, symbol) in symbols.iter().enumerate() {
            result.push((symbol.to_string(), values[i]));
        }
        result.reverse();
        result
    }

    pub fn int_to_roman(n: u16) -> Option<String> {
        if n <= 0 || n > MAX {
            panic!("Argument must be between 1 and 3999");
        }
        let mut result = String::new();
        let mut n = n;
        for (symbol, value) in RomanSymbols::roman_pairs() {
            while n >= value {
                n -= value;
                result.push_str(symbol.as_str());
            }
        }
        Some(result)
    }
}
