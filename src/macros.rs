/// A const time assertion.
#[macro_export]
macro_rules! const_assert {
    ($cond:expr) => {
        // Causes overflow if condition is false
        let _ = [(); 0 - (!($cond) as usize)];
    };
    ($($xs:expr),+) => {
        $crate::const_assert!($($xs)&&+);
    };
    ($($xs:expr);+ $(;)*) => {
        $crate::const_assert!($($xs),+);
    };
}

/// A compile time equality assertion
#[macro_export]
macro_rules! const_assert_eq {
    ($x:expr, $($xs:expr),+) => {
        const _: () = { $crate::const_assert!($($x == $xs),+);};
    }
}

/// A compile time size assertion. Can be used to check the actual struct size.
/// usage:
/// const_assert_size!(<struct or enum name>, <amount-of-expected-bytes>);
/// const_assert_size!(FatEntry, 8);
#[macro_export]
macro_rules! const_assert_size {
    ($struct:ident, $size:expr) => {
        $crate::const_assert_eq!(core::mem::size_of::<$struct>(), ($size));
    };
}

/// Generates a bitmask.
#[macro_export]
macro_rules! define_mask {
    ($end:expr, $beg:expr) => {
        ((1 << $end) - (1 << $beg) + (1 << $end))
    };
}

/// Used in Defreg and Defbit for creating fields.
#[macro_export]
macro_rules! define_bitfield {
    ($field:ident, $size:ident, [$($end:tt - $beg:tt)|*]) => {
        #[allow(non_upper_case_globals)]
        pub const $field: $size = $( $crate::define_mask!($end, $beg) )|*;
    };
}

/// Given a type bit, ease the bit fields manipulation.
#[macro_export]
macro_rules! defbit {
    ($regname:ident) => { defbit!($regname, u64, []); };
    ($regname:ident, $size:ident, [$($field:ident $bits:tt,)*]) => {
        #[allow(non_snake_case)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[repr(C)]
        #[allow(missing_docs)]
        pub struct $regname ($size);
        #[allow(dead_code)]
        #[allow(missing_docs)]
        impl $regname {
            #[inline(always)]
            pub fn new(data: $size) -> $regname {
                $regname(data)
            }

            #[inline(always)]
            pub fn get(&self) -> $size {
                self.0
            }

            #[inline(always)]
            pub fn get_masked(&self, mask: $size) -> $size {
                self.0 & mask
            }

            #[inline(always)]
            pub fn get_value(&self, mask: $size) -> $size {
                (self.0 & mask) >> (mask.trailing_zeros())
            }

            #[inline(always)]
            pub fn set(&mut self, val: $size) -> &mut Self {
                self.0 = val;
                self
            }

            #[inline(always)]
            pub fn set_masked(&mut self, val: $size, mask: $size) -> &mut Self {
                self.0 = (self.0 & !mask) | (val & mask);
                self
            }

            #[inline(always)]
            pub fn set_value<T: Into<$size>>(&mut self, val: T, mask: $size) -> &mut Self {
                let val:$size = val.into();
                self.0 = (self.0 & !mask)
                    | ((val << (mask.trailing_zeros())) & mask);
                self
            }

            #[inline(always)]
            pub fn set_bit(&mut self, mask: $size) -> &mut Self {
                self.0 |= mask;
                self
            }

            #[inline(always)]
            pub fn clear_bit(&mut self, mask: $size) -> &mut Self {
                self.0 &= !mask;
                self
            }

            $( $crate::define_bitfield!($field, $size, $bits); )*
        }
    }
}
