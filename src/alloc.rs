use mlvalues::{Mlsize_t, Value, Tag_t};

extern "C" {
    pub fn caml_alloc(size: Mlsize_t, tag: Tag_t) -> Value;
    pub fn caml_alloc_small(size: Mlsize_t, tag: Tag_t) -> Value;
    pub fn caml_alloc_tuple(size: Mlsize_t) -> Value;
    pub fn caml_alloc_string(size: Mlsize_t) -> Value; // size in bytes
    pub fn caml_copy_string(string: *const u8) -> Value;
    pub fn caml_copy_string_array(arr: *const *const u8) -> Value;

    pub fn caml_copy_double(double: f64) -> Value;
    pub fn caml_copy_int32(int: i32) -> Value; // defined in [ints.c]
    pub fn caml_copy_int64(int: i64) -> Value; // defined in [ints.c]
    pub fn caml_copy_nativeint(int: isize) -> Value; // defined in [ints.c]
    pub fn caml_alloc_array(value: (unsafe extern "C" fn(*const u8) -> Value),
                            array: *mut *mut u8)
                            -> Value;
// CAMLextern value caml_alloc_sprintf( const char * format, ... ); // this is going to be interesting
}
