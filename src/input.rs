use crate::converter::*;

/**
 * Instances of BitInput can be used to load data that has been stored previously. This trait contains a lot of
 * functions that start with 'read'. Every call to such a function reads the next piece of data. The data is
 * not stored under a name, but the order of the function calls determine what data will be read.
 * 
 * All instances of BitInput should have been created directly or indirectly from an instance of BitOutput. The
 * instance of BitInput can be used to load the data that has been stored using the BitOutput instance.
 * 
 * Almost all functions in BitInput have a mirror function in BitOutput. The functions in BitInput can be used
 * to load the data that has been stored using the mirror function of the BitOutput. Functions that have a mirror
 * function have a name like read_*something*. The mirror function of such a function is usually add_*something*.
 * To load the data that was stored in the BitOutput, you should call the functions of the BitInput instance
 * with the exact same order as their mirror functions were called in the BitOutput instance.
 * 
 * This trait also has 'direct' read functions. Those direct functions do the same as their non-direct counterpart,
 * but they don't check if there is enough capacity left to read. Using the direct functions is a little faster,
 * but you should use the ensure_extra_capacity function manually to make sure there is enough data to read. The
 * behavior when exceeding the capacity is undefined.
 * 
 * Because direct read functions read exactly the same data as their non-direct counterparts, both can be used to
 * read exactly the same data and it doesn't matter at all whether the corresponding BitOutput used its direct
 * or non-direct add functions.
 */
pub trait BitInput {

    /**
     * Reads a boolean from this BitInput without checking if there is enough capacity left. This function should
     * only be used after a call to ensure_extra_capacity() guarantees that there is enough capacity left.
     * 
     * The mirror function of this function is add_bool.
     */
    fn read_direct_bool(&mut self) -> bool;

    /**
     * Reads an i8 from this BitInput without checking if there is enough capacity left. This function should only
     * be used after a call to ensure_extra_capacity() guarantees that there is enough capacity left.
     * 
     * The mirror function of this function is add_i8.
     */
    fn read_direct_i8(&mut self) -> i8;

    /**
     * Ensure that at least extra_bools more booleans can be read from this BitInput. In order to make sure you can
     * safely call read_direct_i32, you need to use ensure_extra_capacity(32). 
     * 
     * This method will panic if there is no more data to read.
     * 
     * You only need to use this method if you would like to use direct read functions. The non-direct (normal)
     * read functions will always call this method before reading data.
     */
    fn ensure_extra_capacity(&mut self, extra_bools: usize);

    /**
     * Mark this BitInput as terminated. Structs implementing this trait should discard their data when this method
     * is called. After this method has been used, the behavior of calling read methods of this BitInput is undefined.
     */
    fn terminate(&mut self);

    /**
     * Reads amount bools from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first bool read will be put in dest[start_index] and the last bool read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_bools_from_slice, add_bools_from_vec,
     * add_some_bools_from_slice and add_some_bools_from_vec.
     */
    fn read_direct_bools_to_slice(&mut self, dest: &mut [bool], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_bool();
        }
    }

    /**
    * Reads amount bools from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first bool read will be put in dest[start_index] and the last bool read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_bools_from_slice, add_bools_from_vec,
    * add_some_bools_from_slice and add_some_bools_from_vec.
    */
    fn read_direct_bools_to_vec(&mut self, dest: &mut Vec<bool>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), false);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_bool();
        }
    }

    /**
    * Reads amount bools from this BitInput without checking if this BitInput has enough capacity left. The
    * read bools will be put in a new bool vector and that vector will be returned by this method.
    *
    * The first bool read will be put at the first index of result and the last bool read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_bools_from_slice, add_bools_from_vec,
    * add_some_bools_from_slice and add_some_bools_from_vec.
    */
    fn read_direct_bools(&mut self, amount: usize) -> Vec<bool> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_bool());
        }
        result
    }

    /**
     * Reads a bool vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read bool vector will be returned.
     * 
     * The mirror functions of this function are add_bool_vec and add_bool_slice.
     */
    fn read_direct_bool_vec(&mut self) -> Vec<bool> {
        let amount = self.read_direct_i32();
        self.read_direct_bools(amount as usize)
    }

    /**
     * Reads amount bools from this BitInput and puts them in dest.
     * 
     * The first bool read will be put in dest[start_index] and the last bool read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_bools_from_slice, add_bools_from_vec,
     * add_some_bools_from_slice and add_some_bools_from_vec.
     */
    fn read_bools_to_slice(&mut self, dest: &mut [bool], start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount);
        self.read_direct_bools_to_slice(dest, start_index, amount);
    }

    /**
    * Reads amount bools from this BitInput and puts them in dest.
    *
    * The first bool read will be put in dest[start_index] and the last bool read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_bools_from_slice, add_bools_from_vec,
    * add_some_bools_from_slice and add_some_bools_from_vec.
    */
    fn read_bools_to_vec(&mut self, dest: &mut Vec<bool>, start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount);
        self.read_direct_bools_to_vec(dest, start_index, amount);
    }

    /**
    * Reads amount bools from this BitInput. The read bools will be put in a new bool vector and that 
    * vector will be returned by this method.
    *
    * The first bool read will be put at the first index of result and the last bool read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_bools_from_slice, add_bools_from_vec,
    * add_some_bools_from_slice and add_some_bools_from_vec.
    */
    fn read_bools(&mut self, amount: usize) -> Vec<bool> {
        self.ensure_extra_capacity(amount);
        self.read_direct_bools(amount)
    }

    /**
     * Reads a bool vector from this BitInput. The read bool vector will be returned.
     * 
     * The mirror functions of this function are add_bool_vec and add_bool_slice.
     */
    fn read_bool_vec(&mut self) -> Vec<bool> {
        let amount = self.read_i32() as usize;
        self.ensure_extra_capacity(amount);
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_bool());
        }
        vec
    }

    /**
     * Reads amount i8s from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first i8 read will be put in dest[start_index] and the last i8 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_i8s_from_slice, add_i8s_from_vec,
     * add_some_i8s_from_slice and add_some_i8s_from_vec.
     */
    fn read_direct_i8s_to_slice(&mut self, dest: &mut [i8], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_i8();
        }
    }

    /**
    * Reads amount i8s from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first i8 read will be put in dest[start_index] and the last i8 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_i8s_from_slice, add_i8s_from_vec,
    * add_some_i8s_from_slice and add_some_i8s_from_vec.
    */
    fn read_direct_i8s_to_vec(&mut self, dest: &mut Vec<i8>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), 0);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_i8();
        }
    }

    /**
    * Reads amount i8s from this BitInput without checking if this BitInput has enough capacity left. The
    * read i8s will be put in a new i8 vector and that vector will be returned by this method.
    *
    * The first i8 read will be put at the first index of result and the last i8 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_i8s_from_slice, add_i8s_from_vec,
    * add_some_i8s_from_slice and add_some_i8s_from_vec.
    */
    fn read_direct_i8s(&mut self, amount: usize) -> Vec<i8> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_i8());
        }
        result
    }

    /**
     * Reads a i8 vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read i8 vector will be returned.
     * 
     * The mirror functions of this function are add_i8_vec and add_i8_slice.
     */
    fn read_direct_i8_vec(&mut self) -> Vec<i8> {
        let amount = self.read_direct_i32();
        self.read_direct_i8s(amount as usize)
    }
    
    /**
     * Reads amount i8s from this BitInput and puts them in dest.
     * 
     * The first i8 read will be put in dest[start_index] and the last i8 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_i8s_from_slice, add_i8s_from_vec,
     * add_some_i8s_from_slice and add_some_i8s_from_vec.
     */
    fn read_i8s_to_slice(&mut self, dest: &mut [i8], start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount * 8);
        self.read_direct_i8s_to_slice(dest, start_index, amount);
    }

    /**
    * Reads amount i8s from this BitInput and puts them in dest.
    *
    * The first i8 read will be put in dest[start_index] and the last i8 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_i8s_from_slice, add_i8s_from_vec,
    * add_some_i8s_from_slice and add_some_i8s_from_vec.
    */
    fn read_i8s_to_vec(&mut self, dest: &mut Vec<i8>, start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount * 8);
        self.read_direct_i8s_to_vec(dest, start_index, amount);
    }

    /**
    * Reads amount i8s from this BitInput. The read i8s will be put in a new i8 vector and that 
    * vector will be returned by this method.
    *
    * The first i8 read will be put at the first index of result and the last i8 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_i8s_from_slice, add_i8s_from_vec,
    * add_some_i8s_from_slice and add_some_i8s_from_vec.
    */
    fn read_i8s(&mut self, amount: usize) -> Vec<i8> {
        self.ensure_extra_capacity(amount * 8);
        self.read_direct_i8s(amount)
    }

    /**
     * Reads a i8 vector from this BitInput. The read i8 vector will be returned.
     * 
     * The mirror functions of this function are add_i8_vec and add_i8_slice.
     */
    fn read_i8_vec(&mut self) -> Vec<i8> {
        let amount = self.read_i32() as usize;
        self.ensure_extra_capacity(amount * 8);
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_i8());
        }
        vec
    }

    /**
     * Reads amount i16s from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first i16 read will be put in dest[start_index] and the last i16 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_i16s_from_slice, add_i16s_from_vec,
     * add_some_i16s_from_slice and add_some_i16s_from_vec.
     */
    fn read_direct_i16s_to_slice(&mut self, dest: &mut [i16], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_i16();
        }
    }

    /**
    * Reads amount i16s from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first i16 read will be put in dest[start_index] and the last i16 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_i16s_from_slice, add_i16s_from_vec,
    * add_some_i16s_from_slice and add_some_i16s_from_vec.
    */
    fn read_direct_i16s_to_vec(&mut self, dest: &mut Vec<i16>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), 0);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_i16();
        }
    }

    /**
    * Reads amount i16s from this BitInput without checking if this BitInput has enough capacity left. The
    * read i16s will be put in a new i16 vector and that vector will be returned by this method.
    *
    * The first i16 read will be put at the first index of result and the last i16 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_i16s_from_slice, add_i16s_from_vec,
    * add_some_i16s_from_slice and add_some_i16s_from_vec.
    */
    fn read_direct_i16s(&mut self, amount: usize) -> Vec<i16> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_i16());
        }
        result
    }

    /**
     * Reads a i16 vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read i16 vector will be returned.
     * 
     * The mirror functions of this function are add_i16_vec and add_i16_slice.
     */
    fn read_direct_i16_vec(&mut self) -> Vec<i16> {
        let amount = self.read_direct_i32();
        self.read_direct_i16s(amount as usize)
    }
    
    /**
     * Reads amount i16s from this BitInput and puts them in dest.
     * 
     * The first i16 read will be put in dest[start_index] and the last i16 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_i16s_from_slice, add_i16s_from_vec,
     * add_some_i16s_from_slice and add_some_i16s_from_vec.
     */
    fn read_i16s_to_slice(&mut self, dest: &mut [i16], start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount * 16);
        self.read_direct_i16s_to_slice(dest, start_index, amount);
    }

    /**
    * Reads amount i16s from this BitInput and puts them in dest.
    *
    * The first i16 read will be put in dest[start_index] and the last i16 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_i16s_from_slice, add_i16s_from_vec,
    * add_some_i16s_from_slice and add_some_i16s_from_vec.
    */
    fn read_i16s_to_vec(&mut self, dest: &mut Vec<i16>, start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount * 16);
        self.read_direct_i16s_to_vec(dest, start_index, amount);
    }

    /**
    * Reads amount i16s from this BitInput. The read i16s will be put in a new i16 vector and that 
    * vector will be returned by this method.
    *
    * The first i16 read will be put at the first index of result and the last i16 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_i16s_from_slice, add_i16s_from_vec,
    * add_some_i16s_from_slice and add_some_i16s_from_vec.
    */
    fn read_i16s(&mut self, amount: usize) -> Vec<i16> {
        self.ensure_extra_capacity(amount * 16);
        self.read_direct_i16s(amount)
    }

    /**
     * Reads a i16 vector from this BitInput. The read i16 vector will be returned.
     * 
     * The mirror functions of this function are add_i16_vec and add_i16_slice.
     */
    fn read_i16_vec(&mut self) -> Vec<i16> {
        let amount = self.read_i32() as usize;
        self.ensure_extra_capacity(amount * 16);
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_i16());
        }
        vec
    }




    /**
     * Reads amount i32s from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first i32 read will be put in dest[start_index] and the last i32 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_i32s_from_slice, add_i32s_from_vec,
     * add_some_i32s_from_slice and add_some_i32s_from_vec.
     */
    fn read_direct_i32s_to_slice(&mut self, dest: &mut [i32], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_i32();
        }
    }

    /**
    * Reads amount i32s from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first i32 read will be put in dest[start_index] and the last i32 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_i32s_from_slice, add_i32s_from_vec,
    * add_some_i32s_from_slice and add_some_i32s_from_vec.
    */
    fn read_direct_i32s_to_vec(&mut self, dest: &mut Vec<i32>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), 0);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_i32();
        }
    }

    /**
    * Reads amount i32s from this BitInput without checking if this BitInput has enough capacity left. The
    * read i32s will be put in a new i32 vector and that vector will be returned by this method.
    *
    * The first i32 read will be put at the first index of result and the last i32 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_i32s_from_slice, add_i32s_from_vec,
    * add_some_i32s_from_slice and add_some_i32s_from_vec.
    */
    fn read_direct_i32s(&mut self, amount: usize) -> Vec<i32> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_i32());
        }
        result
    }

    /**
     * Reads a i32 vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read i32 vector will be returned.
     * 
     * The mirror functions of this function are add_i32_vec and add_i32_slice.
     */
    fn read_direct_i32_vec(&mut self) -> Vec<i32> {
        let amount = self.read_direct_i32();
        self.read_direct_i32s(amount as usize)
    }
    
    /**
     * Reads amount i32s from this BitInput and puts them in dest.
     * 
     * The first i32 read will be put in dest[start_index] and the last i32 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_i32s_from_slice, add_i32s_from_vec,
     * add_some_i32s_from_slice and add_some_i32s_from_vec.
     */
    fn read_i32s_to_slice(&mut self, dest: &mut [i32], start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount * 32);
        self.read_direct_i32s_to_slice(dest, start_index, amount);
    }

    /**
    * Reads amount i32s from this BitInput and puts them in dest.
    *
    * The first i32 read will be put in dest[start_index] and the last i32 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_i32s_from_slice, add_i32s_from_vec,
    * add_some_i32s_from_slice and add_some_i32s_from_vec.
    */
    fn read_i32s_to_vec(&mut self, dest: &mut Vec<i32>, start_index: usize, amount: usize) {
        self.ensure_extra_capacity(amount * 32);
        self.read_direct_i32s_to_vec(dest, start_index, amount);
    }

    /**
    * Reads amount i32s from this BitInput. The read i32s will be put in a new i32 vector and that 
    * vector will be returned by this method.
    *
    * The first i32 read will be put at the first index of result and the last i32 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_i32s_from_slice, add_i32s_from_vec,
    * add_some_i32s_from_slice and add_some_i32s_from_vec.
    */
    fn read_i32s(&mut self, amount: usize) -> Vec<i32> {
        self.ensure_extra_capacity(amount * 32);
        self.read_direct_i32s(amount)
    }

    /**
     * Reads a i32 vector from this BitInput. The read i32 vector will be returned.
     * 
     * The mirror functions of this function are add_i32_vec and add_i32_slice.
     */
    fn read_i32_vec(&mut self) -> Vec<i32> {
        let amount = self.read_i32() as usize;
        self.ensure_extra_capacity(amount * 32);
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_i32());
        }
        vec
    }

    /**
     * Reads an u8 from this BitInput without checking if there is enough capacity left in this BitInput.
     * 
     * The mirror function of this function is add_u8.
     */
    fn read_direct_u8(&mut self) -> u8 {
        self.read_direct_i8() as u8
    }

    /**
     * Reads an i16 from this BitInput without checking if there is enough capacity left in this BitInput.
     * 
     * The mirror function of this function is add_i16.
     */
    fn read_direct_i16(&mut self) -> i16 {
        i8s_to_i16(self.read_direct_i8(), self.read_direct_i8())
    }

    /**
     * Reads an u16 from this BitInput without checking if there is enough capacity left in this BitInput.
     * 
     * The mirror function of this function is add_u16.
     */
    fn read_direct_u16(&mut self) -> u16 {
        i8s_to_u16(self.read_direct_i8(), self.read_direct_i8())
    }

    /**
     * Reads an i32 from this BitInput without checking if there is enough capacity left in this BitInput.
     * 
     * The mirror function of this function is add_i32.
     */
    fn read_direct_i32(&mut self) -> i32 {
        i8s_to_i32(self.read_direct_i8(), self.read_direct_i8(), self.read_direct_i8(), self.read_direct_i8())
    }

    /**
     * Reads a u32 value from this BitInput without checking if there is enough capacity left in this BitInput.
     * 
     * The mirror function of this function is add_u32.
     */
    fn read_direct_u32(&mut self) -> u32 {
        i8s_to_u32(self.read_direct_i8(), self.read_direct_i8(), self.read_direct_i8(), self.read_direct_i8())
    }

    /**
     * Reads an i8 value from this BitInput.
     * 
     * The mirror function of this function is add_i8.
     */
    fn read_i8(&mut self) -> i8 {
        self.ensure_extra_capacity(8);
        self.read_direct_i8()
    }

    /**
     * Reads a u16 value from this BitInput.
     * 
     * The mirror function of this function is add_u16.
     */
    fn read_u8(&mut self) -> u8 {
        self.ensure_extra_capacity(8);
        self.read_direct_u8()
    }

    /**
     * Reads an i16 value from this BitInput.
     * 
     * The mirror function of this function is add_i16.
     */
    fn read_i16(&mut self) -> i16 {
        self.ensure_extra_capacity(16);
        self.read_direct_i16()
    }

    /**
     * Reads a u16 value from this BitInput.
     * 
     * The mirror function of this function is add_u16.
     */
    fn read_u16(&mut self) -> u16 {
        self.ensure_extra_capacity(16);
        self.read_direct_u16()
    }

    /**
     * Reads an i32 value from this BitInput.
     * 
     * The mirror function of this function is add_i32.
     */
    fn read_i32(&mut self) -> i32 {
        self.ensure_extra_capacity(32);
        self.read_direct_i32()
    }

    /**
     * Reads a u32 value from this BitInput.
     * 
     * The mirror function of this function is add_u32.
     */
    fn read_u32(&mut self) -> u32 {
        self.ensure_extra_capacity(32);
        self.read_direct_u32()
    }
}

pub struct BoolSliceBitInput<'a> {
    bools: &'a [bool],
    read_index: usize
}

impl<'a> BoolSliceBitInput<'a> {

    pub fn new(bools: &'a[bool]) -> BoolSliceBitInput {
        BoolSliceBitInput {
            bools: bools,
            read_index: 0
        }
    }
}

impl<'a> BitInput for BoolSliceBitInput<'a> {

    fn read_direct_bool(&mut self) -> bool {
        let result = self.bools[self.read_index];
        self.read_index += 1;
        result
    }

    fn read_direct_i8(&mut self) -> i8 {
        let result = bool_slice_to_i8(&self.bools[self.read_index..self.read_index + 8]);
        self.read_index += 8;
        result
    }

    fn ensure_extra_capacity(&mut self, additional: usize){
        if self.read_index + additional > self.bools.len() {
            panic!("length is {}, but read_index is {} and additional is {}", self.bools.len(), self.read_index, additional);
        }
    }

    fn terminate(&mut self){
        self.read_index = self.bools.len();
    }
}

/**
 * A BitInput implementation that reads from an i8 vector. The most straightforward way to create an instance
 * of I8VecBitInput is by using I8VecBitInput::new(vector) where vector comes from an instance of I8VecBitOutput.
 * Using I8VecBitInput is preferred over BoolSliceBitInput because boolean arrays use surprisingly much memory.
 */
pub struct I8VecBitInput {

    vector: Vec<i8>,
    byte_index: usize,
    bool_index: usize
}

impl BitInput for I8VecBitInput {

    fn read_direct_bool(&mut self) -> bool {
        if self.bool_index == 7 {
            self.bool_index = 0;
            let result_byte = self.vector[self.byte_index];
            self.byte_index += 1;
            return result_byte >= 0;
        } else {
            let result = i8_to_bool_array(self.vector[self.byte_index])[self.bool_index];
            self.bool_index += 1;
            return result;
        }
    }

    fn read_direct_i8(&mut self) -> i8 {
        if self.bool_index == 0 {
            let result = self.vector[self.byte_index];
            self.byte_index += 1;
            return result;
        } else {
            let mut bools = [false; 8];
            let first_bools = i8_to_bool_array(self.vector[self.byte_index]);
            self.byte_index += 1;
            let second_bools = i8_to_bool_array(self.vector[self.byte_index]);
            let mut index = 0;
            while self.bool_index < 8 {
                bools[index] = first_bools[self.bool_index];
                index += 1;
                self.bool_index += 1;
            }
            self.bool_index = 0;
            while index < 8 {
                bools[index] = second_bools[self.bool_index];
                index += 1;
                self.bool_index += 1;
            }
            return bool_array_to_i8(bools);
        }
    }

    fn ensure_extra_capacity(&mut self, boolean_amount: usize){
        let remaining = 8 - self.bool_index + 8 * (self.vector.len() - self.byte_index);
        if remaining < boolean_amount {
            panic!("{} more booleans should be readable, but we only have {} left", boolean_amount, remaining);
        }
    }

    fn terminate(&mut self){
        self.vector.clear();
        self.vector.shrink_to_fit();
    }
}

impl I8VecBitInput {

    /**
     * Creates a new I8VecBitInput that will read from the given vector and start with the first i8 of the vector.
     */
    pub fn new(vector: Vec<i8>) -> I8VecBitInput {
        I8VecBitInput {
            vector: vector,
            byte_index: 0,
            bool_index: 0
        }
    }

    /**
     * Creates a new I8VecBitInput that will read from the given vector and start at the given start_index. So,
     * vector[start_index] will be the first i8 value that will be read.
     */
    pub fn with_start_index(vector: Vec<i8>, start_index: usize) -> I8VecBitInput {
        I8VecBitInput {
            vector: vector,
            byte_index: start_index,
            bool_index: 0
        }
    }
}

/**
 * A BitInput implementation that reads from a u8 vector. The most straightforward way to create an instance
 * of U8VecBitInput is by using U8VecBitInput::new(vector) where vector comes from an instance of U8VecBitOutput.
 * Using U8VecBitInput is preferred over BoolSliceBitInput because boolean arrays use surprisingly much memory.
 */
pub struct U8VecBitInput {

    vector: Vec<u8>,
    byte_index: usize,
    bool_index: usize
}

impl BitInput for U8VecBitInput {

    fn read_direct_bool(&mut self) -> bool {
        if self.bool_index == 7 {
            self.bool_index = 0;
            let result_byte = self.vector[self.byte_index] as i8;
            self.byte_index += 1;
            return result_byte >= 0;
        } else {
            let result = i8_to_bool_array(self.vector[self.byte_index] as i8)[self.bool_index];
            self.bool_index += 1;
            return result;
        }
    }

    fn read_direct_i8(&mut self) -> i8 {
        if self.bool_index == 0 {
            let result = self.vector[self.byte_index] as i8;
            self.byte_index += 1;
            return result;
        } else {
            let mut bools = [false; 8];
            let first_bools = i8_to_bool_array(self.vector[self.byte_index] as i8);
            self.byte_index += 1;
            let second_bools = i8_to_bool_array(self.vector[self.byte_index] as i8);
            let mut index = 0;
            while self.bool_index < 8 {
                bools[index] = first_bools[self.bool_index];
                index += 1;
                self.bool_index += 1;
            }
            self.bool_index = 0;
            while index < 8 {
                bools[index] = second_bools[self.bool_index];
                index += 1;
                self.bool_index += 1;
            }
            return bool_array_to_i8(bools);
        }
    }

    fn ensure_extra_capacity(&mut self, boolean_amount: usize){
        let remaining = 8 - self.bool_index + 8 * (self.vector.len() - self.byte_index);
        if remaining < boolean_amount {
            panic!("{} more booleans should be readable, but we only have {} left", boolean_amount, remaining);
        }
    }

    fn terminate(&mut self){
        self.vector.clear();
        self.vector.shrink_to_fit();
    }
}

impl U8VecBitInput {

    /**
     * Creates a new U8VecBitInput that will read from the given vector and start with the first u8 of the vector.
     */
    pub fn new(vector: Vec<u8>) -> U8VecBitInput {
        U8VecBitInput {
            vector: vector,
            byte_index: 0,
            bool_index: 0
        }
    }

    /**
     * Creates a new U8VecBitInput that will read from the given vector and start at the given start_index. So,
     * vector[start_index] will be the first u8 value that will be read.
     */
    pub fn with_start_index(vector: Vec<u8>, start_index: usize) -> U8VecBitInput {
        U8VecBitInput {
            vector: vector,
            byte_index: start_index,
            bool_index: 0
        }
    }
}