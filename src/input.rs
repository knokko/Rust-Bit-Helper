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
 * All non-direct methods of BitInput return a Result that could contain a BitInputError as error. These will be
 * returned if an attempt is made to load more data than there was stored or eventual other method-specific cases
 * where the read input is not valid. These errors will only be returned if the input data is invalid or if the
 * code that uses this bit input is not programmed correctly. If you trust your input, you can simply unwrap all
 * results. But this can be dangerous if you are for instance reading data from clients via sockets, you should
 * not just unwrap them because the clients could be malicious.
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
     * This method will return Ok if there is enough input data to be read. If there is not enough input data available
     * and thus the capacity can't be increased, this method will return an InputCapacityError.
     * 
     * You only need to use this method if you would like to use direct read functions. The non-direct (normal)
     * read functions will always call this method before reading data.
     */
    fn ensure_extra_capacity(&mut self, extra_bools: usize) -> Result<(),InputCapacityError>;

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
    fn read_bools_to_slice(&mut self, dest: &mut [bool], start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount)?;
        self.read_direct_bools_to_slice(dest, start_index, amount);
        Ok(())
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
    fn read_bools_to_vec(&mut self, dest: &mut Vec<bool>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount)?;
        self.read_direct_bools_to_vec(dest, start_index, amount);
        Ok(())
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
    fn read_bools(&mut self, amount: usize) -> Result<Vec<bool>,BitInputError> {
        self.ensure_extra_capacity(amount)?;
        Ok(self.read_direct_bools(amount))
    }

    /**
     * Reads a bool vector from this BitInput. The read bool vector will be returned.
     * 
     * The mirror functions of this function are add_bool_vec and add_bool_slice.
     */
    fn read_bool_vec(&mut self) -> Result<Vec<bool>,BitInputError> {
        let amount = self.read_i32()? as usize;
        self.ensure_extra_capacity(amount)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_bool());
        }
        Ok(vec)
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
    fn read_i8s_to_slice(&mut self, dest: &mut [i8], start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 8)?;
        self.read_direct_i8s_to_slice(dest, start_index, amount);
        Ok(())
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
    fn read_i8s_to_vec(&mut self, dest: &mut Vec<i8>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 8)?;
        self.read_direct_i8s_to_vec(dest, start_index, amount);
        Ok(())
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
    fn read_i8s(&mut self, amount: usize) -> Result<Vec<i8>,BitInputError> {
        self.ensure_extra_capacity(amount * 8)?;
        Ok(self.read_direct_i8s(amount))
    }

    /**
     * Reads a i8 vector from this BitInput. The read i8 vector will be returned.
     * 
     * The mirror functions of this function are add_i8_vec and add_i8_slice.
     */
    fn read_i8_vec(&mut self) -> Result<Vec<i8>,BitInputError> {
        let amount = self.read_i32()? as usize;
        self.ensure_extra_capacity(amount * 8)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_i8());
        }
        Ok(vec)
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
    fn read_i16s_to_slice(&mut self, dest: &mut [i16], start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 16)?;
        self.read_direct_i16s_to_slice(dest, start_index, amount);
        Ok(())
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
    fn read_i16s_to_vec(&mut self, dest: &mut Vec<i16>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 16)?;
        self.read_direct_i16s_to_vec(dest, start_index, amount);
        Ok(())
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
    fn read_i16s(&mut self, amount: usize) -> Result<Vec<i16>,BitInputError> {
        self.ensure_extra_capacity(amount * 16)?;
        Ok(self.read_direct_i16s(amount))
    }

    /**
     * Reads a i16 vector from this BitInput. The read i16 vector will be returned.
     * 
     * The mirror functions of this function are add_i16_vec and add_i16_slice.
     */
    fn read_i16_vec(&mut self) -> Result<Vec<i16>,BitInputError> {
        let amount = self.read_i32()? as usize;
        self.ensure_extra_capacity(amount * 16)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_i16());
        }
        Ok(vec)
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
    fn read_i32s_to_slice(&mut self, dest: &mut [i32], start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 32)?;
        self.read_direct_i32s_to_slice(dest, start_index, amount);
        Ok(())
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
    fn read_i32s_to_vec(&mut self, dest: &mut Vec<i32>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 32)?;
        self.read_direct_i32s_to_vec(dest, start_index, amount);
        Ok(())
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
    fn read_i32s(&mut self, amount: usize) -> Result<Vec<i32>,BitInputError> {
        self.ensure_extra_capacity(amount * 32)?;
        Ok(self.read_direct_i32s(amount))
    }

    /**
     * Reads a i32 vector from this BitInput. The read i32 vector will be returned.
     * 
     * The mirror functions of this function are add_i32_vec and add_i32_slice.
     */
    fn read_i32_vec(&mut self) -> Result<Vec<i32>,BitInputError> {
        let amount = self.read_i32()? as usize;
        self.ensure_extra_capacity(amount * 32)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_i32());
        }
        Ok(vec)
    }




    /**
     * Reads amount u8s from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first u8 read will be put in dest[start_index] and the last u8 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_u8s_from_slice, add_u8s_from_vec,
     * add_some_u8s_from_slice and add_some_u8s_from_vec.
     */
    fn read_direct_u8s_to_slice(&mut self, dest: &mut [u8], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_u8();
        }
    }

    /**
    * Reads amount u8s from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first u8 read will be put in dest[start_index] and the last u8 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_u8s_from_slice, add_u8s_from_vec,
    * add_some_u8s_from_slice and add_some_u8s_from_vec.
    */
    fn read_direct_u8s_to_vec(&mut self, dest: &mut Vec<u8>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), 0);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_u8();
        }
    }

    /**
    * Reads amount u8s from this BitInput without checking if this BitInput has enough capacity left. The
    * read u8s will be put in a new u8 vector and that vector will be returned by this method.
    *
    * The first u8 read will be put at the first index of result and the last u8 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_u8s_from_slice, add_u8s_from_vec,
    * add_some_u8s_from_slice and add_some_u8s_from_vec.
    */
    fn read_direct_u8s(&mut self, amount: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_u8());
        }
        result
    }

    /**
     * Reads a u8 vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read u8 vector will be returned.
     * 
     * The mirror functions of this function are add_u8_vec and add_u8_slice.
     */
    fn read_direct_u8_vec(&mut self) -> Vec<u8> {
        let amount = self.read_direct_u32();
        self.read_direct_u8s(amount as usize)
    }
    
    /**
     * Reads amount u8s from this BitInput and puts them in dest.
     * 
     * The first u8 read will be put in dest[start_index] and the last u8 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_u8s_from_slice, add_u8s_from_vec,
     * add_some_u8s_from_slice and add_some_u8s_from_vec.
     */
    fn read_u8s_to_slice(&mut self, dest: &mut [u8], start_index: usize, amount: usize) -> Result<(),BitInputError>{
        self.ensure_extra_capacity(amount * 8)?;
        self.read_direct_u8s_to_slice(dest, start_index, amount);
        Ok(())
    }

    /**
    * Reads amount u8s from this BitInput and puts them in dest.
    *
    * The first u8 read will be put in dest[start_index] and the last u8 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_u8s_from_slice, add_u8s_from_vec,
    * add_some_u8s_from_slice and add_some_u8s_from_vec.
    */
    fn read_u8s_to_vec(&mut self, dest: &mut Vec<u8>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 8)?;
        self.read_direct_u8s_to_vec(dest, start_index, amount);
        Ok(())
    }

    /**
    * Reads amount u8s from this BitInput. The read u8s will be put in a new u8 vector and that 
    * vector will be returned by this method.
    *
    * The first u8 read will be put at the first index of result and the last u8 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_u8s_from_slice, add_u8s_from_vec,
    * add_some_u8s_from_slice and add_some_u8s_from_vec.
    */
    fn read_u8s(&mut self, amount: usize) -> Result<Vec<u8>,BitInputError> {
        self.ensure_extra_capacity(amount * 8)?;
        Ok(self.read_direct_u8s(amount))
    }

    /**
     * Reads a u8 vector from this BitInput. The read u8 vector will be returned.
     * 
     * The mirror functions of this function are add_u8_vec and add_u8_slice.
     */
    fn read_u8_vec(&mut self) -> Result<Vec<u8>,BitInputError> {
        let amount = self.read_u32()? as usize;
        self.ensure_extra_capacity(amount * 8)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_u8());
        }
        Ok(vec)
    }

    /**
     * Reads amount u16s from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first u16 read will be put in dest[start_index] and the last u16 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_u16s_from_slice, add_u16s_from_vec,
     * add_some_u16s_from_slice and add_some_u16s_from_vec.
     */
    fn read_direct_u16s_to_slice(&mut self, dest: &mut [u16], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_u16();
        }
    }

    /**
    * Reads amount u16s from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first u16 read will be put in dest[start_index] and the last u16 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_u16s_from_slice, add_u16s_from_vec,
    * add_some_u16s_from_slice and add_some_u16s_from_vec.
    */
    fn read_direct_u16s_to_vec(&mut self, dest: &mut Vec<u16>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), 0);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_u16();
        }
    }

    /**
    * Reads amount u16s from this BitInput without checking if this BitInput has enough capacity left. The
    * read u16s will be put in a new u16 vector and that vector will be returned by this method.
    *
    * The first u16 read will be put at the first index of result and the last u16 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_u16s_from_slice, add_u16s_from_vec,
    * add_some_u16s_from_slice and add_some_u16s_from_vec.
    */
    fn read_direct_u16s(&mut self, amount: usize) -> Vec<u16> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_u16());
        }
        result
    }

    /**
     * Reads a u16 vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read u16 vector will be returned.
     * 
     * The mirror functions of this function are add_u16_vec and add_u16_slice.
     */
    fn read_direct_u16_vec(&mut self) -> Vec<u16> {
        let amount = self.read_direct_u32();
        self.read_direct_u16s(amount as usize)
    }
    
    /**
     * Reads amount u16s from this BitInput and puts them in dest.
     * 
     * The first u16 read will be put in dest[start_index] and the last u16 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_u16s_from_slice, add_u16s_from_vec,
     * add_some_u16s_from_slice and add_some_u16s_from_vec.
     */
    fn read_u16s_to_slice(&mut self, dest: &mut [u16], start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 16)?;
        self.read_direct_u16s_to_slice(dest, start_index, amount);
        Ok(())
    }

    /**
    * Reads amount u16s from this BitInput and puts them in dest.
    *
    * The first u16 read will be put in dest[start_index] and the last u16 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_u16s_from_slice, add_u16s_from_vec,
    * add_some_u16s_from_slice and add_some_u16s_from_vec.
    */
    fn read_u16s_to_vec(&mut self, dest: &mut Vec<u16>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 16)?;
        self.read_direct_u16s_to_vec(dest, start_index, amount);
        Ok(())
    }

    /**
    * Reads amount u16s from this BitInput. The read u16s will be put in a new u16 vector and that 
    * vector will be returned by this method.
    *
    * The first u16 read will be put at the first index of result and the last u16 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_u16s_from_slice, add_u16s_from_vec,
    * add_some_u16s_from_slice and add_some_u16s_from_vec.
    */
    fn read_u16s(&mut self, amount: usize) -> Result<Vec<u16>,BitInputError> {
        self.ensure_extra_capacity(amount * 16)?;
        Ok(self.read_direct_u16s(amount))
    }

    /**
     * Reads a u16 vector from this BitInput. The read u16 vector will be returned.
     * 
     * The mirror functions of this function are add_u16_vec and add_u16_slice.
     */
    fn read_u16_vec(&mut self) -> Result<Vec<u16>,BitInputError> {
        let amount = self.read_u32()? as usize;
        self.ensure_extra_capacity(amount * 16)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_u16());
        }
        Ok(vec)
    }

    /**
     * Reads amount u32s from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first u32 read will be put in dest[start_index] and the last u32 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_u32s_from_slice, add_u32s_from_vec,
     * add_some_u32s_from_slice and add_some_u32s_from_vec.
     */
    fn read_direct_u32s_to_slice(&mut self, dest: &mut [u32], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_u32();
        }
    }

    /**
    * Reads amount u32s from this BitInput and puts them in dest, without checking if there is enough capacity
    * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
    * to make sure there is enough data that can be read immediathly.
    *
    * The first u32 read will be put in dest[start_index] and the last u32 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_u32s_from_slice, add_u32s_from_vec,
    * add_some_u32s_from_slice and add_some_u32s_from_vec.
    */
    fn read_direct_u32s_to_vec(&mut self, dest: &mut Vec<u32>, start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        if bound_index > dest.len() {
            dest.resize(bound_index - dest.len(), 0);
        }
        for index in start_index..bound_index {
            dest[index] = self.read_direct_u32();
        }
    }

    /**
    * Reads amount u32s from this BitInput without checking if this BitInput has enough capacity left. The
    * read u32s will be put in a new u32 vector and that vector will be returned by this method.
    *
    * The first u32 read will be put at the first index of result and the last u32 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_u32s_from_slice, add_u32s_from_vec,
    * add_some_u32s_from_slice and add_some_u32s_from_vec.
    */
    fn read_direct_u32s(&mut self, amount: usize) -> Vec<u32> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_u32());
        }
        result
    }

    /**
     * Reads a u32 vector from this BitInput without checking if there is enough capacity left in this BitInput.
     * The read u32 vector will be returned.
     * 
     * The mirror functions of this function are add_u32_vec and add_u32_slice.
     */
    fn read_direct_u32_vec(&mut self) -> Vec<u32> {
        let amount = self.read_direct_u32();
        self.read_direct_u32s(amount as usize)
    }
    
    /**
     * Reads amount u32s from this BitInput and puts them in dest.
     * 
     * The first u32 read will be put in dest[start_index] and the last u32 read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function are add_u32s_from_slice, add_u32s_from_vec,
     * add_some_u32s_from_slice and add_some_u32s_from_vec.
     */
    fn read_u32s_to_slice(&mut self, dest: &mut [u32], start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 32)?;
        self.read_direct_u32s_to_slice(dest, start_index, amount);
        Ok(())
    }

    /**
    * Reads amount u32s from this BitInput and puts them in dest.
    *
    * The first u32 read will be put in dest[start_index] and the last u32 read will be put in
    * dest[start_index + amount - 1].
    * 
    * The mirror functions of this function are add_u32s_from_slice, add_u32s_from_vec,
    * add_some_u32s_from_slice and add_some_u32s_from_vec.
    */
    fn read_u32s_to_vec(&mut self, dest: &mut Vec<u32>, start_index: usize, amount: usize) -> Result<(),BitInputError> {
        self.ensure_extra_capacity(amount * 32)?;
        self.read_direct_u32s_to_vec(dest, start_index, amount);
        Ok(())
    }

    /**
    * Reads amount u32s from this BitInput. The read u32s will be put in a new u32 vector and that 
    * vector will be returned by this method.
    *
    * The first u32 read will be put at the first index of result and the last u32 read will be put in
    * the last index of result.
    * 
    * The mirror functions of this function are add_u32s_from_slice, add_u32s_from_vec,
    * add_some_u32s_from_slice and add_some_u32s_from_vec.
    */
    fn read_u32s(&mut self, amount: usize) -> Result<Vec<u32>,BitInputError> {
        self.ensure_extra_capacity(amount * 32)?;
        Ok(self.read_direct_u32s(amount))
    }

    /**
     * Reads a u32 vector from this BitInput. The read u32 vector will be returned.
     * 
     * The mirror functions of this function are add_u32_vec and add_u32_slice.
     */
    fn read_u32_vec(&mut self) -> Result<Vec<u32>,BitInputError> {
        let amount = self.read_u32()? as usize;
        self.ensure_extra_capacity(amount * 32)?;
        let mut vec = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read_direct_u32());
        }
        Ok(vec)
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
    fn read_i8(&mut self) -> Result<i8,BitInputError> {
        self.ensure_extra_capacity(8)?;
        Ok(self.read_direct_i8())
    }

    /**
     * Reads a u16 value from this BitInput.
     * 
     * The mirror function of this function is add_u16.
     */
    fn read_u8(&mut self) -> Result<u8,BitInputError> {
        self.ensure_extra_capacity(8)?;
        Ok(self.read_direct_u8())
    }

    /**
     * Reads an i16 value from this BitInput.
     * 
     * The mirror function of this function is add_i16.
     */
    fn read_i16(&mut self) -> Result<i16,BitInputError> {
        self.ensure_extra_capacity(16)?;
        Ok(self.read_direct_i16())
    }

    /**
     * Reads a u16 value from this BitInput.
     * 
     * The mirror function of this function is add_u16.
     */
    fn read_u16(&mut self) -> Result<u16,BitInputError> {
        self.ensure_extra_capacity(16)?;
        Ok(self.read_direct_u16())
    }

    /**
     * Reads an i32 value from this BitInput.
     * 
     * The mirror function of this function is add_i32.
     */
    fn read_i32(&mut self) -> Result<i32,BitInputError> {
        self.ensure_extra_capacity(32)?;
        Ok(self.read_direct_i32())
    }

    /**
     * Reads a u32 value from this BitInput.
     * 
     * The mirror function of this function is add_u32.
     */
    fn read_u32(&mut self) -> Result<u32,BitInputError> {
        self.ensure_extra_capacity(32)?;
        Ok(self.read_direct_u32())
    }

    /**
     * Reads the signed integer that has been stored in the next 'bits' bits. This is useful for compactly storing
     * integers that actually only need for instance 47 bits.
     * 
     * The mirror function of this function is add_sized_i64.
     */
    fn read_sized_i64(&mut self, bits: usize) -> Result<i64,BitInputError> {
        let mut bools = [false; 64];
        self.read_bools_to_slice(&mut bools, 0, bits)?;
        Ok(bools_to_sized_i64(bits, &bools[0..bits], 0))
    }

    /**
     * Reads the unsigned integer that has been stored in the next 'bits' bits, without checking
     * if there is enough capacity left in this bit input. This is useful for compactly storing
     * integers that do not really need 64 bits to be stored, but for instance only 43.
     * 
     * The mirror function of this function is add_sized_u64.
     */
    fn read_direct_sized_u64(&mut self, bits: usize) -> u64 {
        let mut bools = [false; 64];
        self.read_direct_bools_to_slice(&mut bools, 0, bits);
        bools_to_sized_u64(bits, &bools[0..bits], 0)
    }

    /**
     * Reads the unsigned integer that has been stored in the next 'bits' bits. This is useful for compactly storing
     * integers that do not really need 64 bits to be stored, but for instance only 43.
     * 
     * The mirror function of this function is add_sized_u64.
     */
    fn read_sized_u64(&mut self, bits: usize) -> Result<u64,BitInputError> {
        self.ensure_extra_capacity(bits)?;
        Ok(self.read_direct_sized_u64(bits))
    }

    /**
     * The mirror function of this function is add_var_u64.
     */
    fn read_direct_var_u64(&mut self) -> u64 {
        let bits = self.read_direct_sized_u64(6) + 1;
        self.read_direct_sized_u64(bits as usize)
    }

    /**
     * The mirror function of this function is add_var_u64.
     */
    fn read_var_u64(&mut self) -> Result<u64,BitInputError> {
        let bits = self.read_sized_u64(6)? + 1;
        self.read_sized_u64(bits as usize)
    }

    /**
     * Reads an optional string from this bit input. This method uses a weird encoding and returns an option instead
     * of just a string to make it compatible with the java and javascript bithelper variants.
     * 
     * A value of None in this method is equivalent to null (and undefined) in java and javascript. Reading a Some
     * in this method is equivalent to reading a non-null string in java or javascript.
     * 
     * This method also wraps the option into a Result because it is possible that no valid string is read from
     * this bit input or that the read length of the string exceeds the provided maximum length.. This differs 
     * from returning None because None is completely valid and simply means that None was passed to the
     * add_string method of the corresponding bit output.
     * This method will never return an error if the source of this bit input comes from a string that has been
     * stored in the corresponding bit output and the max_length is chosen carefully. So if you trust your input,
     * you can safely unwrap the result.
     * 
     * The max_length parameter is only used as a safety check. The length of the string was previously stored
     * in the add_string method of the corresponding bit output. This method will read the length and return
     * an error if the read length is larger than the max_length. The max_length makes sure that corrupted
     * input will not lead to excessive memory allocation.
     * 
     * The mirror function of this function is add_string.
     */
    fn read_string(&mut self, max_length: usize) -> Result<Option<String>,BitInputError> {
        let amount1 = self.read_i8()? as u8;
        if amount1 == 0 {
            return Ok(None);
        }
        let length;
        if amount1 < 255 {
            length = amount1 as usize - 1;
        } else {
            let length32 = self.read_i32()?;
            if length32 < 0 {
                return Err(BitInputError::StringLength(StringLengthError::negative(length32)));
            }
            length = self.read_i32()? as usize;
        }
        if length == 0 {
            return Ok(Some(String::from("")));
        }
        if length > max_length {
            return Err(BitInputError::StringLength(StringLengthError::long(length as i32, max_length)));
        }
        self.ensure_extra_capacity(21)?;
        let min = self.read_direct_u16();
        let bit_count = self.read_direct_sized_u64(5) as usize;
        if bit_count == 0 {
            let result = String::from_utf16(vec![min; length].as_slice());
            if result.is_ok(){
                return Ok(Some(result.unwrap()));
            } else {
                return Err(BitInputError::InvalidString(InvalidStringError));
            }
        } else {
            self.ensure_extra_capacity(bit_count * length)?;
            let mut chars = vec![0; length];
            for index in 0..length {
                chars[index] = min + self.read_direct_sized_u64(bit_count) as u16;
            }
            let result = String::from_utf16(chars.as_slice());
            if result.is_ok(){
                return Ok(Some(result.unwrap()));
            } else {
                return Err(BitInputError::InvalidString(InvalidStringError));
            }
        }
    }
}

/**
 * This enum represents 'everything' that can go wrong when an instance of BitInput is reading from
 * bad data. If the input data is not trusted, these kind of errors should be handled properly and
 * instead of causing the entire application to panic.
 * 
 * If the input data is trusted however, it should be safe to .unwrap() everything that is being
 * read from the BitInput instance.
 * 
 * Currently, there are 3 errors that belong to this enum, namely InputCapacityError, InvalidStringError
 * and StringLengthError. The latter 2 are only applicable when reading strings. The first one can be
 * caused by almost any method.
 */
#[derive(Debug, PartialEq)]
pub enum BitInputError {
    InputCapacity(InputCapacityError),
    InvalidString(InvalidStringError),
    StringLength(StringLengthError)
}

impl std::fmt::Display for BitInputError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/**
 * The read_string method reads the length of the string first. If the read length is negative or too big,
 * this error will be returned. 
 * 
 * Reading negative lengths is possible because the read_i32() method is
 * sometimes used for reading the length. This one is used because that one is also used in java and
 * javascript and it should be able to read strings that were written in java and javascript.
 * 
 * When the length is too long, a large vector (and later String) will have to be allocated to store it.
 * If that is really big, the application could run out of memory. This means that reading strings from
 * for instance web clients would be dangerous because a single malicious client could let the application
 * run out of memory and crash.
 */
#[derive(Debug, PartialEq)]
pub struct StringLengthError {
    read_length: i32,
    max_length: usize
}

impl StringLengthError {

    pub fn negative(read_length: i32) -> StringLengthError {
        StringLengthError {
            read_length: read_length,
            max_length: 0
        }
    }

    pub fn long(read_length: i32, max_length: usize) -> StringLengthError {
        StringLengthError {
            read_length: read_length,
            max_length: max_length
        }
    }
}

impl std::fmt::Display for StringLengthError {

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.read_length < 0 {
            write!(formatter, "Read negative string length ({})", self.read_length)
        } else {
            write!(formatter, "Read string length {}, but the maximum allowed length is {}", self.read_length, self.max_length)
        }
    }
}

impl std::error::Error for StringLengthError {

    fn description(&self) -> &str {
        if self.read_length < 0 {
            "The read string length was negative, but strings can't have a negative length"
        } else {
            "The read string length is longer that the maximum allowed string length"
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

/**
 * In order to read a string, the read_string method of BitInput will prepare a u16 vector that
 * will hold the string content until it is finished. Then it will be actually converted to a
 * String. If that u16 vector happens to contain invalid utf-16 data, this error will be returned.
 */
#[derive(Debug, PartialEq)]
pub struct InvalidStringError;

impl std::fmt::Display for InvalidStringError {

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Attempted to read a string with an invalid encoding")
    }
}

impl std::error::Error for InvalidStringError {

    fn description(&self) -> &str {
        "Attempted to read a string with an invalid encoding"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

/**
 * This is the most common BitInputError. This one will be returned when an attempt is made to read more
 * data from the BitInput than it has. This could happen when for instance not all data has been loaded
 * into the BitInput.
 */
#[derive(Debug, PartialEq)]
pub struct InputCapacityError {
    current_capacity: usize,
    max_capacity: usize,
    requested_extra_capacity: usize
}

impl InputCapacityError {

    pub fn current_capacity(&self) -> usize {
        self.current_capacity
    }

    pub fn max_capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn requested_extra_capacity(&self) -> usize {
        self.requested_extra_capacity
    }
}

impl std::convert::From<InputCapacityError> for BitInputError {

    fn from(error: InputCapacityError) -> BitInputError {
        BitInputError::InputCapacity(error)
    }
}

impl std::fmt::Display for InputCapacityError {

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Current capacity is {} and maximum capacity is {}, but {} more was requested", self.current_capacity, self.max_capacity, self.requested_extra_capacity)
    }
}

impl std::error::Error for InputCapacityError {

    fn description(&self) -> &str {
        "Not enough input data is available to read data from"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
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

    fn ensure_extra_capacity(&mut self, additional: usize) -> Result<(),InputCapacityError> {
        if self.read_index + additional > self.bools.len() {
            Err(InputCapacityError {
                current_capacity: self.read_index,
                max_capacity: self.bools.len(),
                requested_extra_capacity: additional
            })
        } else {
            Ok(())
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

    fn ensure_extra_capacity(&mut self, boolean_amount: usize) -> Result<(),InputCapacityError> {
        let remaining = 8 - self.bool_index + 8 * (self.vector.len() - self.byte_index);
        if remaining < boolean_amount {
            Err(InputCapacityError {
                current_capacity: self.bool_index,
                max_capacity: self.vector.len(),
                requested_extra_capacity: boolean_amount
            })
        } else {
            Ok(())
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
 * 
 * Terminating an U8VecBitInput will clear its vector.
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

    fn ensure_extra_capacity(&mut self, boolean_amount: usize) -> Result<(),InputCapacityError> {
        let remaining = 8 - self.bool_index + 8 * (self.vector.len() - self.byte_index);
        if remaining < boolean_amount {
            Err(InputCapacityError {
                current_capacity: self.bool_index + 8 * self.byte_index,
                max_capacity: 8 * self.vector.len(),
                requested_extra_capacity: boolean_amount
            })
        } else {
            Ok(())
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

/**
 * A BitInput implementation that reads its data from a reference to a u8 vector. Unlike, U8VecBitInput,
 * this struct will NOT own the vector it reads from and thus won't clear it when its terminate method
 * is called.
 */
pub struct U8VecRefBitInput<'a> {

    vector: &'a Vec<u8>,
    byte_index: usize,
    bool_index: usize
}

impl<'a> BitInput for U8VecRefBitInput<'a> {

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

    fn ensure_extra_capacity(&mut self, boolean_amount: usize) -> Result<(),InputCapacityError> {
        let remaining = 8 - self.bool_index + 8 * (self.vector.len() - self.byte_index);
        if remaining < boolean_amount {
            Err(InputCapacityError {
                current_capacity: self.bool_index + 8 * self.byte_index,
                max_capacity: 8 * self.vector.len(),
                requested_extra_capacity: boolean_amount
            })
        } else {
            Ok(())
        }
    }

    fn terminate(&mut self){
        // We don't own the vector, so we can't clear it
    }
}

impl<'a> U8VecRefBitInput<'a> {

    /**
     * Creates a new U8VecBitInput that will read from the given vector and start with the first u8 of the vector.
     */
    pub fn new(vector: &Vec<u8>) -> U8VecRefBitInput {
        U8VecRefBitInput {
            vector: vector,
            byte_index: 0,
            bool_index: 0
        }
    }

    /**
     * Creates a new U8VecBitInput that will read from the given vector and start at the given start_index. So,
     * vector[start_index] will be the first u8 value that will be read.
     */
    pub fn with_start_index(vector: &Vec<u8>, start_index: usize) -> U8VecRefBitInput {
        U8VecRefBitInput {
            vector: vector,
            byte_index: start_index,
            bool_index: 0
        }
    }
}