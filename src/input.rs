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
     * Reads amount booleans from this BitInput and puts them in dest, without checking if there is enough capacity
     * left in this BitInput. This method should only be used after a call to ensure_extra_capacity has been used
     * to make sure there is enough data that can be read immediathly.
     * 
     * The first boolean read will be put in dest[start_index] and the last boolean read will be put in
     * dest[start_index + amount - 1].
     * 
     * The mirror functions of this function is add_bools.
     */
    fn read_direct_some_bools(&mut self, dest: &mut [bool], start_index: usize, amount: usize) {
        let bound_index = start_index + amount;
        for index in start_index..bound_index {
            dest[index] = self.read_direct_bool();
        }
    }

    fn read_direct_bools_vec(&mut self, amount: usize) -> Vec<bool> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            result.push(self.read_direct_bool());
        }
        result
    }
}