#[allow(unused_macros)]
macro_rules! assert_approx_eq {
    ($value: expr, $expected: expr) => (
        if !$value.approx_eq($expected) {
            panic!("value: {}\nexpected: {}", $value, $expected);
        }
    )
}

macro_rules! impl_mut_ref_ops {
    ($imp:ident, $t:ty, $u:ty, $method:ident) => {
        impl<'a> $imp<&'a $u> for $t {
            fn $method(&mut self, other: &'a $u) {
                $imp::$method(self, *other)
            }
        }
    }
}

macro_rules! impl_ref_ops {
    // Standard ops
    ($imp:ident, $t:ty, $method:ident, $output:ty) => {
        impl<'a> $imp for &'a $t {
            type Output = $output;
            
            fn $method(self) -> $output {
                $imp::$method(*self)
            }
        }
    };
    
    ($imp:ident, $t:ty, $u:ty, $method:ident, $output:ty) => {
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
    };
}

macro_rules! impl_op {
    // Self mutable ops
    ($imp:ident, fn $method:ident(&mut $self_:ident: $t:ty) $body:block) => {
        impl $imp for $t {
            type Output = $o;
            
            fn $method(&mut $self_) $body
        }
    };
    
    ($imp:ident, fn $method:ident(&mut $self_:ident: $t:ty, $other:ident: $u:ty) $body:block) => {
        impl $imp<$u> for $t {
            fn $method(&mut $self_, $other: $u) $body
        }
        
        impl_mut_ref_ops! { $imp, $t, $u, $method }
    };
    
    // Standard ops
    ($imp:ident, fn $method:ident($self_:ident: $t:ty) -> $o:ty $body:block) => {
        impl $imp for $t {
            type Output = $o;
            
            fn $method($self_: $t) -> $o $body
        }
        
        impl_ref_ops! { $imp, $t, $method, $o }
    };

    ($imp:ident, fn $method:ident($self_:ident: $t:ty, $other:ident: $u:ty) -> $o:ty $body:block) => {
        impl $imp<$u> for $t {
            type Output = $o;
            
            fn $method($self_: $t, $other: $u) -> $o $body
        }
        
        impl_ref_ops! { $imp, $t, $u, $method, $o }
    };
}