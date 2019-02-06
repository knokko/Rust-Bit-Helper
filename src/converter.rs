static mut POWERS: [u64; 63] = [0; 63];

/**
 * Initialized static variables that are required for a part of the functionality of this library.
 * This method must be called before any functions of this library are used.
 * Calling this multiple times will not do harm, but is a waste of power and performance.
 */
pub fn init(){
    unsafe {
        POWERS = create_powers();
    }
}

fn create_powers() -> [u64; 63] {
    let mut powers: [u64; 63] = [0; 63];
    let mut l: u64 = 1;

    for index in 0..powers.len() {
        powers[index] = l;
        l *= 2;
    }

    powers
}

/**
 * Converts 8 booleans to an i8. This can be useful for efficiently storing boolean values because they occupy
 * less memory this way. Also, this can be used to efficiently store them in a file or send them over the network.
 * The original booleans can be obtained by using i8_to_booleans or i8_to_boolean_array.
 */
pub fn booleans_to_i8(b1: bool, b2: bool, b3: bool, b4: bool, b5: bool, b6: bool, b7: bool, b8: bool) -> i8 {
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
pub fn boolean_tuple_to_i8(bools: (bool,bool,bool,bool,bool,bool,bool,bool)) -> i8 {
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
pub fn boolean_array_to_i8(source: [bool; 8]) -> i8 {
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
pub fn boolean_slice_to_i8(source: &[bool; 8]) -> i8 {
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
pub fn i8_to_boolean_array(mut byte: i8) -> [bool; 8] {
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
pub fn i8_to_boolean_tuple(mut byte: i8) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
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