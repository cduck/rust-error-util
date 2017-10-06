
#[macro_export]
macro_rules! impl_error_group {
    //( $name:ident { $( $x:ident : $y:path ),* } ) |
    ( enum $name:ident { $( $kind:ident ( $kind_type:path $( , $desc:expr )* ) ),* $(,)* } ) => {
        #[derive(Debug)]
        enum $name {
            $( $kind($kind_type), )*
        }

        impl ::std::error::Error for $name {
            fn description(&self) -> &str {
                match self {
                    $( &$name::$kind(ref e) => ::std::error::Error::description(e), )*
                }
            }
            fn cause(&self) -> Option<&::std::error::Error> {
                match self {
                    $( &$name::$kind(ref e) => return Some(e), )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match self {
                    $( &$name::$kind(ref err) => write!(f, concat!($( $desc, ": ", )* "{}"), err), )*
                }
            }
        }

        $(
        impl From<$kind_type> for $name {
            fn from(error: $kind_type) -> Self {
                $name::$kind(error)
            }
        }
        )*
    }
}

