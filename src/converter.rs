const POWERS: [u64; 63] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 
524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 
2147483648, 4294967296, 8589934592, 17179869184, 34359738368, 68719476736, 137438953472, 274877906944, 549755813888, 
1099511627776, 2199023255552, 4398046511104, 8796093022208, 17592186044416, 35184372088832, 70368744177664, 140737488355328, 
281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 
36028797018963968, 72057594037927936, 144115188075855872, 288230376151711744, 576460752303423488, 1152921504606846976, 
2305843009213693952, 4611686018427387904];

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
pub fn bool_slice_to_i8(source: &[bool; 8]) -> i8 {
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
 * to a another i16 value. This function can be used to convert the result of
 * i16_to_i8_tuple, i16_to_i8_array or i16_to_i8_1 and i16_to_i8_2 back to the original
 * i16 value.
 */
pub fn i8s_to_i16(byte1: i8, byte2: i8) -> i16 {
    ((byte2 as i16) << 8) | ((byte1 as i16) & 0xff)
}

/**
 * Convert a pair of i8 values to an i16 value. Every distinct pair of i8 values will be mapped 
 * to a another i16 value. This function can be used to convert the result of
 * i16_to_i8_tuple, i16_to_i8_array or i16_to_i8_1 and i16_to_i8_2 back to the original
 * i16 value.
 */
pub fn i8_tuple_to_i16(bytes: (i8, i8)) -> i16 {
    i8s_to_i16(bytes.0, bytes.1)
}

/**
 * Convert an array containing 2 i8 values to a i16 value. Every distinct pair of i8 
 * values will be mapped to a another i16 value. This function can be used to convert 
 * the result of i16_to_i8_array, i16_to_i8_tuple or i16_to_i8_1 and i16_to_i8_2 back 
 * the original i16 value.
 */
pub fn i8_array_to_i16(bytes: [i8; 2]) -> i16 {
    i8s_to_i16(bytes[0], bytes[1])
}

/**
 * Convert a slice containing 2 i8 values to a i16 value. Every distinct pair of i8 
 * values will be mapped to a another i16 value. This function can be used to convert 
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
 * Convert 4 i8 values to an i32 value. Every distinct tuple of i8 values will be mapped 
 * to a another i32 value. This function can be used to convert the result of
 * i32_to_i8_tuple, i32_to_i8_array or i32_to_i8_1...4 back to the original i32 value.
 */
pub fn i8s_to_i32(byte1: i8, byte2: i8, byte3: i8, byte4: i8) -> i32 {
    ((byte4 as i32) << 24) | (((byte3 as i32) & 0xff) << 16) | (((byte2 as i32) & 0xFF) << 8) | ((byte1 as i32) & 0xFF)
}

/**
 * Convert a tuple of i8 values to an i32 value. Every distinct pair of i8 values will be mapped 
 * to a another i32 value. This function can be used to convert the result of
 * i32_to_i8_tuple, i32_to_i8_array or i32_to_i8_1...4 back to the original i32 value.
 */
pub fn i8_tuple_to_i32(bytes: (i8, i8, i8, i8)) -> i32 {
    i8s_to_i32(bytes.0, bytes.1, bytes.2, bytes.3)
}

/**
 * Convert an array containing 4 i8 values to a i32 value. Every distinct tuple of i8 
 * values will be mapped to a another i32 value. This function can be used to convert 
 * the result of i32_to_i8_array, i32_to_i8_tuple or i16_to_i8_1...4 back 
 * the original i32 value.
 */
pub fn i8_array_to_i32(bytes: [i8; 4]) -> i32 {
    i8s_to_i32(bytes[0], bytes[1], bytes[2], bytes[3])
}

/**
 * Convert a slice containing 4 i8 values to a i32 value. Every distinct tuple of i8 
 * values will be mapped to a another i32 value. This function can be used to convert 
 * the result of i32_to_i8_array, i32_to_i8_tuple or i32_to_i8_1...4 back 
 * the original i32 value.
 */
pub fn i8_slice_to_i32(bytes: &[i8; 4]) -> i32 {
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