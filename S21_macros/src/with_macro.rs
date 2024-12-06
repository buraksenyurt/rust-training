macro_rules! max_of {
    ($x:expr) => {
        $x
    };
    ($x:expr,$y:expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
    ($x:expr,$($y:expr),+) => {
        max_of!($x,max_of!($($y),+))
    }
}

/*
   Aşağıdaki makro tanımında da crud operasyonlarını(şimdilik create) üreten bir kod
   parçası yer alıyor. Buradaki ident, ty gibi ifadeler "metasyntactic variables"
   olarak geçer ve özel anlamları vardır. Örneğin ident, identifier olarak ifade edilebilir
   ve bir tanımlayıcıyı temsil eder. ty f32, String, Option<T> vb türleri temsil eder.
*/
macro_rules! crud {
    ($struct_name:ident, $($field_name:ident: $field_type:ty),*) => {
        #[derive(Debug)]
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> $struct_name {
                $struct_name { $($field_name),* }
            }
        }
    };
}

crud!(Product, id: i32,title: String,list_price:f32,category: String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_macro() {
        assert_eq!(max_of!(1), 1);
        assert_eq!(max_of!(2, 7), 7);
        assert_eq!(max_of!(10, 0, 6, 19, 20, 3, 2, 7), 20);
        assert_eq!(max_of!(-8, -5, -3, -99), -3);
    }

    #[test]
    fn test_crud_macro() {
        let c64 = Product::new(
            1,
            "C64 monitor 14.1inch".to_string(),
            999.99,
            "Retro Computer".to_string(),
        );
        assert_eq!(c64.id, 1);
        assert_eq!(c64.title, "C64 monitor 14.1inch".to_string());
        assert_eq!(c64.list_price, 999.99);
    }
}
