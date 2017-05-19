//! Contains OCaml types and conversion functions from runtime representations.
//!

#[allow(non_camel_case_types)]
pub type Value = usize;
pub type Uintnat = usize;
#[allow(non_camel_case_types)]
pub type Mlsize_t = Uintnat;
#[allow(non_camel_case_types)]
pub type Tag_t = u8; //typedef unsigned int tag_t; // Actually, an unsigned char
#[allow(non_camel_case_types)]
pub type Color_t = Uintnat;
#[allow(non_camel_case_types)]
pub type Mark_t = Uintnat;

/** Structure of the header:

For 16-bit and 32-bit architectures:

```text
     +--------+-------+-----+
     | wosize | color | tag |
     +--------+-------+-----+
bits  31    10 9     8 7   0
```

For 64-bit architectures:

```text
     +--------+-------+-----+
     | wosize | color | tag |
     +--------+-------+-----+
bits  63    10 9     8 7   0
```
*/
pub struct Header {}

#[macro_export]
/// (((intnat)(x) << 1) + 1)
macro_rules! val_long {
($x:expr) => ((($x as usize) << 1) + 1);
($x:ident) => ((($x as usize) << 1) + 1);
}

#[macro_export]
/// `Long_val(x)     ((x) >> 1)`
macro_rules! long_val {
($x:ident) => ($x as usize >> 1);
($x:expr) => ($x as usize >> 1);
}

#[macro_export]
/// Converts a machine `usize` into an OCaml `int`
///
/// `Val_int(x) Val_long(x)`
macro_rules! val_int {
($x:expr) => ( val_long!($x) );
($x:ident) => ( val_long!($x) );
}

#[macro_export]
/// Converts an OCaml `int` into a `usize`
/// `Int_val(x) ((int) Long_val(x))`
macro_rules! int_val {
($x:ident) => (long_val!($x));
($x:expr) => (long_val!($x));
}

// #define Max_long (((intnat)1 << (8 * sizeof(value) - 2)) - 1)
// #define Min_long (-((intnat)1 << (8 * sizeof(value) - 2)))

#[macro_export]
/// Extracts from the `$block` an OCaml value at the `$ith`-field
macro_rules! field {
    ($block:ident, $i:expr) => (
    unsafe { (block as *mut Value).offset(i)}
    );
}

/// The OCaml `()` (`unit`) value - rien.
pub const UNIT: Value = val_int!(0);

/*
pub const Num_tags: ::std::os::raw::c_ushort = 256;
pub const No_scan_tag: ::std::os::raw::c_uchar = 251;
pub const Forward_tag: ::std::os::raw::c_uchar = 250;
pub const Infix_tag: ::std::os::raw::c_uchar = 249;
pub const Object_tag: ::std::os::raw::c_uchar = 248;
pub const Closure_tag: ::std::os::raw::c_uchar = 247;
pub const Lazy_tag: ::std::os::raw::c_uchar = 246;
pub const Abstract_tag: ::std::os::raw::c_uchar = 251;
pub const String_tag: ::std::os::raw::c_uchar = 252;
pub const Double_tag: ::std::os::raw::c_uchar = 253;
pub const Double_array_tag: ::std::os::raw::c_uchar = 254;
pub const Custom_tag: ::std::os::raw::c_uchar = 255;
pub const Tag_cons: ::std::os::raw::c_uchar = 0;
*/

// Strings
/// The OCaml GC tag for a `string`
pub const STRING_TAG: u8 = 252;

/// Pointer to the first byte
#[macro_export]
macro_rules! bp_val {
  ($v: ident) => {
      $v as *const u8
  }
}

#[macro_export]
/// Extracts a machine `ptr` to the bytes making up an OCaml `string`
macro_rules! string_val {
  ($v:ident) => {
      bp_val!($v)
  }
}

extern "C" {
    /// Returns size of the string in `value` in bytes
    pub fn caml_string_length(value: Value) -> Mlsize_t;
}
