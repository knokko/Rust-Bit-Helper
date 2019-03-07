pub mod converter;
pub mod output;
pub mod input;

#[cfg(test)]
mod tests {

    use crate::converter::*;
    use crate::output::*;
    use crate::input::*;

    #[test]
    fn int8s_to_booleans() {

        // Only 256 possible values, so just test them all
        for element in -128i8..=127i8 {
            let boolean_tuple = i8_to_bool_tuple(element);
            let boolean_array = i8_to_bool_array(element);

            let reverted1 = bool_tuple_to_i8(boolean_tuple);
            let reverted2 = bools_to_i8(boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7);
            let reverted3 = bool_array_to_i8([boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7]);
            let reverted4 = bool_slice_to_i8(&[boolean_tuple.0, boolean_tuple.1, boolean_tuple.2, boolean_tuple.3, boolean_tuple.4, boolean_tuple.5, boolean_tuple.6, boolean_tuple.7]);

            let reverted5 = bool_tuple_to_i8((boolean_array[0], boolean_array[1], boolean_array[2], boolean_array[3], boolean_array[4], boolean_array[5], boolean_array[6], boolean_array[7]));
            let reverted6 = bools_to_i8(boolean_array[0], boolean_array[1], boolean_array[2], boolean_array[3], boolean_array[4], boolean_array[5], boolean_array[6], boolean_array[7]);
            let reverted7 = bool_array_to_i8(boolean_array);
            let reverted8 = bool_slice_to_i8(&boolean_array);

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

        // Only 256 possible values, so just test them all
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

        // I can't imagine a better way to test the conversion of 16-bit numbers than just testing them all
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

    #[test]
    fn test_i8_to_i32() {

        // 4 billion tests is not so nice, so let's skip some values...
        let mut counter = 0;
        let mut int = -2147483648;
        while counter < 34000 {
            let byte1 = i32_to_i8_1(int);
            let byte2 = i32_to_i8_2(int);
            let byte3 = i32_to_i8_3(int);
            let byte4 = i32_to_i8_4(int);
            let pair = i32_to_i8_tuple(int);
            let array = i32_to_i8_array(int);

            assert_eq!(byte1, pair.0);
            assert_eq!(byte2, pair.1);
            assert_eq!(byte3, pair.2);
            assert_eq!(byte4, pair.3);
            assert_eq!(byte1, array[0]);
            assert_eq!(byte2, array[1]);
            assert_eq!(byte3, array[2]);
            assert_eq!(byte4, array[3]);

            let reverted1 = i8s_to_i32(byte1, byte2, byte3, byte4);
            let reverted2 = i8_tuple_to_i32(pair);
            let reverted3 = i8_array_to_i32(array);
            let reverted4 = i8_slice_to_i32(&array);

            assert_eq!(int, reverted1);
            assert_eq!(int, reverted2);
            assert_eq!(int, reverted3);
            assert_eq!(int, reverted4);

            int += 123456;
            counter += 1;
        }
    }

    #[test]
    fn test_bool_array_bit_io(){
        let mut output = BoolVecBitOutput::new(10);
        println!("Initial output is {:?}", output);
        output.ensure_extra_capacity(20);
        println!("Increased capacity output is {:?}", output);
        output.add_direct_bool(true);
        output.add_direct_bool(false);
        output.add_direct_i8(120);
        output.add_direct_bools(&[true,false,true,false,true]);
        println!("After saving some data: {:?}", output);
    }

    #[test]
    fn test_integer_to_bools(){
        let mut integer = -9223372036854775808;
        while integer < 23738474347634 {
            let mut as_bools = [false; 64];
            signed_int_to_bools(integer, 64, &mut as_bools, 0);
            let reverted = bools_to_signed_int(64, &as_bools, 0);
            assert_eq!(integer, reverted);

            integer += 9538274823127357;
        }
    }
}