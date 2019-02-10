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
     * Add all provided booleans to this BitOutput without checking if there is enough capacity left. This is just
     * a shortcut for adding all booleans one by one. The amount of booleans is NOT stored, so make sure your
     * application knows how many bools were stored. The mirror function of this function is read_bools and the
     * amount of bools to read must be stored as parameter.
     * 
     * If you want to store the amount as well, use add_direct_bool_slice instead.
     */
    fn add_direct_bools(&mut self, bools: &[bool]){
        for value in bools {
            self.add_direct_bool(*value);
        }
    }

    /**
     * Add the length of the boolean slice and the values of all booleans in the slice without
     * checking the capacity. The mirror function of this function is read_bool_array. There is 
     * no read_bool_slice because it doesn't really make sense to borrow the data since the 
     * BitInput will create the array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_bool_slice(&mut self, bools: &[bool]){
        self.add_direct_i32(bools.len() as i32);
        self.add_direct_bools(bools);
    }

    /**
     * Add all i8 values in the provided slice to this BitOutput without checking the capacity.
     * A call to this function is equivalent to adding all i8 values directly one by one. The length
     * of the slice is not stored, so make sure the application always knows how many i8 values
     * to read. The mirror function of this function is read_i8s and notice that it needs the amount
     * of i8 values to read as parameter.
     * 
     * If you want to store the length as well, use add_direct_i8_slice instead.
     */
    fn add_direct_i8s(&mut self, bytes: &[i8]){
        for value in bytes {
            self.add_direct_i8(*value);
        }
    }

    /**
     * Add the length of the i8 slice and the values of all i8s in the slice without
     * checking the capacity. The mirror function of this function is read_i8_array. There is 
     * no read_i8_slice because it doesn't really make sense to borrow the data since the 
     * BitInput will create the array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i8_slice(&mut self, bytes: &[i8]){
        self.add_direct_i32(bytes.len() as i32);
        self.add_direct_i8s(bytes);
    }

    /**
     * Add the provided i16 value to this BitOutput without checking the capacity. The mirror function
     * of this function is read_i16.
     */
    fn add_direct_i16(&mut self, integer: i16){
        self.add_direct_i8(i16_to_i8_1(integer));
        self.add_direct_i8(i16_to_i8_2(integer));
    }

    /**
     * Add all i16 values in the provided slice to this BitOutput without checking the capacity.
     * A call to this function is equivalent to adding all i16 values directly one by one. The length
     * of the slice is not stored, so make sure the application always knows how many i16 values
     * to read. The mirror function of this function is read_i16s and notice that it needs the amount
     * of i16 values to read as parameter.
     * 
     * If you want to store the length as well, use add_direct_i16_slice instead.
     */
    fn add_direct_i16s(&mut self, values: &[i16]){
        for value in values {
            self.add_direct_i16(*value);
        }
    }

    /**
     * Add the length of the i16 slice and the values of all i16s in the slice without
     * checking the capacity. The mirror function of this function is read_i16_array. There is 
     * no read_i16_slice because it doesn't really make sense to borrow the data since the 
     * BitInput will create the array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i16_slice(&mut self, values: &[i16]){
        self.add_direct_i32(values.len() as i32);
        self.add_direct_i16s(values);
    }

    /**
     * Add the provided i32 value to this BitOutput without checking the capacity. The mirror function
     * of this function is read_i32.
     */
    fn add_direct_i32(&mut self, integer: i32){
        self.add_direct_i8(i32_to_i8_1(integer));
        self.add_direct_i8(i32_to_i8_2(integer));
        self.add_direct_i8(i32_to_i8_3(integer));
        self.add_direct_i8(i32_to_i8_4(integer));
    }

    /**
     * Add all i32 values in the provided slice to this BitOutput without checking the capacity.
     * A call to this function is equivalent to adding all i32 values directly one by one. The length
     * of the slice is not stored, so make sure the application always knows how many i32 values
     * to read. The mirror function of this function is read_i32s and notice that it needs the amount
     * of i32 values to read as parameter.
     * 
     * If you want to store the length as well, use add_direct_i32_slice instead.
     */
    fn add_direct_i32s(&mut self, values: &[i32]){
        for value in values {
            self.add_direct_i32(*value);
        }
    }

    /**
     * Add the length of the i32 slice and the values of all i32s in the slice without
     * checking the capacity. The mirror function of this function is read_i32_array. There is 
     * no read_i32_slice because it doesn't really make sense to borrow the data since the 
     * BitInput will create the array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_direct_i32_slice(&mut self, values: &[i32]){
        self.add_direct_i32(values.len() as i32);
        self.add_direct_i32s(values);
    }

    /**
     * Add a bool value to this BitOutput. The mirror function of this function is read_bool.
     */
    fn add_bool(&mut self, value: bool){
        self.ensure_extra_capacity(1);
        self.add_direct_bool(value);
    }

    /**
     * Add all provided booleans to this BitOutput. This is a little faster than adding all values one by one
     * because the capacity only needs to be checked once. The amount of booleans is NOT stored, so make sure your
     * application knows how many bools were stored. The mirror function of this function is read_bools and the
     * amount of bools to read must be stored as parameter.
     * 
     * If you want to store the amount as well, use add_bool_slice instead.
     */
    fn add_bools(&mut self, values: &[bool]){
        self.ensure_extra_capacity(values.len());
        self.add_direct_bools(values);
    }

    /**
     * Add the length of the boolean slice and the values of all booleans in the slice. The 
     * mirror function of this function is read_bool_array. There is no read_bool_slice because 
     * it doesn't really make sense to borrow the data since the BitInput will create the array 
     * and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_bool_slice(&mut self, values: &[bool]){
        self.ensure_extra_capacity(32 + values.len());
        self.add_direct_bool_slice(values);
    }

    /**
     * Add an i8 value to this BitOutput. The mirror function of this function is read_i8.
     */
    fn add_i8(&mut self, value: i8){
        self.ensure_extra_capacity(8);
        self.add_direct_i8(value);
    }

    /**
     * Add all i8 values in the provided slice to this BitOutput.
     * A call to this function is faster than adding all i8 values one by one because the capacity
     * only needs to be checked once. The length of the slice is not stored, so make sure the 
     * application always knows how many i8 values to read. The mirror function of this function 
     * is read_i8s and notice that it needs the amount of i8 values to read as parameter.
     * 
     * If you want to store the length as well, use add_i8_slice instead.
     */
    fn add_i8s(&mut self, values: &[i8]){
        self.ensure_extra_capacity(values.len() * 8);
        self.add_direct_i8s(values);
    }

    /**
     * Add the length of the i8 slice and the values of all i8s in the slice. The mirror 
     * function of this function is read_i8_array. There is no read_i8_slice because it 
     * doesn't really make sense to borrow the data since the BitInput will create the 
     * array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i8_slice(&mut self, values: &[i8]){
        self.ensure_extra_capacity(32 + values.len() * 8);
        self.add_direct_i8_slice(values);
    }

    /**
     * Add an i16 value to this BitOutput. The mirror function of this function is read_i16.
     */
    fn add_i16(&mut self, value: i16){
        self.ensure_extra_capacity(16);
        self.add_direct_i16(value);
    }

    /**
     * Add all i16 values in the provided slice to this BitOutput.
     * A call to this function is faster than adding all i16 values one by one because the capacity
     * only needs to be checked once. The length of the slice is not stored, so make sure the 
     * application always knows how many i16 values to read. The mirror function of this function 
     * is read_i16s and notice that it needs the amount of i16 values to read as parameter.
     * 
     * If you want to store the length as well, use add_i16_slice instead.
     */
    fn add_i16s(&mut self, values: &[i16]){
        self.ensure_extra_capacity(values.len() * 16);
        self.add_direct_i16s(values);
    }

    /**
     * Add the length of the i16 slice and the values of all i16s in the slice. The mirror 
     * function of this function is read_i16_array. There is no read_i16_slice because it 
     * doesn't really make sense to borrow the data since the BitInput will create the 
     * array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i16_slice(&mut self, values: &[i16]){
        self.ensure_extra_capacity(32 + values.len() * 16);
        self.add_direct_i16_slice(values);
    }

    /**
     * Add an i32 value to this BitOutput. The mirror function of this function is read_i32.
     */
    fn add_i32(&mut self, value: i32){
        self.ensure_extra_capacity(32);
        self.add_direct_i32(value);
    }

    /**
     * Add all i32 values in the provided slice to this BitOutput.
     * A call to this function is faster than adding all i32 values one by one because the capacity
     * only needs to be checked once. The length of the slice is not stored, so make sure the 
     * application always knows how many i32 values to read. The mirror function of this function 
     * is read_i32s and notice that it needs the amount of i32 values to read as parameter.
     * 
     * If you want to store the length as well, use add_i32_slice instead.
     */
    fn add_i32s(&mut self, values: &[i32]){
        self.ensure_extra_capacity(values.len() * 32);
        self.add_direct_i32s(values);
    }

    /**
     * Add the length of the i32 slice and the values of all i32s in the slice. The mirror 
     * function of this function is read_i32_array. There is no read_i32_slice because it 
     * doesn't really make sense to borrow the data since the BitInput will create the 
     * array and won't need it for itself.
     * 
     * The length will be stored as i32 to make sure the stored data can also be read by 
     * java or javascript applications that use the BitHelper variant for their language.
     */
    fn add_i32_slice(&mut self, values: &[i32]){
        self.ensure_extra_capacity(32 + values.len() * 32);
        self.add_direct_i32_slice(values);
    }
}

pub struct BoolVecBitOutput {
    vector: Vec<bool>
}

impl BitOutput for BoolVecBitOutput {

    fn add_direct_bool(&mut self, value: bool){
        self.vector.push(value);
    }

    fn add_direct_i8(&mut self, value: i8){
        self.add_direct_bools(&i8_to_bool_array(value));
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
}

impl std::fmt::Debug for BoolVecBitOutput {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BoolArrayBitOutput({:?} with capacity {})", self.vector, self.vector.capacity())
    }
}