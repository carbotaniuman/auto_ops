#[doc(hidden)]
#[macro_export]
macro_rules! _parse_unary_op_gen {
    (-, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_unary_op_internal_gen!(Neg, neg, <$($T $(: $C)?),+>, $($t)+););
    (!, <$($T:ident $(: $C:ident)?),*>, $($t:tt)+) => ($crate::_impl_unary_op_internal_gen!(Not, not, <$($T $(: $C)?),+>, $($t)+););
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_unary_op_internal_gen {
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, &$lhs:ty, $out:ty, $lhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait for &$lhs {
            type Output = $out;

            fn $ops_fn(self) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
    ($ops_trait:ident, $ops_fn:ident, <$($T:ident $(: $C:ident)?),*>, $lhs:ty, $out:ty, $lhs_i:ident, $body:block) => {
        impl<$($T $(: $C)?),+> ::std::ops::$ops_trait for $lhs {
            type Output = $out;

            fn $ops_fn(self) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}
