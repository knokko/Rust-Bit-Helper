





pub fn booleans_to_u8(b1: bool, b2: bool, b3:bool, b4:bool, b5:bool, b6:bool, b7:bool, b8:bool) -> u8 {
    let mut result: u8 = 0;
    if b1 {
        result += 128;
    }
    if b2 {
        result += 64;
    }
    if b3 {
        result += 32;
    }
    if b4 {
        result += 16;
    }
    if b5 {
        result += 8;
    }
    if b6 {
        result += 4;
    }
    if b7 {
        result += 2;
    }
    if b8 {
        result += 1;
    }
    result
}

pub fn boolean_tuple_to_u8(bools: (bool,bool,bool,bool,bool,bool,bool,bool)) -> u8 {
    let mut result: u8 = 0;
    if bools.0 {
        result += 128;
    }
    if bools.1 {
        result += 64;
    }
    if bools.2 {
        result += 32;
    }
    if bools.3 {
        result += 16;
    }
    if bools.4 {
        result += 8;
    }
    if bools.5 {
        result += 4;
    }
    if bools.6 {
        result += 2;
    }
    if bools.7 {
        result += 1;
    }
    result
}

pub fn boolean_array_to_u8(bools: [bool; 8]) -> u8 {
    let mut result: u8 = 0;
    if bools[0] {
        result += 128;
    }
    if bools[1] {
        result += 64;
    }
    if bools[2] {
        result += 32;
    }
    if bools[3] {
        result += 16;
    }
    if bools[4] {
        result += 8;
    }
    if bools[5] {
        result += 4;
    }
    if bools[6] {
        result += 2;
    }
    if bools[7] {
        result += 1;
    }
    result
}

pub fn u8_to_booleans(mut byte: u8) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let mut result = (false,false,false,false,false,false,false,false);
    if byte >= 128 {
        result.0 = true;
        byte -= 128;
    }
    if byte >= 64 {
        result.1 = true;
        byte -= 64;
    }
    if byte >= 32 {
        result.2 = true;
        byte -= 32;
    }
    if byte >= 16 {
        result.3 = true;
        byte -= 16;
    }
    if byte >= 8 {
        result.4 = true;
        byte -= 8;
    }
    if byte >= 4 {
        result.5 = true;
        byte -= 4;
    }
    if byte >= 2 {
        result.6 = true;
        byte -= 2;
    }
    result.7 = byte == 1;
    result
}

pub fn u8_to_boolean_array(mut byte: u8) -> [bool; 8] {
    let mut result = [false; 8];
    if byte >= 128 {
        result[0] = true;
        byte -= 128;
    }
    if byte >= 64 {
        result[1] = true;
        byte -= 64;
    }
    if byte >= 32 {
        result[2] = true;
        byte -= 32;
    }
    if byte >= 16 {
        result[3] = true;
        byte -= 16;
    }
    if byte >= 8 {
        result[4] = true;
        byte -= 8;
    }
    if byte >= 4 {
        result[5] = true;
        byte -= 4;
    }
    if byte >= 2 {
        result[6] = true;
        byte -= 2;
    }
    result[7] = byte == 1;
    result
}

pub fn i8s_to_i32(byte1: i8, byte2: i8, byte3: i8, byte4: i8) -> i32 {
    (((byte4 as i32) << 24) | ((byte3 as i32 & 0xff) << 16) | ((byte2 as i32 & 0xff) << 8) | ((byte1 as i32 & 0xff)))
}

pub fn i32_to_i8_1(int: i32) -> i8 {
    int as i8
}

pub fn i32_to_i8_2(int: i32) -> i8 {
    (int >> 8) as i8
}

pub fn i32_to_i8_3(int: i32) -> i8 {
    (int >> 16) as i8
}

pub fn i32_to_i8_4(int: i32) -> i8 {
    (int >> 24) as i8
}

pub mod converter;

#[cfg(test)]
mod tests {

    use crate::converter::*;

    #[test]
    fn int8s_to_booleans() {
        init();
        for element in -128i8..127i8 {
            let boolean_tuple = i8_to_boolean_tuple(element);
            let boolean_array = i8_to_boolean_array(element);

            let reverted1 = boolean_tuple_to_i8(boolean_tuple);
            let reverted2 = booleans_to_i8(boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7);
            let reverted3 = boolean_array_to_i8([boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7]);
            let reverted4 = boolean_slice_to_i8(&[boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7]);

            let reverted5 = boolean_tuple_to_i8((boolean_array[0], boolean_array[1], boolean_array[2], boolean_array[3], boolean_array[4], boolean_array[5], boolean_array[6], boolean_array[7]));
            let reverted6 = booleans_to_i8(boolean_array[0], boolean_array[1], boolean_array[2], boolean_array[3], boolean_array[4], boolean_array[5], boolean_array[6], boolean_array[7]);
            let reverted7 = boolean_array_to_i8(boolean_array);
            let reverted8 = boolean_slice_to_i8(&boolean_array);

            assert_eq!(element, reverted1);
            assert_eq!(element, reverted2);
            assert_eq!(element, reverted3);
            assert_eq!(element, reverted4);
            assert_eq!(element, reverted5);
            assert_eq!(element, reverted6);
            assert_eq!(element, reverted7);
            assert_eq!(element, reverted8);
        }
    }
}