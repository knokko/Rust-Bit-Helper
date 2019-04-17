use crate::converter::*;

/** 
 * Instances of BitOutput can be used to save data to for the purpose to load the data later.
 * This trait contains a lot of functions that start with 'add'. All of these functions store
 * the next 'piece of data'. The values are not stored under a name, but the order determines
 * how they can be loaded after saving. 
 * 
 * All instances of BitOutput can be used somehow to create an instance of BitInput. That 
 * BitInput should be used to load the data that was saved to this BitOutput. 
 * 
 * All functions of BitOutput have a 'mirror' function in BitInput that can be used to load 
 * the data that was saved with that function. To load the data correctly, the order of the
 * function calls in BitOutput must be the same as the order of the mirror functions calls
 * in the BitInput.
 * 
 * This trait can be used to store data quite compact and provides a lot of control. Every
 * boolean that is stored will only take 1/8 byte (although the bytes stored by all booleans
 * together will later have to be rounded upwards).
 * 
 * This trait also has 'direct' functions for many 'add' functions. Those 'direct' functions
 * will store the exact same data ad the standard 'add' functions, but the performance can
 * be a little better because it is assumed that the capacity is already big enough and thus
 * it will not be checked. So add_bool and add_direct_bool both have the same mirror function,
 * namely read_bool.
 */
pub trait BitOutput {

    /**
     * Add a boolean to this BitOutput without checking if there is enough capacity left. The
     * mirror function of this function is read_bool.
     */
    fn add_direct_bool(&mut self, boolean: bool);

    /**
     * Add an i8 to this BitOutput without checking if there is enough capacity left. The mirror
     * function of this function is read_i8.
     */
    fn add_direct_i8(&mut self, byte: i8);

    /**
     * Ensure that at least extra_bools bools can be added to this BitOutput before running out of capacity.
     * So, the function add_direct_bool can safely be called extra_bools times after a call to this function.
     * 
     * Notice that this function only ensures that there is capacity for at least extra_bools bools, the
     * implementation of this trait determines if and how much more capacity will be added.
     */
    fn ensure_extra_capacity(&mut self, extra_bools: usize);

    /**
     * Mark this BitOutput as terminated. If this BitOutput is connected to a stream, the stream will be closed.
     * If this BitOutput is array based, not much will happen. If this BitOutput is vector based, all
     * remaining space in the vector will be released.
     */
    fn terminate(&mut self);

    /**
     * Add the provided u8 to this BitOutput without checking the capacity of this BitOutput. The
     * mirror function of this function is read_u8.
     */
    fn add_direct_u8(&mut self, integer: u8){
        self.add_direct_i8(integer as i8);
    }

    /**
     * Add the provided i16 value to this BitOutput without checking the capacity of this BitOutput. 
     * The mirror function of this function is read_i16.
     */
    fn add_direct_i16(&mut self, integer: i16){
        self.add_direct_i8(i16_to_i8_1(integer));
        self.add_direct_i8(i16_to_i8_2(integer));
    }

    /**
     * Add the provided u16 value to this BitOutput without checking the capacity of this BitOutput.
     * The mirror function of this function is read_u16.
     */
    fn add_direct_u16(&mut self, integer: u16){
        self.add_direct_i8(u16_to_i8_1(integer));
        self.add_direct_i8(u16_to_i8_2(integer));
    }

    /**
     * Add the provided i32 value to this BitOutput without checking the capacity of this BitOutput. 
     * The mirror function of this function is read_i32.
     */
    fn add_direct_i32(&mut self, integer: i32){
        self.add_direct_i8(i32_to_i8_1(integer));
        self.add_direct_i8(i32_to_i8_2(integer));
        self.add_direct_i8(i32_to_i8_3(integer));
        self.add_direct_i8(i32_to_i8_4(integer));
    }

    /**
     * Add the provided u32 value to this BitOutput without checking the capacity of this BitOutput. 
     * The mirror function of this function is read_u32.
     */
    fn add_direct_u32(&mut self, integer: u32){
        self.add_direct_i8(u32_to_i8_1(integer));
        self.add_direct_i8(u32_to_i8_2(integer));
        self.add_direct_i8(u32_to_i8_3(integer));
        self.add_direct_i8(u32_to_i8_4(integer));
    }

    /**
     * Add all bools in the slice to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all bools one by one. The amount of bools is NOT stored, 
     * so make sure your application knows how many bools were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_bool_slice instead.
     */
    fn add_direct_bools_from_slice(&mut self, bools: &[bool]){
        for value in bools {
            self.add_direct_bool(*value);
        }
    }

    /**
     * Add all bools in the vector to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all bools one by one. The amount of bools is NOT stored, 
     * so make sure your application knows how many bools were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_bool_vec instead.
     */
    fn add_direct_bools_from_vec(&mut self, bools: &Vec<bool>){
        for value in bools {
            self.add_direct_bool(*value);
        }
    }

    /**
     * Add the bools in the range [start_index, start_index + amount> from bools to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all bools in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many bools were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     */
    fn add_direct_some_bools_from_slice(&mut self, bools: &[bool], start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_bool(bools[index]);
        }
    }

    /**
     * Add the bools in the range [start_index, start_index + amount> from bools to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all bools in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many bools were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     */
    fn add_direct_some_bools_from_vec(&mut self, bools: &Vec<bool>, start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_bool(bools[index]);
        }
    }

    /**
     * Add the length of the bool slice and the values of all bools in the slice without
     * checking the capacity of this BitOutput. Always call ensure_extra_capacity before
     * using this function.
     * 
     * The mirror function of this function is read_bool_vec. There is no read_bool_array
     * or read_bool_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_bool_slice(&mut self, bools: &[bool]){
        self.add_direct_i32(bools.len() as i32);
        self.add_direct_bools_from_slice(bools);
    }

    /**
     * Add the length of the bool vector and the values of all bools in the vector without
     * checking the capacity of this BitOutput. You should use ensure_extra_capacity before
     * calling this function.
     * 
     * The mirror function of this function is read_bool_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_bool_vec(&mut self, bools: &Vec<bool>){
        self.add_direct_i32(bools.len() as i32);
        self.add_direct_bools_from_vec(bools);
    }

    /**
     * Add all bools in the slice to this BitOutput. This faster than adding all bools one by 
     * one because the capacity only needs to be checked once. The amount of bools is NOT stored, 
     * so make sure your application knows how many bools were stored.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_bool_slice instead.
     */
    fn add_bools_from_slice(&mut self, bools: &[bool]){
        self.ensure_extra_capacity(bools.len());
        self.add_direct_bools_from_slice(bools);
    }

    /**
     * Add all bools in the vector to this BitOutput. This is faster than adding all bools one by one
     * because the capacity only needs to be checked once. The amount of bools is NOT stored, 
     * so make sure your application knows how many bools were stored.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_bool_vec instead.
     */
    fn add_bools_from_vec(&mut self, bools: &Vec<bool>){
        self.ensure_extra_capacity(bools.len());
        self.add_direct_bools_from_vec(bools);
    }

    /**
     * Add the bools in the range [start_index, start_index + amount> from bools to this BitOutput. This is 
     * faster than adding all bools in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many bools were stored.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     */
    fn add_some_bools_from_slice(&mut self, bools: &[bool], start_index: usize, amount: usize){
        self.ensure_extra_capacity(amount);
        self.add_direct_some_bools_from_slice(bools, start_index, amount);
    }

    /**
     * Add the bools in the range [start_index, start_index + amount> from bools to this BitOutput. This is 
     * faster than adding all bools in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many bools were stored.
     * 
     * The mirror functions of this funcion are read_bools, read_bools_to_slice and read_bools_to_vec.
     */
    fn add_some_bools_from_vec(&mut self, bools: &Vec<bool>, start_index: usize, amount: usize){
        self.ensure_extra_capacity(amount);
        self.add_direct_some_bools_from_vec(bools, start_index, amount);
    }

    /**
     * Add the length of the bool slice and the values of all bools in the slice to
     * this BitOutput.
     * 
     * The mirror function of this function is read_bool_vec. There is no read_bool_array
     * or read_bool_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_bool_slice(&mut self, bools: &[bool]){
        self.ensure_extra_capacity(32 + bools.len());
        self.add_direct_bool_slice(bools);
    }

    /**
     * Add the length of the bool vector and the values of all bools in the vector to
     * this BitOutput.
     * 
     * The mirror function of this function is read_bool_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_bool_vec(&mut self, bools: &Vec<bool>){
        self.ensure_extra_capacity(32 + bools.len());
        self.add_direct_bool_vec(bools);
    }

    /**
     * Add all i8s in the slice to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all i8s one by one. The amount of i8s is NOT stored, 
     * so make sure your application knows how many i8s were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_i8_slice instead.
     */
    fn add_direct_i8s_from_slice(&mut self, i8s: &[i8]){
        for value in i8s {
            self.add_direct_i8(*value);
        }
    }

    /**
     * Add all i8s in the vector to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all i8s one by one. The amount of i8s is NOT stored, 
     * so make sure your application knows how many i8s were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_i8_vec instead.
     */
    fn add_direct_i8s_from_vec(&mut self, i8s: &Vec<i8>){
        for value in i8s {
            self.add_direct_i8(*value);
        }
    }

    /**
     * Add the i8s in the range [start_index, start_index + amount> from i8s to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all i8s in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i8s were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     */
    fn add_direct_some_i8s_from_slice(&mut self, i8s: &[i8], start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_i8(i8s[index]);
        }
    }

    /**
     * Add the i8s in the range [start_index, start_index + amount> from i8s to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all i8s in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i8s were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     */
    fn add_direct_some_i8s_from_vec(&mut self, i8s: &Vec<i8>, start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_i8(i8s[index]);
        }
    }

    /**
     * Add the length of the i8 slice and the values of all i8s in the slice without
     * checking the capacity of this BitOutput. Always call ensure_extra_capacity before
     * using this function.
     * 
     * The mirror function of this function is read_i8_vec. There is no read_i8_array
     * or read_i8_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i8_slice(&mut self, i8s: &[i8]){
        self.add_direct_i32(i8s.len() as i32);
        self.add_direct_i8s_from_slice(i8s);
    }

    /**
     * Add the length of the i8 vector and the values of all i8s in the vector without
     * checking the capacity of this BitOutput. You should use ensure_extra_capacity before
     * calling this function.
     * 
     * The mirror function of this function is read_i8_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i8_vec(&mut self, i8s: &Vec<i8>){
        self.add_direct_i32(i8s.len() as i32);
        self.add_direct_i8s_from_vec(i8s);
    }

    /**
     * Add all i8s in the slice to this BitOutput. This faster than adding all i8s one by 
     * one because the capacity only needs to be checked once. The amount of i8s is NOT stored, 
     * so make sure your application knows how many i8s were stored.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_i8_slice instead.
     */
    fn add_i8s_from_slice(&mut self, i8s: &[i8]){
        self.ensure_extra_capacity(8 * i8s.len());
        self.add_direct_i8s_from_slice(i8s);
    }

    /**
     * Add all i8s in the vector to this BitOutput. This is faster than adding all i8s one by one
     * because the capacity only needs to be checked once. The amount of i8s is NOT stored, 
     * so make sure your application knows how many i8s were stored.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_i8_vec instead.
     */
    fn add_i8s_from_vec(&mut self, i8s: &Vec<i8>){
        self.ensure_extra_capacity(8 * i8s.len());
        self.add_direct_i8s_from_vec(i8s);
    }

    /**
     * Add the i8s in the range [start_index, start_index + amount> from i8s to this BitOutput. This is 
     * faster than adding all i8s in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i8s were stored.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     */
    fn add_some_i8s_from_slice(&mut self, i8s: &[i8], start_index: usize, amount: usize){
        self.ensure_extra_capacity(8 * amount);
        self.add_direct_some_i8s_from_slice(i8s, start_index, amount);
    }

    /**
     * Add the i8s in the range [start_index, start_index + amount> from i8s to this BitOutput. This is 
     * faster than adding all i8s in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i8s were stored.
     * 
     * The mirror functions of this funcion are read_i8s, read_i8s_to_slice and read_i8s_to_vec.
     */
    fn add_some_i8s_from_vec(&mut self, i8s: &Vec<i8>, start_index: usize, amount: usize){
        self.ensure_extra_capacity(8 * amount);
        self.add_direct_some_i8s_from_vec(i8s, start_index, amount);
    }

    /**
     * Add the length of the i8 slice and the values of all i8s in the slice to
     * this BitOutput.
     * 
     * The mirror function of this function is read_i8_vec. There is no read_i8_array
     * or read_i8_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i8_slice(&mut self, i8s: &[i8]){
        self.ensure_extra_capacity(32 + 8 * i8s.len());
        self.add_direct_i8_slice(i8s);
    }

    /**
     * Add the length of the i8 vector and the values of all i8s in the vector to
     * this BitOutput.
     * 
     * The mirror function of this function is read_i8_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i8_vec(&mut self, i8s: &Vec<i8>){
        self.ensure_extra_capacity(32 + 8 * i8s.len());
        self.add_direct_i8_vec(i8s);
    }

    /**
     * Add all i16s in the slice to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all i16s one by one. The amount of i16s is NOT stored, 
     * so make sure your application knows how many i16s were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_i16_slice instead.
     */
    fn add_direct_i16s_from_slice(&mut self, i16s: &[i16]){
        for value in i16s {
            self.add_direct_i16(*value);
        }
    }

    /**
     * Add all i16s in the vector to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all i16s one by one. The amount of i16s is NOT stored, 
     * so make sure your application knows how many i16s were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_i16_vec instead.
     */
    fn add_direct_i16s_from_vec(&mut self, i16s: &Vec<i16>){
        for value in i16s {
            self.add_direct_i16(*value);
        }
    }

    /**
     * Add the i16s in the range [start_index, start_index + amount> from i16s to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all i16s in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i16s were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     */
    fn add_direct_some_i16s_from_slice(&mut self, i16s: &[i16], start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_i16(i16s[index]);
        }
    }

    /**
     * Add the i16s in the range [start_index, start_index + amount> from i16s to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all i16s in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i16s were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     */
    fn add_direct_some_i16s_from_vec(&mut self, i16s: &Vec<i16>, start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_i16(i16s[index]);
        }
    }

    /**
     * Add the length of the i16 slice and the values of all i16s in the slice without
     * checking the capacity of this BitOutput. Always call ensure_extra_capacity before
     * using this function.
     * 
     * The mirror function of this function is read_i16_vec. There is no read_i16_array
     * or read_i16_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i16_slice(&mut self, i16s: &[i16]){
        self.add_direct_i32(i16s.len() as i32);
        self.add_direct_i16s_from_slice(i16s);
    }

    /**
     * Add the length of the i16 vector and the values of all i16s in the vector without
     * checking the capacity of this BitOutput. You should use ensure_extra_capacity before
     * calling this function.
     * 
     * The mirror function of this function is read_i16_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i16_vec(&mut self, i16s: &Vec<i16>){
        self.add_direct_i32(i16s.len() as i32);
        self.add_direct_i16s_from_vec(i16s);
    }

    /**
     * Add all i16s in the slice to this BitOutput. This faster than adding all i16s one by 
     * one because the capacity only needs to be checked once. The amount of i16s is NOT stored, 
     * so make sure your application knows how many i16s were stored.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_i16_slice instead.
     */
    fn add_i16s_from_slice(&mut self, i16s: &[i16]){
        self.ensure_extra_capacity(16 * i16s.len());
        self.add_direct_i16s_from_slice(i16s);
    }

    /**
     * Add all i16s in the vector to this BitOutput. This is faster than adding all i16s one by one
     * because the capacity only needs to be checked once. The amount of i16s is NOT stored, 
     * so make sure your application knows how many i16s were stored.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_i16_vec instead.
     */
    fn add_i16s_from_vec(&mut self, i16s: &Vec<i16>){
        self.ensure_extra_capacity(16 * i16s.len());
        self.add_direct_i16s_from_vec(i16s);
    }

    /**
     * Add the i16s in the range [start_index, start_index + amount> from i16s to this BitOutput. This is 
     * faster than adding all i16s in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i16s were stored.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     */
    fn add_some_i16s_from_slice(&mut self, i16s: &[i16], start_index: usize, amount: usize){
        self.ensure_extra_capacity(16 * amount);
        self.add_direct_some_i16s_from_slice(i16s, start_index, amount);
    }

    /**
     * Add the i16s in the range [start_index, start_index + amount> from i16s to this BitOutput. This is 
     * faster than adding all i16s in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i16s were stored.
     * 
     * The mirror functions of this funcion are read_i16s, read_i16s_to_slice and read_i16s_to_vec.
     */
    fn add_some_i16s_from_vec(&mut self, i16s: &Vec<i16>, start_index: usize, amount: usize){
        self.ensure_extra_capacity(16 * amount);
        self.add_direct_some_i16s_from_vec(i16s, start_index, amount);
    }

    /**
     * Add the length of the i16 slice and the values of all i16s in the slice to
     * this BitOutput.
     * 
     * The mirror function of this function is read_i16_vec. There is no read_i16_array
     * or read_i16_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i16_slice(&mut self, i16s: &[i16]){
        self.ensure_extra_capacity(32 + 16 * i16s.len());
        self.add_direct_i16_slice(i16s);
    }

    /**
     * Add the length of the i16 vector and the values of all i16s in the vector to
     * this BitOutput.
     * 
     * The mirror function of this function is read_i16_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i16_vec(&mut self, i16s: &Vec<i16>){
        self.ensure_extra_capacity(32 + 16 * i16s.len());
        self.add_direct_i16_vec(i16s);
    }




    /**
     * Add all i32s in the slice to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all i32s one by one. The amount of i32s is NOT stored, 
     * so make sure your application knows how many i32s were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_i32_slice instead.
     */
    fn add_direct_i32s_from_slice(&mut self, i32s: &[i32]){
        for value in i32s {
            self.add_direct_i32(*value);
        }
    }

    /**
     * Add all i32s in the vector to this BitOutput without checking if there is enough capacity left in this
     * BitOutput. This is just a shortcut for adding all i32s one by one. The amount of i32s is NOT stored, 
     * so make sure your application knows how many i32s were stored. You should always use
     * ensure_extra_capacity before calling this function.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_direct_i32_vec instead.
     */
    fn add_direct_i32s_from_vec(&mut self, i32s: &Vec<i32>){
        for value in i32s {
            self.add_direct_i32(*value);
        }
    }

    /**
     * Add the i32s in the range [start_index, start_index + amount> from i32s to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all i32s in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i32s were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     */
    fn add_direct_some_i32s_from_slice(&mut self, i32s: &[i32], start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_i32(i32s[index]);
        }
    }

    /**
     * Add the i32s in the range [start_index, start_index + amount> from i32s to this BitOutput without
     * checking the capacity of this BitOutput. This is just a shortcut for adding all i32s in that range
     * directly. The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i32s were stored. Also make sure to use ensure_extra_capacity before calling this
     * function.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     */
    fn add_direct_some_i32s_from_vec(&mut self, i32s: &Vec<i32>, start_index: usize, amount: usize){
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            self.add_direct_i32(i32s[index]);
        }
    }

    /**
     * Add the length of the i32 slice and the values of all i32s in the slice without
     * checking the capacity of this BitOutput. Always call ensure_extra_capacity before
     * using this function.
     * 
     * The mirror function of this function is read_i32_vec. There is no read_i32_array
     * or read_i32_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i32_slice(&mut self, i32s: &[i32]){
        self.add_direct_i32(i32s.len() as i32);
        self.add_direct_i32s_from_slice(i32s);
    }

    /**
     * Add the length of the i32 vector and the values of all i32s in the vector without
     * checking the capacity of this BitOutput. You should use ensure_extra_capacity before
     * calling this function.
     * 
     * The mirror function of this function is read_i32_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i32_vec(&mut self, i32s: &Vec<i32>){
        self.add_direct_i32(i32s.len() as i32);
        self.add_direct_i32s_from_vec(i32s);
    }

    /**
     * Add all i32s in the slice to this BitOutput. This faster than adding all i32s one by 
     * one because the capacity only needs to be checked once. The amount of i32s is NOT stored, 
     * so make sure your application knows how many i32s were stored.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_i32_slice instead.
     */
    fn add_i32s_from_slice(&mut self, i32s: &[i32]){
        self.ensure_extra_capacity(32 * i32s.len());
        self.add_direct_i32s_from_slice(i32s);
    }

    /**
     * Add all i32s in the vector to this BitOutput. This is faster than adding all i32s one by one
     * because the capacity only needs to be checked once. The amount of i32s is NOT stored, 
     * so make sure your application knows how many i32s were stored.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     * 
     * If you want to store the length of the vector as well, use add_i32_vec instead.
     */
    fn add_i32s_from_vec(&mut self, i32s: &Vec<i32>){
        self.ensure_extra_capacity(32 * i32s.len());
        self.add_direct_i32s_from_vec(i32s);
    }

    /**
     * Add the i32s in the range [start_index, start_index + amount> from i32s to this BitOutput. This is 
     * faster than adding all i32s in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i32s were stored.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     */
    fn add_some_i32s_from_slice(&mut self, i32s: &[i32], start_index: usize, amount: usize){
        self.ensure_extra_capacity(32 * amount);
        self.add_direct_some_i32s_from_slice(i32s, start_index, amount);
    }

    /**
     * Add the i32s in the range [start_index, start_index + amount> from i32s to this BitOutput. This is 
     * faster than adding all i32s in that range one by one because the capacity only needs to be checked once. 
     * The amount and start_index are NOT stored in this BitOutput, so make sure your application
     * knows how many i32s were stored.
     * 
     * The mirror functions of this funcion are read_i32s, read_i32s_to_slice and read_i32s_to_vec.
     */
    fn add_some_i32s_from_vec(&mut self, i32s: &Vec<i32>, start_index: usize, amount: usize){
        self.ensure_extra_capacity(32 * amount);
        self.add_direct_some_i32s_from_vec(i32s, start_index, amount);
    }

    /**
     * Add the length of the i32 slice and the values of all i32s in the slice to
     * this BitOutput.
     * 
     * The mirror function of this function is read_i32_vec. There is no read_i32_array
     * or read_i32_slice because array sizes in Rust must be known at compile time.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i32_slice(&mut self, i32s: &[i32]){
        self.ensure_extra_capacity(32 + 32 * i32s.len());
        self.add_direct_i32_slice(i32s);
    }

    /**
     * Add the length of the i32 vector and the values of all i32s in the vector to
     * this BitOutput.
     * 
     * The mirror function of this function is read_i32_vec.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i32_vec(&mut self, i32s: &Vec<i32>){
        self.ensure_extra_capacity(32 + 32 * i32s.len());
        self.add_direct_i32_vec(i32s);
    }

    /**
     * Add a bool value to this BitOutput. The mirror function of this function is read_bool.
     */
    fn add_bool(&mut self, value: bool){
        self.ensure_extra_capacity(1);
        self.add_direct_bool(value);
    }

    /**
     * Add an i8 value to this BitOutput. The mirror function of this function is read_i8.
     */
    fn add_i8(&mut self, value: i8){
        self.ensure_extra_capacity(8);
        self.add_direct_i8(value);
    }

    /**
     * Add a u8 value to this BitOutput. The mirror function of this function is read_i=u8.
     */
    fn add_u8(&mut self, value: u8){
        self.ensure_extra_capacity(8);
        self.add_direct_u8(value);
    }

    /**
     * Add an i16 value to this BitOutput. The mirror function of this function is read_i16.
     */
    fn add_i16(&mut self, value: i16){
        self.ensure_extra_capacity(16);
        self.add_direct_i16(value);
    }

    /**
     * Add a u16 value to this BitOutput. The mirror function of this function is read_u16.
     */
    fn add_u16(&mut self, value: u16){
        self.ensure_extra_capacity(16);
        self.add_direct_u16(value);
    }

    /**
     * Add an i32 value to this BitOutput. The mirror function of this function is read_i32.
     */
    fn add_i32(&mut self, value: i32){
        self.ensure_extra_capacity(32);
        self.add_direct_i32(value);
    }

    /**
     * Add a u32 value to this BitOutput. The mirror function of this function is read_u32.
     */
    fn add_u32(&mut self, value: u32){
        self.ensure_extra_capacity(32);
        self.add_direct_u32(value);
    }

    /**
     * Stores the given signed integer using the given amount of bits. The number of bits
     * can be any integer in the interval [0, 64]. This function allows you to store integers
     * that only need for instance 37 bits compactly.
     * 
     * The given value must be in the interval [-2^(bits - 1), 2^(bits - 1) - 1]. If it is not,
     * this function will panic.
     * 
     * The mirror function of this function is read_signed_int.
     */
    fn add_var_int64(&mut self, value: i64, bits: usize){
        self.ensure_extra_capacity(bits);

        // It is not allowed to create a variable length array, so 64 is the safe choise
        let mut buffer = [false; 64];
        signed_int_to_bools(value, bits, &mut buffer, 0);
        self.add_direct_bool_slice(&buffer[0..bits]);
    }
}

/**
 * This is the most straight-forward implementation of BitOutput. It literally uses booleans to store
 * its data. Unfortunately, boolean vectors take a lot of memory, so this is usually not a compact
 * way to store data.
 */
pub struct BoolVecBitOutput {
    vector: Vec<bool>
}

impl BitOutput for BoolVecBitOutput {

    fn add_direct_bool(&mut self, value: bool){
        self.vector.push(value);
    }

    fn add_direct_i8(&mut self, value: i8){
        self.add_direct_bools_from_slice(&i8_to_bool_array(value));
    }

    fn ensure_extra_capacity(&mut self, extra_bools: usize){
        self.vector.reserve(extra_bools);
    }

    fn terminate(&mut self){
        self.vector.shrink_to_fit();
    }
}

impl BoolVecBitOutput {

    pub fn new(initial_capacity: usize) -> BoolVecBitOutput{
        BoolVecBitOutput {
            vector: Vec::with_capacity(initial_capacity)
        }
    }

    pub fn get_slice(&self) -> &[bool] {
        self.vector.as_slice()
    }

    pub fn get_vec(&self) -> &Vec<bool> {
        &self.vector
    }
}

impl std::fmt::Debug for BoolVecBitOutput {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BoolArrayBitOutput({:?} with capacity {})", self.vector, self.vector.capacity())
    }
}

/**
 * An implementation of BitOutput that uses a Vec<i8> to store its data. This should be much more memory efficient that
 * the BoolVecBitOutput because computers use surprisingly much data to store a boolean vector.
 */
pub struct I8VecBitOutput {

    /**
     * The backing vector of this I8VecBitOutput. This is public because it can be quite convenient for the owner of
     * this bit output. This vector should usually not be accessed until all data has been written and the data is about
     * to be stored or sent. Accessing this vector directly is faster than using to_i8_vector() because it doesn't need
     * to clone the vector.
     * 
     * This vector could have more capacity than necessary if the terminate() method of this bit output has not (yet)
     * been called.
     */
    pub vector: Vec<i8>,
    byte_index: usize,
    bool_index: usize
}

impl BitOutput for I8VecBitOutput {

    fn add_direct_bool(&mut self, value: bool){
        let mut bools = i8_to_bool_array(self.vector[self.byte_index]);
        bools[self.bool_index] = value;
        self.bool_index += 1;
        self.vector[self.byte_index] = bool_array_to_i8(bools);
        if self.bool_index == 8 {
            self.bool_index = 0;
            self.byte_index += 1;
            self.vector.push(0);
        }
    }

    fn add_direct_i8(&mut self, value: i8){
        if self.bool_index == 0 {
            self.vector.push(value);
            self.byte_index += 1;
        } else {
            let bool_values = i8_to_bool_array(value);
            let mut value_index = 0;
            let mut current = i8_to_bool_array(self.vector[self.byte_index]);
            let mut next = [false; 8];
            while self.bool_index < 8 {
                current[self.bool_index] = bool_values[value_index];
                value_index += 1;
                self.bool_index += 1;
            }
            self.bool_index = 0;
            while value_index < 8 {
                next[self.bool_index] = bool_values[value_index];
                self.bool_index += 1;
                value_index += 1;
            }
            self.vector[self.byte_index] = bool_array_to_i8(current);
            self.vector.push(bool_array_to_i8(next));
            self.byte_index += 1;
        }
    }

    fn ensure_extra_capacity(&mut self, bool_amount: usize){
        let mut extra = bool_amount / 8;
        if bool_amount - extra * 8 + self.bool_index >= 8 {
			extra += 1;
		}
        self.vector.reserve(extra);
    }

    fn terminate(&mut self){
        self.vector.shrink_to_fit();
    }
}

impl I8VecBitOutput {

    /**
     * Creates a new instance of I8VecBitOutput with the given capacity in bytes. Please try to use a good capacity because
     * that will improve the performance and memory usage of this instance.
     */
    pub fn with_capacity(capacity: usize) -> I8VecBitOutput {
        let mut vector = Vec::with_capacity(capacity);
        vector.push(0);
        I8VecBitOutput {
            vector: vector,
            byte_index: 0,
            bool_index: 0
        }
    }

    /**
     * Returns a copy of the vector of this bit output. It will have exactly the required length and modifications to the
     * returned vector will not affect the vector of this bit output.
     * If you don't want to copy the vector of this bit output, you can directly access the vector of this struct instead,
     * but use it carefully.
     */
    pub fn to_i8_vector(&self) -> Vec<i8> {
        self.vector.clone()
    }
}

impl std::fmt::Debug for I8VecBitOutput {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "I8VecBitOutput({:?} with capacity {})", self.vector, self.vector.capacity())
    }
}