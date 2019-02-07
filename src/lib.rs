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
        for element in -128i8..=127i8 {
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

    #[test]
    fn uint8s_to_booleans() {
        let mut l: u64 = 1;
        for _ in 0..63 {
            print!("{}, ", l);
            l *= 2;
        }
        for element in 0u8..=255 {
            let boolean_tuple = u8_to_boolean_tuple(element);
            let boolean_array = u8_to_boolean_array(element);

            let reverted1 = boolean_tuple_to_u8(boolean_tuple);
            let reverted2 = booleans_to_u8(boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7);
            let reverted3 = boolean_array_to_u8([boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7]);
            let reverted4 = boolean_slice_to_u8(&[boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7]);

            let reverted5 = boolean_tuple_to_u8((boolean_array[0], boolean_array[1], boolean_array[2], boolean_array[3], boolean_array[4], boolean_array[5], boolean_array[6], boolean_array[7]));
            let reverted6 = booleans_to_u8(boolean_array[0], boolean_array[1], boolean_array[2], boolean_array[3], boolean_array[4], boolean_array[5], boolean_array[6], boolean_array[7]);
            let reverted7 = boolean_array_to_u8(boolean_array);
            let reverted8 = boolean_slice_to_u8(&boolean_array);

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

    #[test]
    fn test_i8_to_i16() {
        for short in -32768i16..=32767 {
            let byte1 = i16_to_i8_1(short);
            let byte2 = i16_to_i8_2(short);
            let pair = i16_to_i8_tuple(short);
            let array = i16_to_i8_array(short);

            assert_eq!(byte1, pair.0);
            assert_eq!(byte2, pair.1);
            assert_eq!(byte1, array[0]);
            assert_eq!(byte2, array[1]);

            let reverted1 = i8s_to_i16(byte1, byte2);
            let reverted2 = i8_tuple_to_i16(pair);
            let reverted3 = i8_array_to_i16(array);
            let reverted4 = i8_slice_to_i16(&array);

            assert_eq!(short, reverted1);
            assert_eq!(short, reverted2);
            assert_eq!(short, reverted3);
            assert_eq!(short, reverted4);
        }
    }
}