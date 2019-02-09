pub trait BitOutput {

    /**
     * Add a boolean to this BitOutput without checking if there is enough capacity left.
     */
    fn add_direct_bool(&self, boolean: bool);

    /**
     * Add an i8 to this BitOutput without checking if there is enough capacity left.
     */
    fn add_direct_i8(&self, byte: i8);

    /**
     * Ensure that at least extra_bools can be added to this BitOutput before running out of capacity.
     * So, the function add_direct_bool can safely be called extra_bools times after a call to this function.
     */
    fn ensure_extra_capacity(&self, extra_bools: u32);

    /**
     * Mark this BitOutput as terminated. If this BitOutput is connected to a stream, the stream will be closed.
     * If this BitOutput is array based, not much will happen. If this BitOutput is vector based, all
     * remaining space in the vector will be released.
     */
    fn terminate(&self);

    /**
     * Add all provided booleans to this BitOutput without checking if there is enough capacity left.
     */
    fn add_direct_bools(&self, bools: &[bool]){
        for value in bools {
            self.add_direct_bool(*value);
        }
    }
}