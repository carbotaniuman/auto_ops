#[doc(hidden)]
#[macro_export]
macro_rules! _parse_binary_op_gen {
    (+, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Add, add, <$($T $(: $C)?),+>, $($t)+););
    (-, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Sub, sub, <$($T $(: $C)?),+>, $($t)+););
    (*, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Mul, mul, <$($T $(: $C)?),+>, $($t)+););
    (/, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Div, div, <$($T $(: $C)?),+>, $($t)+););
    (%, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Rem, rem, <$($T $(: $C)?),+>, $($t)+););
    (&, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(BitAnd, bitand, <$($T $(: $C)?),+>, $($t)+););
    (|, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(BitOr, bitor, <$($T $(: $C)?),+>, $($t)+););
    (^, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(BitXor, bitxor, <$($T $(: $C)?),+>, $($t)+););
    (<<, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Shl, shl, <$($T $(: $C)?),+>, $($t)+););
    (>>, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_binary_op_internal_gen!(Shr, shr, <$($T $(: $C)?),+>, $($t)+););
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_internal_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, &$lhs:ty, &$rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        $crate::_impl_binary_op_borrowed_borrowed_gen!(
            $ops_trait, $ops_fn, <$($T $(: $C)?),+>, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, &$lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        $crate::_impl_binary_op_borrowed_owned_gen!(
            $ops_trait, $ops_fn, <$($T $(: $C)?),+>, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, &$rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        $crate::_impl_binary_op_owned_borrowed_gen!(
            $ops_trait, $ops_fn, <$($T $(: $C)?),+>, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        $crate::_impl_binary_op_owned_owned_gen!(
            $ops_trait, $ops_fn, <$($T $(: $C)?),+>, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_owned_owned_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait<$rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_owned_borrowed_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait<&$rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: &$rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_borrowed_owned_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait<$rhs> for &$lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_borrowed_borrowed_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait<&$rhs> for &$lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: &$rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}
