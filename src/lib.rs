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

        output.add_string(Some(String::from("𝄞music")));
        output.add_string(None);

        output.add_i8_slice(&[-42, 11, 127, 100, 0, -21]);
        output.add_i8_vec(&vec![36, -128, -45, 96]);
        output.add_i8s_from_slice(&[111, -111, 35, 97, -69]);
        output.add_i8s_from_vec(&vec![-1, -2, 1, 2, 72, 53]);
        output.add_some_i8s_from_slice(&[-78, 88, 19, 58, -90], 1, 3);
        output.add_some_i8s_from_vec(&vec![37, 83, 73, -65], 1, 2);

        output.add_i16_slice(&[3483, -3498, 31834, -32745, 31834, 9834, -3456]);
        output.add_i16_vec(&vec![8374, -32756, 31234, -845, 0, 2324]);
        output.add_i16s_from_slice(&[3487, 8252, -9424, -12345, 8745]);
        output.add_i16s_from_vec(&vec![23742, -4573, 12, 8457, -31245]);
        output.add_some_i16s_from_slice(&[5328, -31587, 2374, 742, -455], 1, 3);
        output.add_some_i16s_from_vec(&vec![-4567, 7651, 324, -7325], 0, 2);

        output.add_i32_slice(&[9453948, 837247, -2378347, 18342, -347]);
        output.add_i32_vec(&vec![-4739, 347129, 179348, -8457834]);
        output.add_i32s_from_slice(&[7467, -34974857, 237834834, -6823, 101]);
        output.add_i32s_from_vec(&vec![64354, -735192, 9472, 43472823]);
        output.add_some_i32s_from_slice(&[1000, -274583634, 86374573, 9234671, 5132343, 1000], 1, 4);
        output.add_some_i32s_from_vec(&vec![2000, 2000, 85736372, -1763487, 2000], 2, 2);

        output.add_sized_i64(-15, 5);
        output.add_sized_i64(5000000000, 34);
        output.add_sized_i64(i64::min_value(), 64);
        output.add_sized_i64(i64::max_value(), 64);

        output.add_sized_u64(127, 7);
        output.add_sized_u64(0, 0);
        output.add_sized_u64(u64::max_value(), 64);

        output.add_u8_slice(&[42, 11, 127, 100, 0, 21]);
        output.add_u8_vec(&vec![36, 128, 45, 96]);
        output.add_u8s_from_slice(&[111, 111, 35, 97, 69]);
        output.add_u8s_from_vec(&vec![1, 2, 1, 2, 72, 53]);
        output.add_some_u8s_from_slice(&[78, 88, 19, 58, 90], 1, 3);
        output.add_some_u8s_from_vec(&vec![37, 83, 73, 65], 1, 2);

        output.add_u16_slice(&[3483, 3498, 31834, 32745, 31834, 9834, 3456]);
        output.add_u16_vec(&vec![8374, 32756, 31234, 845, 0, 2324]);
        output.add_u16s_from_slice(&[3487, 8252, 9424, 12345, 8745]);
        output.add_u16s_from_vec(&vec![23742, 4573, 12, 8457, 31245]);
        output.add_some_u16s_from_slice(&[5328, 31587, 2374, 742, 455], 1, 3);
        output.add_some_u16s_from_vec(&vec![4567, 7651, 324, 7325], 0, 2);

        output.add_u32_slice(&[9453948, 837247, 2378347, 18342, 347]);
        output.add_u32_vec(&vec![4739, 347129, 179348, 8457834]);
        output.add_u32s_from_slice(&[7467, 34974857, 237834834, 6823, 101]);
        output.add_u32s_from_vec(&vec![64354, 735192, 9472, 43472823]);
        output.add_some_u32s_from_slice(&[1000, 274583634, 86374573, 9234671, 5132343, 1000], 1, 4);
        output.add_some_u32s_from_vec(&vec![2000, 2000, 85736372, 1763487, 2000], 2, 2);
    }

    fn check_stuff_in_bit_input(input: &mut BitInput){
        assert_eq!(input.read_bools(5).unwrap(), vec![false, true, true, false, true]);
        assert_eq!(input.read_i8().unwrap(), -125);
        assert_eq!(input.read_u8().unwrap(), 234);
        assert_eq!(input.read_i16().unwrap(), -21345);
        assert_eq!(input.read_u16().unwrap(), 25565);
        assert_eq!(input.read_i32().unwrap(), 2123456789);

        assert_eq!(input.read_bool_vec().unwrap(), vec![false, false, true, false, true, true]);
        assert_eq!(input.read_bool_vec().unwrap(), vec![true, true, false, false]);
        assert_eq!(input.read_bools(5).unwrap(), vec![true, false, true, false, true]);
        assert_eq!(input.read_bools(6).unwrap(), vec![false, false, false, true, false, true]);
        let mut test_bool_vec = vec![false; 3];
        input.read_bools_to_vec(&mut test_bool_vec, 0, 3).unwrap();
        assert_eq!(test_bool_vec, vec![true, false, true]);
        let mut test_bool_slice = [true; 2];
        input.read_bools_to_slice(&mut test_bool_slice, 0, 2).unwrap();
        assert_eq!(test_bool_slice, [false, false]);

        assert_eq!(input.read_string(7), Ok(Some(String::from("𝄞music"))));
        assert_eq!(input.read_string(0), Ok(None));

        assert_eq!(input.read_i8_vec().unwrap(), vec![-42, 11, 127, 100, 0, -21]);
        assert_eq!(input.read_i8_vec().unwrap(), vec![36, -128, -45, 96]);
        assert_eq!(input.read_i8s(5).unwrap(), vec![111, -111, 35, 97, -69]);
        assert_eq!(input.read_i8s(6).unwrap(), vec![-1, -2, 1, 2, 72, 53]);
        let mut test_i8_vec = vec![-4; 3];
        input.read_i8s_to_vec(&mut test_i8_vec, 0, 3).unwrap();
        assert_eq!(test_i8_vec, vec![88, 19, 58]);
        let mut test_i8_slice = [-6; 2];
        input.read_i8s_to_slice(&mut test_i8_slice, 0, 2).unwrap();
        assert_eq!(test_i8_slice, [83, 73]);

        assert_eq!(input.read_i16_vec().unwrap(), vec![3483, -3498, 31834, -32745, 31834, 9834, -3456]);
        assert_eq!(input.read_i16_vec().unwrap(), vec![8374, -32756, 31234, -845, 0, 2324]);
        assert_eq!(input.read_i16s(5).unwrap(), vec![3487, 8252, -9424, -12345, 8745]);
        assert_eq!(input.read_i16s(5).unwrap(), vec![23742, -4573, 12, 8457, -31245]);
        let mut test_i16_vec = vec![1; 10];
        input.read_i16s_to_vec(&mut test_i16_vec, 2, 3).unwrap();
        assert_eq!(test_i16_vec, vec![1, 1, -31587, 2374, 742, 1, 1, 1, 1, 1]);
        let mut test_i16_array = [1; 5];
        input.read_i16s_to_slice(&mut test_i16_array, 1, 2).unwrap();
        assert_eq!(test_i16_array, [1, -4567, 7651, 1, 1]);

        assert_eq!(input.read_i32_vec().unwrap(), vec![9453948, 837247, -2378347, 18342, -347]);
        assert_eq!(input.read_i32_vec().unwrap(), vec![-4739, 347129, 179348, -8457834]);
        assert_eq!(input.read_i32s(5).unwrap(), vec![7467, -34974857, 237834834, -6823, 101]);
        assert_eq!(input.read_i32s(4).unwrap(), vec![64354, -735192, 9472, 43472823]);
        let mut test_i32_vec = vec![1; 8];
        input.read_i32s_to_vec(&mut test_i32_vec, 1, 4).unwrap();
        assert_eq!(test_i32_vec, vec![1, -274583634, 86374573, 9234671, 5132343, 1, 1, 1]);
        let mut test_i32_array = [2; 8];
        input.read_i32s_to_slice(&mut test_i32_array, 3, 2).unwrap();
        assert_eq!(test_i32_array, [2, 2, 2, 85736372, -1763487, 2, 2, 2]);

        assert_eq!(input.read_sized_i64(5).unwrap(), -15);
        assert_eq!(input.read_sized_i64(34).unwrap(), 5000000000);
        assert_eq!(input.read_sized_i64(64).unwrap(), i64::min_value());
        assert_eq!(input.read_sized_i64(64).unwrap(), i64::max_value());

        assert_eq!(input.read_sized_u64(7).unwrap(), 127);
        assert_eq!(input.read_sized_u64(0).unwrap(), 0);
        assert_eq!(input.read_sized_u64(64).unwrap(), u64::max_value());

        assert_eq!(input.read_u8_vec().unwrap(), vec![42, 11, 127, 100, 0, 21]);
        assert_eq!(input.read_u8_vec().unwrap(), vec![36, 128, 45, 96]);
        assert_eq!(input.read_u8s(5).unwrap(), vec![111, 111, 35, 97, 69]);
        assert_eq!(input.read_u8s(6).unwrap(), vec![1, 2, 1, 2, 72, 53]);
        let mut test_u8_vec = vec![4; 3];
        input.read_u8s_to_vec(&mut test_u8_vec, 0, 3).unwrap();
        assert_eq!(test_u8_vec, vec![88, 19, 58]);
        let mut test_u8_slice = [6; 2];
        input.read_u8s_to_slice(&mut test_u8_slice, 0, 2).unwrap();
        assert_eq!(test_u8_slice, [83, 73]);

        assert_eq!(input.read_u16_vec().unwrap(), vec![3483, 3498, 31834, 32745, 31834, 9834, 3456]);
        assert_eq!(input.read_u16_vec().unwrap(), vec![8374, 32756, 31234, 845, 0, 2324]);
        assert_eq!(input.read_u16s(5).unwrap(), vec![3487, 8252, 9424, 12345, 8745]);
        assert_eq!(input.read_u16s(5).unwrap(), vec![23742, 4573, 12, 8457, 31245]);
        let mut test_u16_vec = vec![1; 10];
        input.read_u16s_to_vec(&mut test_u16_vec, 2, 3).unwrap();
        assert_eq!(test_u16_vec, vec![1, 1, 31587, 2374, 742, 1, 1, 1, 1, 1]);
        let mut test_u16_array = [1; 5];
        input.read_u16s_to_slice(&mut test_u16_array, 1, 2).unwrap();
        assert_eq!(test_u16_array, [1, 4567, 7651, 1, 1]);

        assert_eq!(input.read_u32_vec().unwrap(), vec![9453948, 837247, 2378347, 18342, 347]);
        assert_eq!(input.read_u32_vec().unwrap(), vec![4739, 347129, 179348, 8457834]);
        assert_eq!(input.read_u32s(5).unwrap(), vec![7467, 34974857, 237834834, 6823, 101]);
        assert_eq!(input.read_u32s(4).unwrap(), vec![64354, 735192, 9472, 43472823]);
        let mut test_u32_vec = vec![1; 8];
        input.read_u32s_to_vec(&mut test_u32_vec, 1, 4).unwrap();
        assert_eq!(test_u32_vec, vec![1, 274583634, 86374573, 9234671, 5132343, 1, 1, 1]);
        let mut test_u32_array = [2; 8];
        input.read_u32s_to_slice(&mut test_u32_array, 3, 2).unwrap();
        assert_eq!(test_u32_array, [2, 2, 2, 85736372, 1763487, 2, 2, 2]);

        let maybe_capacity_error = input.read_i16_vec();
        let capacity_error = maybe_capacity_error.unwrap_err();
        match capacity_error {
            BitInputError::StringLength(_) => panic!("Should have been capacity error"),
            BitInputError::InputCapacity(c) => assert_eq!(c.requested_extra_capacity(), 32),
            BitInputError::InvalidString(_) => panic!("Should have been capacity error")
        };
    }

    #[test]
    fn test_sized_i64_to_bools(){
        let mut integer = -9223372036854775808;
        while integer < 23738474347634 {
            test_single_sized_i64(integer);
            integer += 9538274823127357;
        }
        test_single_sized_i64(0);
    }

    fn test_single_sized_i64(integer: i64){
        let mut as_bools = [false; 64];
        sized_i64_to_bools(integer, 64, &mut as_bools, 0);
        let reverted = bools_to_sized_i64(64, &as_bools, 0);
        assert_eq!(integer, reverted);
    }

    #[test]
    fn test_sized_u64_to_bools(){
        let mut integer = u64::max_value();
        let step_size = 3487384783472473;
        while integer > step_size {
            test_single_sized_u64(integer);
            integer -= step_size;
        }
    }

    fn test_single_sized_u64(integer: u64){
        let mut as_bools = [false; 64];
        sized_u64_to_bools(integer, 64, &mut as_bools, 0);
        let reverted = bools_to_sized_u64(64, &as_bools, 0);
        assert_eq!(integer, reverted);
    }
}