#[allow(unused_macros)]
macro_rules! assert_approx_eq {
    ($value: expr, $expected: expr) => (
        if !$value.approx_eq($expected) {
            panic!("value: {}\nexpected: {}", $value, $expected);
        }
    )
}

macro_rules! impl_ref_ops { 
    (impl $imp:ident for $t:ty, $u:ty, $method:ident, $output:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = $output;
            
            fn $method(self, other: $u) -> $output {
                $imp::$method(*self, other)
            }
        }
        
        impl<'a> $imp<&'a $u> for $t {
            type Output = $output;
            
            fn $method(self, other: &'a $u) -> $output {
                $imp::$method(self, *other)
            }
        }
        
        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = $output;
            
            fn $method(self, other: &'a $u) -> $output {
                $imp::$method(*self, *other)
            }
        }
    }
}