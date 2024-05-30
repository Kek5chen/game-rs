pub struct FatPtr<I> {
    pub data: *mut I,
    pub v_table: *const (),
}
