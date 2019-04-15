const POWERS: [u64; 64] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 
524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 
2147483648, 4294967296, 8589934592, 17179869184, 34359738368, 68719476736, 137438953472, 274877906944, 549755813888, 
1099511627776, 2199023255552, 4398046511104, 8796093022208, 17592186044416, 35184372088832, 70368744177664, 140737488355328, 
281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 
36028797018963968, 72057594037927936, 144115188075855872, 288230376151711744, 576460752303423488, 1152921504606846976, 
2305843009213693952, 4611686018427387904, 9223372036854775808];

pub fn get_power_of_2(power: usize) -> u64 {
    POWERS[power]
}

fn check_bitcount(size_bits: usize){
    if size_bits >= 64 {
        panic!("You can't use more than 63 bits to store the magnitude of a signed integer, but you are using {} bits", size_bits);
    }
}

fn check_overflow(number: i64, size_bits: usize){
    if size_bits != 63 && (POWERS[size_bits] as i64 <= number || (POWERS[size_bits] as i64) < -number) {
        panic!("The magnitude of the integer {} can't be stored using only {} bits.", number, size_bits);
    }
}

/**
 * Converts a signed integer to booleans using the given number of bits/booleans. The result will be placed in
 * dest (the parameter) and the first boolean will be stored in dest[start_index]. This function can be used to
 * store integers that for instance only need 37 bits. This function will panic if the given number of booleans
 * is not enough to store the given integer.
 */
pub fn signed_int_to_bools(integer: i64, bits: usize, dest: &mut [bool], start_index: usize){
    let size_bits = bits - 1;
    check_bitcount(size_bits);
    check_overflow(integer, size_bits);

    let mut unsigned;

    if integer >= 0 {
        dest[start_index] = true;
        unsigned = integer as u64;
    } else {
        dest[start_index] = false;
        unsigned = (integer.wrapping_neg().wrapping_sub(1)) as u64;
    }

    for index in 1..=size_bits {
        if unsigned >= POWERS[size_bits - index] {
            unsigned -= POWERS[size_bits - index];
            dest[start_index + index] = true;
        } else {
            dest[start_index + index] = false;
        }
    }
}

/**
 * Converts a bool slice (back) to a signed integer. This function is made to convert the booleans stored by
 * signed_int_to_bools back to the original integer value. The bits parameter must be the same one as the
 * bits parameter supplied to signed_int_to_bools.
 * 
 * The first bit will be used to determine the sign of the number. 0 for negative and 1 for a positive
 * number. The other bits will be used to store the magnitude of the number. If the number was negative,
 * it will be substracted by 1 at the end (so if the magnitude was stored as 2, the result will be -3).
 */
pub fn bools_to_signed_int(bits: usize, bools: &[bool], start_index: usize) -> i64 {
    let size_bits = bits - 1;
    check_bitcount(size_bits);
    let mut integer: i64 = 0;

    for b in 1..=size_bits {
        if bools[start_index + b] {
            integer += POWERS[size_bits - b] as i64;
        }
    }

    if !bools[start_index] {
        integer = -integer - 1;
    }

    integer
}

/**
 * Converts 8 booleans to an i8. This can be useful for efficiently storing boolean values because they occupy
 * less memory this way. Also, this can be used to efficiently store them in a file or send them over the network.
 * The original booleans can be obtained by using i8_to_booleans or i8_to_boolean_array.
 */
pub fn bools_to_i8(b1: bool, b2: bool, b3: bool, b4: bool, b5: bool, b6: bool, b7: bool, b8: bool) -> i8 {
    let mut result: i8 = 0;
    if b1 {
         result += 64;
    }
    if b2 {
        result += 32;
    }
    if b3 {
        result += 16;
    }
    if b4 {
        result += 8;
    }
    if b5 {
        result += 4;
    }
    if b6 {
        result += 2;
    }
    if b7 {
        result += 1;
    }
    if !b8 {
        result = -result - 1;
    }
    result
}

/**
 * Converts a tuple of 8 booleans to an i8. This can be useful for efficiently storing boolean values because they 
 * occupy less memory this way. Also, this can be used to efficiently store them in a file or send them over the 
 * network. The original booleans can be obtained by using i8_to_boolean_tuple, i8_to_booleans or i8_to_boolean_array.
 */
pub fn bool_tuple_to_i8(bools: (bool,bool,bool,bool,bool,bool,bool,bool)) -> i8 {
    let mut result: i8 = 0;
    if bools.0 {
        result += 64;
    }
    if bools.1 {
        result += 32;
    }
    if bools.2 {
        result += 16;
    }
    if bools.3 {
        result += 8;
    }
    if bools.4 {
        result += 4;
    }
    if bools.5 {
        result += 2;
    }
    if bools.6 {
        result += 1;
    }
    if !bools.7 {
        result = -result - 1;
    }
    result
}

/**
 * Converts 8 booleans to an i8. This can be useful for efficiently storing boolean values because they occupy
 * less memory this way. Also, this can be used to efficiently store them in a file or send them over the network.
 * The original booleans can be obtained by using i8_to_boolean_array or i8_to_booleans.
 */
pub fn bool_array_to_i8(source: [bool; 8]) -> i8 {
    let mut result: i8 = 0;
    if source[0] {
        result += 64;
    }
    if source[1] {
        result += 32;
    }
    if source[2] {
        result += 16;
    }
    if source[3] {
        result += 8;
    }
    if source[4] {
        result += 4;
    }
    if source[5] {
        result += 2;
    }
    if source[6] {
        result += 1;
    }
    if !source[7] {
        result = -result - 1;
    }
    result
}

/**
 * Converts 8 booleans to an i8. This can be useful for efficiently storing boolean values because they occupy
 * less memory this way. Also, this can be used to efficiently store them in a file or send them over the network.
 * The original booleans can be obtained by using i8_to_boolean_array or i8_to_booleans.
 */
pub fn bool_slice_to_i8(source: &[bool]) -> i8 {
    let mut result: i8 = 0;
    if source[0] {
        result += 64;
    }
    if source[1] {
        result += 32;
    }
    if source[2] {
        result += 16;
    }
    if source[3] {
        result += 8;
    }
    if source[4] {
        result += 4;
    }
    if source[5] {
        result += 2;
    }
    if source[6] {
        result += 1;
    }
    if !source[7] {
        result = -result - 1;
    }
    result
}

/**
 * Converts an i8 to a boolean array of size 8. This function is made to convert the result of boolean_array_to_i8,
 * booleans_to_i8 or boolean_tuple_to_i8 back to the original booleans. Don't try to get the original i8 back 
 * using anything but the functions that were just mentioned. This function will map every unique i8 value to a 
 * unique 8-tuple of booleans.
 */
pub fn i8_to_bool_array(mut byte: i8) -> [bool; 8] {
    let mut booleans = [false; 8];
    if byte >= 0 {
        booleans[7] = true;
    } else {
        byte = -(byte + 1)
    }

    if byte >= 64 {
        booleans[0] = true;
        byte -= 64;
    }

    if byte >= 32 {
        booleans[1] = true;
        byte -= 32;
    }

    if byte >= 16 {
        booleans[2] = true;
        byte -= 16;
    }

    if byte >= 8 {
        booleans[3] = true;
        byte -= 8;
    }

    if byte >= 4 {
        booleans[4] = true;
        byte -= 4;
    }

    if byte >= 2 {
        booleans[5] = true;
        byte -= 2;
    }

    booleans[6] = byte == 1;

    booleans
}

/**
 * Converts an i8 to a tuple of 8 booleans. This function is made to convert the result of booleans_to_i8
 * or boolean_array_to_i8 back to the original booleans. Don't try to get the original i8 back using anything 
 * but the functions that were just mentioned. This function will map every unique i8 value to a unique 8-tuple 
 * of booleans.
 */
pub fn i8_to_bool_tuple(mut byte: i8) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let mut booleans = (false,false,false,false,false,false,false,false);
    if byte >= 0 {
        booleans.7 = true;
    } else {
        byte = -(byte + 1)
    }

    if byte >= 64 {
        booleans.0 = true;
        byte -= 64;
    }

    if byte >= 32 {
        booleans.1 = true;
        byte -= 32;
    }

    if byte >= 16 {
        booleans.2 = true;
        byte -= 16;
    }

    if byte >= 8 {
        booleans.3 = true;
        byte -= 8;
    }

    if byte >= 4 {
        booleans.4 = true;
        byte -= 4;
    }

    if byte >= 2 {
        booleans.5 = true;
        byte -= 2;
    }

    booleans.6 = byte == 1;

    booleans
}

/**
 * Converts 8 booleans to a u8. This function can be used to store multiple booleans more compactly in
 * memory or in disk. The original booleans can be obtained by using u8_to_boolean_tuple or
 * u8_to_boolean_array. Every unique 8-tuple of booleans will map to a unique u8.
 */
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

/**
 * Converts a boolean-tuple of size 8 to a u8. This function can be used to store multiple 
 * booleans more compactly in memory or in disk. The original booleans can be obtained by 
 * using u8_to_boolean_tuple or u8_to_boolean_array. Every unique 8-tuple of booleans will 
 * map to a unique u8.
 */
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

/**
 * Converts a boolean array of size 8 to a u8. This function can be used to store multiple 
 * booleans more compactly in memory or in disk. The original booleans can be obtained by 
 * using u8_to_boolean_array or u8_to_boolean_tuple. Every unique 8-tuple of booleans will 
 * map to a unique u8.
 */
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

/**
 * Converts a boolean slice of size 8 to a u8. This function can be used to store multiple 
 * booleans more compactly in memory or in disk. The original booleans can be obtained by 
 * using u8_to_boolean_array or u8_to_boolean_tuple. Every unique 8-tuple of booleans will 
 * map to a unique u8.
 */
pub fn boolean_slice_to_u8(bools: &[bool; 8]) -> u8 {
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

/**
 * Converts a u8 to a boolean-tuple of size 8. This function can be used to convert the result
 * of boolean_tuple_to_u8 back to the original u8. This function will map every unique u8 value
 * to a unique boolean tuple.
 */
pub fn u8_to_boolean_tuple(mut byte: u8) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
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

/**
 * Converts a u8 to a boolean array of size 8. This function can be used to convert the result
 * of boolean_array_to_u8 back to the original u8. This function will map every unique u8 value
 * to a unique boolean array.
 */
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

/**
 * Convert 2 i8 values to an i16 value. Every distinct pair of i8 values will be mapped 
 * to another  i16 value. This function can be used to convert the result of
 * i16_to_i8_tuple, i16_to_i8_array or i16_to_i8_1 and i16_to_i8_2 back to the original
 * i16 value.
 */
pub fn i8s_to_i16(byte1: i8, byte2: i8) -> i16 {
    ((byte2 as i16) << 8) | ((byte1 as i16) & 0xff)
}

/**
 * Convert a pair of i8 values to an i16 value. Every distinct pair of i8 values will be mapped 
 * to another  i16 value. This function can be used to convert the result of
 * i16_to_i8_tuple, i16_to_i8_array or i16_to_i8_1 and i16_to_i8_2 back to the original
 * i16 value.
 */
pub fn i8_tuple_to_i16(bytes: (i8, i8)) -> i16 {
    i8s_to_i16(bytes.0, bytes.1)
}

/**
 * Convert an array containing 2 i8 values to a i16 value. Every distinct pair of i8 
 * values will be mapped to another  i16 value. This function can be used to convert 
 * the result of i16_to_i8_array, i16_to_i8_tuple or i16_to_i8_1 and i16_to_i8_2 back 
 * the original i16 value.
 */
pub fn i8_array_to_i16(bytes: [i8; 2]) -> i16 {
    i8s_to_i16(bytes[0], bytes[1])
}

/**
 * Convert a slice containing 2 i8 values to a i16 value. Every distinct pair of i8 
 * values will be mapped to another  i16 value. This function can be used to convert 
 * the result of i16_to_i8_array, i16_to_i8_tuple or i16_to_i8_1 and i16_to_i8_2 back 
 * the original i16 value.
 */
pub fn i8_slice_to_i16(bytes: &[i8; 2]) -> i16 {
    i8s_to_i16(bytes[0], bytes[1])
}

/**
 * The first function to convert an i16 value to i8 values. This function is useless
 * without the function i16_to_i8_2, that converts the i16 value to the other
 * i8 value. These 2 functions together will map every distinct i16 value to another
 * pair of i8 values. The original i16 value can be restored with the function
 * i8s_to_i16(i16_to_i8_1(original), i16_to_i8_2(original)). Similarly, the original
 * value can be restored by using i8_tuple_to_i16 and i8_array_to_i16.
 */
pub fn i16_to_i8_1(int16: i16) -> i8 {
    int16 as i8
}

/**
 * The second function to convert an i16 value to i8 values. This function is useless
 * without the function i16_to_i8_1, that converts the i16 value to the other
 * i8 value. These 2 functions together will map every distinct i16 value to another
 * pair of i8 values. The original i16 value can be restored with the function
 * i8s_to_i16(i16_to_i8_1(original), i16_to_i8_2(original)). Similarly, the original
 * value can be restored by using i8_tuple_to_i16, i8_array_to_i16 and i8_slice_to_i16.
 */
pub fn i16_to_i8_2(int16: i16) -> i8 {
    (int16 >> 8) as i8
}

/**
 * Converts an i16 value to a pair of i8 values. Every distinct i16 value will be mapped
 * to another pair of i8 values. This function can be used to store an i16 value on disk
 * or to send it over the network. The original i16 value can be restored using
 * i8_tuple_to_i16, i8s_to_i16, i8_array_to_i16 or i8_slice_to_i16.
 */
pub fn i16_to_i8_tuple(int16: i16) -> (i8, i8) {
    (i16_to_i8_1(int16), i16_to_i8_2(int16))
}

/**
 * Converts an i16 value to an array of i8 values. Every distinct i16 value will be mapped
 * to another array of i8 values. This function can be used to store an i16 value on disk
 * or to send it over the network. The original i16 value can be restored using
 * i8_array_to_i16, i8_slice_to_i16, i8s_to_i16 or i8_tuple_to_i16.
 */
pub fn i16_to_i8_array(int16: i16) -> [i8; 2] {
    [i16_to_i8_1(int16), i16_to_i8_2(int16)]
}

/**
 * Convert 2 i8 values to an u16 value. Every distinct pair of i8 values will be mapped 
 * to another  u16 value. This function can be used to convert the result of
 * u16_to_i8_tuple, u16_to_i8_array or u16_to_i8_1 and u16_to_i8_2 back to the original
 * u16 value.
 */
pub fn i8s_to_u16(byte1: i8, byte2: i8) -> u16 {
    ((byte2 as u16) << 8) | ((byte1 as u16) & 0xff)
}

/**
 * Convert a pair of i8 values to an u16 value. Every distinct pair of i8 values will be mapped 
 * to another  u16 value. This function can be used to convert the result of
 * u16_to_i8_tuple, u16_to_i8_array or u16_to_i8_1 and u16_to_i8_2 back to the original
 * u16 value.
 */
pub fn i8_tuple_to_u16(bytes: (i8, i8)) -> u16 {
    i8s_to_u16(bytes.0, bytes.1)
}

/**
 * Convert an array containing 2 i8 values to a u16 value. Every distinct pair of i8 
 * values will be mapped to another  u16 value. This function can be used to convert 
 * the result of u16_to_i8_array, u16_to_i8_tuple or u16_to_i8_1 and u16_to_i8_2 back 
 * the original u16 value.
 */
pub fn i8_array_to_u16(bytes: [i8; 2]) -> u16 {
    i8s_to_u16(bytes[0], bytes[1])
}

/**
 * Convert a slice containing 2 i8 values to a u16 value. Every distinct pair of i8 
 * values will be mapped to another  u16 value. This function can be used to convert 
 * the result of u16_to_i8_array, u16_to_i8_tuple or u16_to_i8_1 and u16_to_i8_2 back 
 * the original u16 value.
 */
pub fn i8_slice_to_u16(bytes: &[i8; 2]) -> u16 {
    i8s_to_u16(bytes[0], bytes[1])
}

/**
 * The first function to convert an u16 value to i8 values. This function is useless
 * without the function u16_to_i8_2, that converts the u16 value to the other
 * i8 value. These 2 functions together will map every distinct u16 value to another
 * pair of i8 values. The original u16 value can be restored with the function
 * i8s_to_u16(u16_to_i8_1(original), u16_to_i8_2(original)). Similarly, the original
 * value can be restored by using i8_tuple_to_u16 and i8_array_to_u16.
 */
pub fn u16_to_i8_1(int16: u16) -> i8 {
    int16 as i8
}

/**
 * The second function to convert an u16 value to i8 values. This function is useless
 * without the function u16_to_i8_1, that converts the u16 value to the other
 * i8 value. These 2 functions together will map every distinct u16 value to another
 * pair of i8 values. The original u16 value can be restored with the function
 * i8s_to_u16(u16_to_i8_1(original), u16_to_i8_2(original)). Similarly, the original
 * value can be restored by using i8_tuple_to_u16, i8_array_to_u16 and i8_slice_to_u16.
 */
pub fn u16_to_i8_2(int16: u16) -> i8 {
    (int16 >> 8) as i8
}

/**
 * Converts an u16 value to a pair of i8 values. Every distinct u16 value will be mapped
 * to another pair of i8 values. This function can be used to store an u16 value on disk
 * or to send it over the network. The original u16 value can be restored using
 * i8_tuple_to_u16, i8s_to_u16, i8_array_to_u16 or i8_slice_to_u16.
 */
pub fn u16_to_i8_tuple(int16: u16) -> (i8, i8) {
    (u16_to_i8_1(int16), u16_to_i8_2(int16))
}

/**
 * Converts an u16 value to an array of i8 values. Every distinct u16 value will be mapped
 * to another array of i8 values. This function can be used to store an u16 value on disk
 * or to send it over the network. The original u16 value can be restored using
 * i8_array_to_u16, i8_slice_to_u16, i8s_to_u16 or i8_tuple_to_u16.
 */
pub fn u16_to_i8_array(int16: u16) -> [i8; 2] {
    [u16_to_i8_1(int16), u16_to_i8_2(int16)]
}

/**
 * Convert 4 i8 values to an i32 value. Every distinct tuple of i8 values will be mapped 
 * to another i32 value. This function can be used to convert the result of
 * i32_to_i8_tuple, i32_to_i8_array or i32_to_i8_1...4 back to the original i32 value.
 */
pub fn i8s_to_i32(byte1: i8, byte2: i8, byte3: i8, byte4: i8) -> i32 {
    ((byte4 as i32) << 24) | (((byte3 as i32) & 0xff) << 16) | (((byte2 as i32) & 0xFF) << 8) | ((byte1 as i32) & 0xFF)
}

/**
 * Convert a tuple of i8 values to an i32 value. Every distinct pair of i8 values will be mapped 
 * to another i32 value. This function can be used to convert the result of
 * i32_to_i8_tuple, i32_to_i8_array or i32_to_i8_1...4 back to the original i32 value.
 */
pub fn i8_tuple_to_i32(bytes: (i8, i8, i8, i8)) -> i32 {
    i8s_to_i32(bytes.0, bytes.1, bytes.2, bytes.3)
}

/**
 * Convert an array containing 4 i8 values to a i32 value. Every distinct tuple of i8 
 * values will be mapped to another i32 value. This function can be used to convert 
 * the result of i32_to_i8_array, i32_to_i8_tuple or i16_to_i8_1...4 back 
 * the original i32 value.
 */
pub fn i8_array_to_i32(bytes: [i8; 4]) -> i32 {
    i8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Convert a slice containing 4 i8 values to a i32 value. Every distinct pair of i8 
 * values will be mapped to another i32 value. This function can be used to convert 
 * the result of i32_to_i8_array, i32_to_i8_tuple or i32_to_i8_1...4 back 
 * the original i32 value.
 */
pub fn i8_slice_to_i32(bytes: &[i8; 4]) -> i32 {
    i8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Converts a vector containing 4 i8 values to an i32 value. Every distinct pair of i8
 * values will be mapped to another i32 value. This function can be used to convert 
 * the result of i32_to_i8_array, i32_to_i8_tuple or i32_to_i8_1...4 back 
 * the original i32 value.
 */
pub fn i8_vec_to_i32(bytes: &Vec<i8>) -> i32 {
    i8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * The first function to convert an i32 value to i8 values. This function is useless
 * without the other i32_to_i8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of i8 values. The original i32 value can be 
 * restored with the function i8s_to_i32. Similarly, the original
 * value can be restored by using i8_tuple_to_i32, i8_array_to_i32 and i8_slice_to_i32.
 */
pub fn i32_to_i8_1(int32: i32) -> i8 {
    int32 as i8
}

/**
 * The second function to convert an i32 value to i8 values. This function is useless
 * without the other i32_to_i8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of i8 values. The original i32 value can be 
 * restored with the function i8s_to_i32. Similarly, the original
 * value can be restored by using i8_tuple_to_i32, i8_array_to_i32 and i8_slice_to_i32.
 */
pub fn i32_to_i8_2(int32: i32) -> i8 {
    (int32 >> 8) as i8
}

/**
 * The third function to convert an i32 value to i8 values. This function is useless
 * without the other i32_to_i8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of i8 values. The original i32 value can be 
 * restored with the function i8s_to_i32. Similarly, the original
 * value can be restored by using i8_tuple_to_i32, i8_array_to_i32 and i8_slice_to_i32.
 */
pub fn i32_to_i8_3(int32: i32) -> i8 {
    (int32 >> 16) as i8
}

/**
 * The fourth function to convert an i32 value to i8 values. This function is useless
 * without the other i32_to_i8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of i8 values. The original i32 value can be 
 * restored with the function i8s_to_i32. Similarly, the original
 * value can be restored by using i8_tuple_to_i32, i8_array_to_i32 and i8_slice_to_i32.
 */
pub fn i32_to_i8_4(int32: i32) -> i8 {
    (int32 >> 24) as i8
}

/**
 * Converts an i32 value to a tuple of i8 values. Every distinct i32 value will be mapped
 * to another pair of i8 values. This function can be used to store an i32 value on disk
 * or to send it over the network. The original i32 value can be restored using
 * i8_tuple_to_i32, i8s_to_i32, i8_array_to_i32 or i8_slice_to_i32.
 */
pub fn i32_to_i8_tuple(int32: i32) -> (i8, i8, i8, i8) {
    (i32_to_i8_1(int32), i32_to_i8_2(int32), i32_to_i8_3(int32), i32_to_i8_4(int32))
}

/**
 * Converts an i32 value to an array of i8 values. Every distinct i32 value will be mapped
 * to another array of i8 values. This function can be used to store an i32 value on disk
 * or to send it over the network. The original i32 value can be restored using
 * i8_array_to_i32, i8_slice_to_i32, i8s_to_i32 or i8_tuple_to_i32.
 */
pub fn i32_to_i8_array(int32: i32) -> [i8; 4] {
    [i32_to_i8_1(int32), i32_to_i8_2(int32), i32_to_i8_3(int32), i32_to_i8_4(int32)]
}


/**
 * Convert 4 i8 values to an u32 value. Every distinct tuple of i8 values will be mapped 
 * to another u32 value. This function can be used to convert the result of
 * u32_to_i8_tuple, u32_to_i8_array or u32_to_i8_1...4 back to the original u32 value.
 */
pub fn i8s_to_u32(byte1: i8, byte2: i8, byte3: i8, byte4: i8) -> u32 {
    ((byte4 as u32) << 24) | (((byte3 as u32) & 0xff) << 16) | (((byte2 as u32) & 0xFF) << 8) | ((byte1 as u32) & 0xFF)
}

/**
 * Convert a tuple of i8 values to an u32 value. Every distinct pair of i8 values will be mapped 
 * to another u32 value. This function can be used to convert the result of
 * u32_to_i8_tuple, u32_to_i8_array or u32_to_i8_1...4 back to the original u32 value.
 */
pub fn i8_tuple_to_u32(bytes: (i8, i8, i8, i8)) -> u32 {
    i8s_to_u32(bytes.0, bytes.1, bytes.2, bytes.3)
}

/**
 * Convert an array containing 4 i8 values to a u32 value. Every distinct tuple of i8 
 * values will be mapped to another u32 value. This function can be used to convert 
 * the result of u32_to_i8_array, u32_to_i8_tuple or i16_to_i8_1...4 back 
 * the original u32 value.
 */
pub fn i8_array_to_u32(bytes: [i8; 4]) -> u32 {
    i8s_to_u32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Convert a slice containing 4 i8 values to a u32 value. Every distinct pair of i8 
 * values will be mapped to another u32 value. This function can be used to convert 
 * the result of u32_to_i8_array, u32_to_i8_tuple or u32_to_i8_1...4 back 
 * the original u32 value.
 */
pub fn i8_slice_to_u32(bytes: &[i8; 4]) -> u32 {
    i8s_to_u32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Converts a vector containing 4 i8 values to an u32 value. Every distinct pair of i8
 * values will be mapped to another u32 value. This function can be used to convert 
 * the result of u32_to_i8_array, u32_to_i8_tuple or u32_to_i8_1...4 back 
 * the original u32 value.
 */
pub fn i8_vec_to_u32(bytes: &Vec<i8>) -> u32 {
    i8s_to_u32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * The first function to convert an u32 value to i8 values. This function is useless
 * without the other u32_to_i8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of i8 values. The original u32 value can be 
 * restored with the function i8s_to_u32. Similarly, the original
 * value can be restored by using i8_tuple_to_u32, i8_array_to_u32 and i8_slice_to_u32.
 */
pub fn u32_to_i8_1(int32: u32) -> i8 {
    int32 as i8
}

/**
 * The second function to convert an u32 value to i8 values. This function is useless
 * without the other u32_to_i8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of i8 values. The original u32 value can be 
 * restored with the function i8s_to_u32. Similarly, the original
 * value can be restored by using i8_tuple_to_u32, i8_array_to_u32 and i8_slice_to_u32.
 */
pub fn u32_to_i8_2(int32: u32) -> i8 {
    (int32 >> 8) as i8
}

/**
 * The third function to convert an u32 value to i8 values. This function is useless
 * without the other u32_to_i8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of i8 values. The original u32 value can be 
 * restored with the function i8s_to_u32. Similarly, the original
 * value can be restored by using i8_tuple_to_u32, i8_array_to_u32 and i8_slice_to_u32.
 */
pub fn u32_to_i8_3(int32: u32) -> i8 {
    (int32 >> 16) as i8
}

/**
 * The fourth function to convert an u32 value to i8 values. This function is useless
 * without the other u32_to_i8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of i8 values. The original u32 value can be 
 * restored with the function i8s_to_u32. Similarly, the original
 * value can be restored by using i8_tuple_to_u32, i8_array_to_u32 and i8_slice_to_u32.
 */
pub fn u32_to_i8_4(int32: u32) -> i8 {
    (int32 >> 24) as i8
}

/**
 * Converts an u32 value to a tuple of i8 values. Every distinct u32 value will be mapped
 * to another pair of i8 values. This function can be used to store an u32 value on disk
 * or to send it over the network. The original u32 value can be restored using
 * i8_tuple_to_u32, i8s_to_u32, i8_array_to_u32 or i8_slice_to_u32.
 */
pub fn u32_to_i8_tuple(int32: u32) -> (i8, i8, i8, i8) {
    (u32_to_i8_1(int32), u32_to_i8_2(int32), u32_to_i8_3(int32), u32_to_i8_4(int32))
}

/**
 * Converts an u32 value to an array of i8 values. Every distinct u32 value will be mapped
 * to another array of i8 values. This function can be used to store an u32 value on disk
 * or to send it over the network. The original u32 value can be restored using
 * i8_array_to_u32, i8_slice_to_u32, i8s_to_u32 or i8_tuple_to_u32.
 */
pub fn u32_to_i8_array(int32: u32) -> [i8; 4] {
    [u32_to_i8_1(int32), u32_to_i8_2(int32), u32_to_i8_3(int32), u32_to_i8_4(int32)]
}


/**
 * Convert 2 u8 values to an i16 value. Every distinct pair of u8 values will be mapped 
 * to another i16 value. This function can be used to convert the result of
 * i16_to_u8_tuple, i16_to_u8_array or i16_to_u8_1 and i16_to_u8_2 back to the original
 * i16 value.
 */
pub fn u8s_to_i16(byte1: u8, byte2: u8) -> i16 {
    ((byte2 as i16) << 8) | ((byte1 as i16) & 0xff)
}

/**
 * Convert a pair of u8 values to an i16 value. Every distinct pair of u8 values will be mapped 
 * to another i16 value. This function can be used to convert the result of
 * i16_to_u8_tuple, i16_to_u8_array or i16_to_u8_1 and i16_to_u8_2 back to the original
 * i16 value.
 */
pub fn u8_tuple_to_i16(bytes: (u8, u8)) -> i16 {
    u8s_to_i16(bytes.0, bytes.1)
}

/**
 * Convert an array containing 2 u8 values to a i16 value. Every distinct pair of u8 
 * values will be mapped to another i16 value. This function can be used to convert 
 * the result of i16_to_u8_array, i16_to_u8_tuple or i16_to_u8_1 and i16_to_u8_2 back 
 * the original i16 value.
 */
pub fn u8_array_to_i16(bytes: [u8; 2]) -> i16 {
    u8s_to_i16(bytes[0], bytes[1])
}

/**
 * Convert a slice containing 2 u8 values to a i16 value. Every distinct pair of u8 
 * values will be mapped to another i16 value. This function can be used to convert 
 * the result of i16_to_u8_array, i16_to_u8_tuple or i16_to_u8_1 and i16_to_u8_2 back 
 * the original i16 value.
 */
pub fn u8_slice_to_i16(bytes: &[u8; 2]) -> i16 {
    u8s_to_i16(bytes[0], bytes[1])
}

/**
 * The first function to convert an i16 value to u8 values. This function is useless
 * without the function i16_to_u8_2, that converts the i16 value to the other
 * u8 value. These 2 functions together will map every distinct i16 value to another
 * pair of u8 values. The original i16 value can be restored with the function
 * u8s_to_i16(i16_to_u8_1(original), i16_to_u8_2(original)). Similarly, the original
 * value can be restored by using u8_tuple_to_i16 and u8_array_to_i16.
 */
pub fn i16_to_u8_1(int16: i16) -> u8 {
    int16 as u8
}

/**
 * The second function to convert an i16 value to u8 values. This function is useless
 * without the function i16_to_u8_1, that converts the i16 value to the other
 * u8 value. These 2 functions together will map every distinct i16 value to another
 * pair of u8 values. The original i16 value can be restored with the function
 * u8s_to_i16(i16_to_u8_1(original), i16_to_u8_2(original)). Similarly, the original
 * value can be restored by using u8_tuple_to_i16, u8_array_to_i16 and u8_slice_to_i16.
 */
pub fn i16_to_u8_2(int16: i16) -> u8 {
    (int16 >> 8) as u8
}

/**
 * Converts an i16 value to a pair of u8 values. Every distinct i16 value will be mapped
 * to another pair of u8 values. This function can be used to store an i16 value on disk
 * or to send it over the network. The original i16 value can be restored using
 * u8_tuple_to_i16, u8s_to_i16, u8_array_to_i16 or u8_slice_to_i16.
 */
pub fn i16_to_u8_tuple(int16: i16) -> (u8, u8) {
    (i16_to_u8_1(int16), i16_to_u8_2(int16))
}

/**
 * Converts an i16 value to an array of u8 values. Every distinct i16 value will be mapped
 * to another array of u8 values. This function can be used to store an i16 value on disk
 * or to send it over the network. The original i16 value can be restored using
 * u8_array_to_i16, u8_slice_to_i16, u8s_to_i16 or u8_tuple_to_i16.
 */
pub fn i16_to_u8_array(int16: i16) -> [u8; 2] {
    [i16_to_u8_1(int16), i16_to_u8_2(int16)]
}

/**
 * Convert 2 u8 values to an u16 value. Every distinct pair of u8 values will be mapped 
 * to another  u16 value. This function can be used to convert the result of
 * u16_to_u8_tuple, u16_to_u8_array or u16_to_u8_1 and u16_to_u8_2 back to the original
 * u16 value.
 */
pub fn u8s_to_u16(byte1: u8, byte2: u8) -> u16 {
    ((byte2 as u16) << 8) | ((byte1 as u16) & 0xff)
}

/**
 * Convert a pair of u8 values to an u16 value. Every distinct pair of u8 values will be mapped 
 * to another  u16 value. This function can be used to convert the result of
 * u16_to_u8_tuple, u16_to_u8_array or u16_to_u8_1 and u16_to_u8_2 back to the original
 * u16 value.
 */
pub fn u8_tuple_to_u16(bytes: (u8, u8)) -> u16 {
    u8s_to_u16(bytes.0, bytes.1)
}

/**
 * Convert an array containing 2 u8 values to a u16 value. Every distinct pair of u8 
 * values will be mapped to another  u16 value. This function can be used to convert 
 * the result of u16_to_u8_array, u16_to_u8_tuple or u16_to_u8_1 and u16_to_u8_2 back 
 * the original u16 value.
 */
pub fn u8_array_to_u16(bytes: [u8; 2]) -> u16 {
    u8s_to_u16(bytes[0], bytes[1])
}

/**
 * Convert a slice containing 2 u8 values to a u16 value. Every distinct pair of u8 
 * values will be mapped to another  u16 value. This function can be used to convert 
 * the result of u16_to_u8_array, u16_to_u8_tuple or u16_to_u8_1 and u16_to_u8_2 back 
 * the original u16 value.
 */
pub fn u8_slice_to_u16(bytes: &[u8; 2]) -> u16 {
    u8s_to_u16(bytes[0], bytes[1])
}

/**
 * The first function to convert an u16 value to u8 values. This function is useless
 * without the function u16_to_u8_2, that converts the u16 value to the other
 * u8 value. These 2 functions together will map every distinct u16 value to another
 * pair of u8 values. The original u16 value can be restored with the function
 * u8s_to_u16(u16_to_u8_1(original), u16_to_u8_2(original)). Similarly, the original
 * value can be restored by using u8_tuple_to_u16 and u8_array_to_u16.
 */
pub fn u16_to_u8_1(int16: u16) -> u8 {
    int16 as u8
}

/**
 * The second function to convert an u16 value to u8 values. This function is useless
 * without the function u16_to_u8_1, that converts the u16 value to the other
 * u8 value. These 2 functions together will map every distinct u16 value to another
 * pair of u8 values. The original u16 value can be restored with the function
 * u8s_to_u16(u16_to_u8_1(original), u16_to_u8_2(original)). Similarly, the original
 * value can be restored by using u8_tuple_to_u16, u8_array_to_u16 and u8_slice_to_u16.
 */
pub fn u16_to_u8_2(int16: u16) -> u8 {
    (int16 >> 8) as u8
}

/**
 * Converts an u16 value to a pair of u8 values. Every distinct u16 value will be mapped
 * to another pair of u8 values. This function can be used to store an u16 value on disk
 * or to send it over the network. The original u16 value can be restored using
 * u8_tuple_to_u16, u8s_to_u16, u8_array_to_u16 or u8_slice_to_u16.
 */
pub fn u16_to_u8_tuple(int16: u16) -> (u8, u8) {
    (u16_to_u8_1(int16), u16_to_u8_2(int16))
}

/**
 * Converts an u16 value to an array of u8 values. Every distinct u16 value will be mapped
 * to another array of u8 values. This function can be used to store an u16 value on disk
 * or to send it over the network. The original u16 value can be restored using
 * u8_array_to_u16, u8_slice_to_u16, u8s_to_u16 or u8_tuple_to_u16.
 */
pub fn u16_to_u8_array(int16: u16) -> [u8; 2] {
    [u16_to_u8_1(int16), u16_to_u8_2(int16)]
}

/**
 * Convert 4 u8 values to an i32 value. Every distinct tuple of u8 values will be mapped 
 * to another i32 value. This function can be used to convert the result of
 * i32_to_u8_tuple, i32_to_u8_array or i32_to_u8_1...4 back to the original i32 value.
 */
pub fn u8s_to_i32(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> i32 {
    ((byte4 as i32) << 24) | (((byte3 as i32) & 0xff) << 16) | (((byte2 as i32) & 0xFF) << 8) | ((byte1 as i32) & 0xFF)
}

/**
 * Convert a tuple of u8 values to an i32 value. Every distinct pair of u8 values will be mapped 
 * to another i32 value. This function can be used to convert the result of
 * i32_to_u8_tuple, i32_to_u8_array or i32_to_u8_1...4 back to the original i32 value.
 */
pub fn u8_tuple_to_i32(bytes: (u8, u8, u8, u8)) -> i32 {
    u8s_to_i32(bytes.0, bytes.1, bytes.2, bytes.3)
}

/**
 * Convert an array containing 4 u8 values to a i32 value. Every distinct tuple of u8 
 * values will be mapped to another i32 value. This function can be used to convert 
 * the result of i32_to_u8_array, i32_to_u8_tuple or i16_to_u8_1...4 back 
 * the original i32 value.
 */
pub fn u8_array_to_i32(bytes: [u8; 4]) -> i32 {
    u8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Convert a slice containing 4 u8 values to a i32 value. Every distinct pair of u8 
 * values will be mapped to another i32 value. This function can be used to convert 
 * the result of i32_to_u8_array, i32_to_u8_tuple or i32_to_u8_1...4 back 
 * the original i32 value.
 */
pub fn u8_slice_to_i32(bytes: &[u8; 4]) -> i32 {
    u8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Converts a vector containing 4 u8 values to an i32 value. Every distinct pair of u8
 * values will be mapped to another i32 value. This function can be used to convert 
 * the result of i32_to_u8_array, i32_to_u8_tuple or i32_to_u8_1...4 back 
 * the original i32 value.
 */
pub fn u8_vec_to_i32(bytes: &Vec<u8>) -> i32 {
    u8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * The first function to convert an i32 value to u8 values. This function is useless
 * without the other i32_to_u8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of u8 values. The original i32 value can be 
 * restored with the function u8s_to_i32. Similarly, the original
 * value can be restored by using u8_tuple_to_i32, u8_array_to_i32 and u8_slice_to_i32.
 */
pub fn i32_to_u8_1(int32: i32) -> u8 {
    int32 as u8
}

/**
 * The second function to convert an i32 value to u8 values. This function is useless
 * without the other i32_to_u8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of u8 values. The original i32 value can be 
 * restored with the function u8s_to_i32. Similarly, the original
 * value can be restored by using u8_tuple_to_i32, u8_array_to_i32 and u8_slice_to_i32.
 */
pub fn i32_to_u8_2(int32: i32) -> u8 {
    (int32 >> 8) as u8
}

/**
 * The third function to convert an i32 value to u8 values. This function is useless
 * without the other i32_to_u8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of u8 values. The original i32 value can be 
 * restored with the function u8s_to_i32. Similarly, the original
 * value can be restored by using u8_tuple_to_i32, u8_array_to_i32 and u8_slice_to_i32.
 */
pub fn i32_to_u8_3(int32: i32) -> u8 {
    (int32 >> 16) as u8
}

/**
 * The fourth function to convert an i32 value to u8 values. This function is useless
 * without the other i32_to_u8_ functions. These 4 functions together will map every 
 * distinct i32 value to another tuple of u8 values. The original i32 value can be 
 * restored with the function u8s_to_i32. Similarly, the original
 * value can be restored by using u8_tuple_to_i32, u8_array_to_i32 and u8_slice_to_i32.
 */
pub fn i32_to_u8_4(int32: i32) -> u8 {
    (int32 >> 24) as u8
}

/**
 * Converts an i32 value to a tuple of u8 values. Every distinct i32 value will be mapped
 * to another pair of u8 values. This function can be used to store an i32 value on disk
 * or to send it over the network. The original i32 value can be restored using
 * u8_tuple_to_i32, u8s_to_i32, u8_array_to_i32 or u8_slice_to_i32.
 */
pub fn i32_to_u8_tuple(int32: i32) -> (u8, u8, u8, u8) {
    (i32_to_u8_1(int32), i32_to_u8_2(int32), i32_to_u8_3(int32), i32_to_u8_4(int32))
}

/**
 * Converts an i32 value to an array of u8 values. Every distinct i32 value will be mapped
 * to another array of u8 values. This function can be used to store an i32 value on disk
 * or to send it over the network. The original i32 value can be restored using
 * u8_array_to_i32, u8_slice_to_i32, u8s_to_i32 or u8_tuple_to_i32.
 */
pub fn i32_to_u8_array(int32: i32) -> [u8; 4] {
    [i32_to_u8_1(int32), i32_to_u8_2(int32), i32_to_u8_3(int32), i32_to_u8_4(int32)]
}


/**
 * Convert 4 u8 values to an u32 value. Every distinct tuple of u8 values will be mapped 
 * to another u32 value. This function can be used to convert the result of
 * u32_to_u8_tuple, u32_to_u8_array or u32_to_u8_1...4 back to the original u32 value.
 */
pub fn u8s_to_u32(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> u32 {
    ((byte4 as u32) << 24) | (((byte3 as u32) & 0xff) << 16) | (((byte2 as u32) & 0xFF) << 8) | ((byte1 as u32) & 0xFF)
}

/**
 * Convert a tuple of u8 values to an u32 value. Every distinct pair of u8 values will be mapped 
 * to another u32 value. This function can be used to convert the result of
 * u32_to_u8_tuple, u32_to_u8_array or u32_to_u8_1...4 back to the original u32 value.
 */
pub fn u8_tuple_to_u32(bytes: (u8, u8, u8, u8)) -> u32 {
    u8s_to_u32(bytes.0, bytes.1, bytes.2, bytes.3)
}

/**
 * Convert an array containing 4 u8 values to a u32 value. Every distinct tuple of u8 
 * values will be mapped to another u32 value. This function can be used to convert 
 * the result of u32_to_u8_array, u32_to_u8_tuple or i16_to_u8_1...4 back 
 * the original u32 value.
 */
pub fn u8_array_to_u32(bytes: [u8; 4]) -> u32 {
    u8s_to_u32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Convert a slice containing 4 u8 values to a u32 value. Every distinct pair of u8 
 * values will be mapped to another u32 value. This function can be used to convert 
 * the result of u32_to_u8_array, u32_to_u8_tuple or u32_to_u8_1...4 back 
 * the original u32 value.
 */
pub fn u8_slice_to_u32(bytes: &[u8; 4]) -> u32 {
    u8s_to_u32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Converts a vector containing 4 u8 values to an u32 value. Every distinct pair of u8
 * values will be mapped to another u32 value. This function can be used to convert 
 * the result of u32_to_u8_array, u32_to_u8_tuple or u32_to_u8_1...4 back 
 * the original u32 value.
 */
pub fn u8_vec_to_u32(bytes: &Vec<u8>) -> u32 {
    u8s_to_u32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * The first function to convert an u32 value to u8 values. This function is useless
 * without the other u32_to_u8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of u8 values. The original u32 value can be 
 * restored with the function u8s_to_u32. Similarly, the original
 * value can be restored by using u8_tuple_to_u32, u8_array_to_u32 and u8_slice_to_u32.
 */
pub fn u32_to_u8_1(int32: u32) -> u8 {
    int32 as u8
}

/**
 * The second function to convert an u32 value to u8 values. This function is useless
 * without the other u32_to_u8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of u8 values. The original u32 value can be 
 * restored with the function u8s_to_u32. Similarly, the original
 * value can be restored by using u8_tuple_to_u32, u8_array_to_u32 and u8_slice_to_u32.
 */
pub fn u32_to_u8_2(int32: u32) -> u8 {
    (int32 >> 8) as u8
}

/**
 * The third function to convert an u32 value to u8 values. This function is useless
 * without the other u32_to_u8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of u8 values. The original u32 value can be 
 * restored with the function u8s_to_u32. Similarly, the original
 * value can be restored by using u8_tuple_to_u32, u8_array_to_u32 and u8_slice_to_u32.
 */
pub fn u32_to_u8_3(int32: u32) -> u8 {
    (int32 >> 16) as u8
}

/**
 * The fourth function to convert an u32 value to u8 values. This function is useless
 * without the other u32_to_u8_ functions. These 4 functions together will map every 
 * distinct u32 value to another tuple of u8 values. The original u32 value can be 
 * restored with the function u8s_to_u32. Similarly, the original
 * value can be restored by using u8_tuple_to_u32, u8_array_to_u32 and u8_slice_to_u32.
 */
pub fn u32_to_u8_4(int32: u32) -> u8 {
    (int32 >> 24) as u8
}

/**
 * Converts an u32 value to a tuple of u8 values. Every distinct u32 value will be mapped
 * to another pair of u8 values. This function can be used to store an u32 value on disk
 * or to send it over the network. The original u32 value can be restored using
 * u8_tuple_to_u32, u8s_to_u32, u8_array_to_u32 or u8_slice_to_u32.
 */
pub fn u32_to_u8_tuple(int32: u32) -> (u8, u8, u8, u8) {
    (u32_to_u8_1(int32), u32_to_u8_2(int32), u32_to_u8_3(int32), u32_to_u8_4(int32))
}

/**
 * Converts an u32 value to an array of u8 values. Every distinct u32 value will be mapped
 * to another array of u8 values. This function can be used to store an u32 value on disk
 * or to send it over the network. The original u32 value can be restored using
 * u8_array_to_u32, u8_slice_to_u32, u8s_to_u32 or u8_tuple_to_u32.
 */
pub fn u32_to_u8_array(int32: u32) -> [u8; 4] {
    [u32_to_u8_1(int32), u32_to_u8_2(int32), u32_to_u8_3(int32), u32_to_u8_4(int32)]
}