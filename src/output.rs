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
     * Add the provided i32 value to this BitOutput without checking the capacity. The mirror function
     * of this function is read_i32.
     */
    fn add_direct_i32(&mut self, integer: i32){
        let bytes = i32_to_i8_tuple(integer);
        self.add_direct_i8(bytes.0);
        self.add_direct_i8(bytes.1);
        self.add_direct_i8(bytes.2);
        self.add_direct_i8(bytes.3);
    }
}

pub struct BoolArrayBitOutput {
    vector: Vec<bool>
}

impl BitOutput for BoolArrayBitOutput {

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

impl BoolArrayBitOutput {

    pub fn new(initial_capacity: usize) -> BoolArrayBitOutput{
        BoolArrayBitOutput {
            vector: Vec::with_capacity(initial_capacity)
        }
    }
}

impl std::fmt::Debug for BoolArrayBitOutput {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BoolArrayBitOutput({:?} with capacity {})", self.vector, self.vector.capacity())
    }
}