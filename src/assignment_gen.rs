#[doc(hidden)]
#[macro_export]
macro_rules! _parse_assignment_op_gen {
    (+=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(AddAssign, add_assign, <$($T $(: $C)?),+>, $($t)+););
    (-=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(SubAssign, sub_assign, <$($T $(: $C)?),+>, $($t)+););
    (*=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(MulAssign, mul_assign, <$($T $(: $C)?),+>, $($t)+););
    (/=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(DivAssign, div_assign, <$($T $(: $C)?),+>, $($t)+););
    (%=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(RemAssign, rem_assign, <$($T $(: $C)?),+>, $($t)+););
    (&=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(BitAndAssign, bitand_assign, <$($T $(: $C)?),+>, $($t)+););
    (|=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(BitOrAssign, bitor_assign, <$($T $(: $C)?),+>, $($t)+););
    (^=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(BitXorAssign, bitxor_assign, <$($T $(: $C)?),+>, $($t)+););
    (<<=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(ShlAssign, shl_assign, <$($T $(: $C)?),+>, $($t)+););
    (>>=, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_assignment_op_internal_gen!(ShrAssign, shr_assign, <$($T $(: $C)?),+>, $($t)+););
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_assignment_op_internal_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, &$rhs:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait<&$rhs> for $lhs {
            fn $ops_fn(&mut self, $rhs_i: &$rhs) {
                let mut $lhs_i = self;
                $body
            }
        }
    };
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $rhs:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait<$rhs> for $lhs {
            fn $ops_fn(&mut self, $rhs_i: $rhs) {
                let mut $lhs_i = self;
                $body
            }
        }
    };
}
