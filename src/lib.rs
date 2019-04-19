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
    fn test_u8_to_i16() {

        // I can't imagine a better way to test the conversion of 16-bit numbers than just testing them all
        for short in -32768i16..=32767 {
            let byte1 = i16_to_u8_1(short);
            let byte2 = i16_to_u8_2(short);
            let pair = i16_to_u8_tuple(short);
            let array = i16_to_u8_array(short);

            assert_eq!(byte1, pair.0);
            assert_eq!(byte2, pair.1);
            assert_eq!(byte1, array[0]);
            assert_eq!(byte2, array[1]);

            let reverted1 = u8s_to_i16(byte1, byte2);
            let reverted2 = u8_tuple_to_i16(pair);
            let reverted3 = u8_array_to_i16(array);
            let reverted4 = u8_slice_to_i16(&array);

            assert_eq!(short, reverted1);
            assert_eq!(short, reverted2);
            assert_eq!(short, reverted3);
            assert_eq!(short, reverted4);
        }
    }

    #[test]
    fn test_u8_to_i32() {

        // 4 billion tests is not so nice, so let's skip some values...
        let mut counter = 0;
        let mut int = -2147483648;
        while counter < 34000 {
            let byte1 = i32_to_u8_1(int);
            let byte2 = i32_to_u8_2(int);
            let byte3 = i32_to_u8_3(int);
            let byte4 = i32_to_u8_4(int);
            let pair = i32_to_u8_tuple(int);
            let array = i32_to_u8_array(int);

            assert_eq!(byte1, pair.0);
            assert_eq!(byte2, pair.1);
            assert_eq!(byte3, pair.2);
            assert_eq!(byte4, pair.3);
            assert_eq!(byte1, array[0]);
            assert_eq!(byte2, array[1]);
            assert_eq!(byte3, array[2]);
            assert_eq!(byte4, array[3]);

            let reverted1 = u8s_to_i32(byte1, byte2, byte3, byte4);
            let reverted2 = u8_tuple_to_i32(pair);
            let reverted3 = u8_array_to_i32(array);
            let reverted4 = u8_slice_to_i32(&array);

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
        put_stuff_in_bit_output(&mut output);
        let mut input = BoolSliceBitInput::new(output.get_slice());
        check_stuff_in_bit_input(&mut input);
    }

    #[test]
    fn test_i8_vec_bit_io(){
        let mut output = I8VecBitOutput::with_capacity(10);
        put_stuff_in_bit_output(&mut output);
        output.terminate();
        let mut input = I8VecBitInput::new(output.vector);
        check_stuff_in_bit_input(&mut input);
        input.terminate();
    }

    #[test]

    fn test_i8_vec_bit_output_capacity(){
        let mut output = I8VecBitOutput::with_capacity(0);
        assert_eq!(output.vector.capacity(), 0);
        output.ensure_extra_capacity(64);
        let old_capacity = output.vector.capacity();
        assert!(old_capacity >= 8);
        output.add_i32(474857);
        output.add_u32(3478);
        assert_eq!(old_capacity, output.vector.capacity());

        output = I8VecBitOutput::with_capacity(4);
        assert_eq!(output.vector.capacity(), 4);
        output.add_i32(-18273);
        assert_eq!(output.vector.capacity(), 4);

        assert_eq!(output.to_i8_vector().len(), 4);
        output.add_bool(true);
        assert_eq!(output.to_i8_vector().len(), 5);
    }

    #[test]
    fn test_u8_vec_bit_io(){
        let mut output = U8VecBitOutput::with_capacity(10);
        put_stuff_in_bit_output(&mut output);
        output.terminate();
        let mut input = U8VecBitInput::new(output.vector);
        check_stuff_in_bit_input(&mut input);
        input.terminate();
    }

    #[test]
    fn test_u8_vec_bit_output_capacity(){
        let mut output = U8VecBitOutput::with_capacity(0);
        assert_eq!(output.vector.capacity(), 0);
        output.ensure_extra_capacity(32);
        let old_capacity = output.vector.capacity();
        assert!(old_capacity >= 4);
        output.add_i8s_from_vec(&vec![23, -97, 100, 45]);
        assert_eq!(old_capacity, output.vector.capacity());

        output = U8VecBitOutput::with_capacity(4);
        assert_eq!(output.vector.capacity(), 4);
        output.add_direct_u16(54673);
        output.add_u16(27346);
        assert_eq!(output.vector.capacity(), 4);

        assert_eq!(output.to_u8_vector().len(), 4);
        output.add_bool(true);
        assert_eq!(output.to_u8_vector().len(), 5);
    }

    #[test]

    fn test_cross_i8_u8_vec_bit_io(){
        let mut u_output = U8VecBitOutput::with_capacity(10);
        put_stuff_in_bit_output(&mut u_output);
        u_output.terminate();
        unsafe {
            let i_vector = std::mem::transmute(u_output.vector);
            let mut i_input = I8VecBitInput::new(i_vector);
            check_stuff_in_bit_input(&mut i_input);
            i_input.terminate();
        }

        let mut i_output = I8VecBitOutput::with_capacity(10);
        put_stuff_in_bit_output(&mut i_output);
        i_output.terminate();
        unsafe {
            let u_vector = std::mem::transmute(i_output.vector);
            let mut u_input = U8VecBitInput::new(u_vector);
            check_stuff_in_bit_input(&mut u_input);
            u_input.terminate();
        }
    }

    fn put_stuff_in_bit_output(output: &mut BitOutput){
        output.add_bools_from_slice(&[false, true, true, false, true]);
        output.add_i8(-125);
        output.add_u8(234);
        output.add_i16(-21345);
        output.add_u16(25565);
        output.add_i32(2123456789);

        output.add_bool_slice(&[false, false, true, false, true, true]);
        output.add_bool_vec(&vec![true, true, false, false]);
        output.add_bools_from_slice(&[true, false, true, false, true]);
        output.add_bools_from_vec(&vec![false, false, false, true, false, true]);
        output.add_some_bools_from_slice(&[false, true, false, true, false], 1, 3);
        output.add_some_bools_from_vec(&vec![true, false, false, true], 1, 2);

        output.add_i8_slice(&[-42, 11, 127, 100, 0, -21]);
        output.add_i8_vec(&vec![36, -128, -45, 96]);
        output.add_i8s_from_slice(&[111, -111, 35, 97, -69]);
        output.add_i8s_from_vec(&vec![-1, -2, 1, 2, 72, 53]);
        output.add_some_i8s_from_slice(&[-78, 88, 19, 58, -90], 1, 3);
        output.add_some_i8s_from_vec(&vec![37, 83, 73, -65], 1, 2);
    }

    fn check_stuff_in_bit_input(input: &mut BitInput){
        assert_eq!(input.read_bools(5), vec![false, true, true, false, true]);
        assert_eq!(input.read_i8(), -125);
        assert_eq!(input.read_u8(), 234);
        assert_eq!(input.read_i16(), -21345);
        assert_eq!(input.read_u16(), 25565);
        assert_eq!(input.read_i32(), 2123456789);

        assert_eq!(input.read_bool_vec(), vec![false, false, true, false, true, true]);
        assert_eq!(input.read_bool_vec(), vec![true, true, false, false]);
        assert_eq!(input.read_bools(5), vec![true, false, true, false, true]);
        assert_eq!(input.read_bools(6), vec![false, false, false, true, false, true]);
        let mut test_bool_vec = vec![false; 3];
        input.read_bools_to_vec(&mut test_bool_vec, 0, 3);
        assert_eq!(test_bool_vec, vec![true, false, true]);
        let mut test_bool_slice = [true; 2];
        input.read_bools_to_slice(&mut test_bool_slice, 0, 2);
        assert_eq!(test_bool_slice, [false, false]);

        assert_eq!(input.read_i8_vec(), vec![-42, 11, 127, 100, 0, -21]);
        assert_eq!(input.read_i8_vec(), vec![36, -128, -45, 96]);
        assert_eq!(input.read_i8s(5), vec![111, -111, 35, 97, -69]);
        assert_eq!(input.read_i8s(6), vec![-1, -2, 1, 2, 72, 53]);
        let mut test_i8_vec = vec![-4; 3];
        input.read_i8s_to_vec(&mut test_i8_vec, 0, 3);
        assert_eq!(test_i8_vec, vec![88, 19, 58]);
        let mut test_i8_slice = [-6; 2];
        input.read_i8s_to_slice(&mut test_i8_slice, 0, 2);
        assert_eq!(test_i8_slice, [83, 73]);
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