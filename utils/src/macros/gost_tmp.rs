macro_rules! gost {

    // internal rules
    (@go_make $vis:vis fn default($var:ident ($def:ident) $t:ty))
    => {
        impl $var {
            $vis fn default() -> $t {
                $def
            }
        }
    };
    (@go_make $vis:vis fn default($var:ident () $t))
    => {
        impl $var {
            $vis fn default() -> Self {
                Self::default()
            }
        }
    };
    (@go_make
        $vis:vis struct $name:ident {
            $(
                $l_vis:vis $val:ident: $t:ty
            ),*
        }
    )
    => {
        #[derive(Debug)]
        $vis struct $name {
            $(
                $l_vis $val : $t
            ),*
        }
        impl $name {
            $vis new() -> Self {
                Self { $($val : $val.default()),* }
            }
        }
    };
    //

    // Super Struct
    (
        $vis:vis struct $name {
            $(
                $r_vis:vis $var:ident => {
                    $(
                        $l_vis:vis $val:ident $t:ty $(=> $default:tt)?
                    ),*
                }
            )*
        }
    )
    => {

        #[allow(non_snake_case)]
        $vis mod $name {
            use super::*;

            $(
                $(
                    gost!(@go_make
                            $r_vis struct $var {
                                $(
                                    $l_vis $val: $t
                                ),*
                            }
                    );
                    gost!(
                        @go_make fn default($var ($defaut) $t)
                    );
                )+
            )+

            gost!(@go_make
                    struct $name {
                        $(
                            $r_vis $var: $var
                        ),*
                    }
            );

            pub fn new() -> $name {
                $name::new()
            }

        }

    };
    //

    // Super Enum
    (
        $vis:vis enum $name:ident : $t:ty $(=> $def:ident)? {   // single type
            $($var:ident => $val:ident)+
        }
    )
    => {
        ordering! {
            $vis enum $name {
                $($var),*
            }
        }
        impl $name {
            $vis fn value(&self) -> $t {
                match self {
                    $($name::$var => $val),*
                }
            }
            $vis fn default() -> Self {
                if $def {
                    return Self::$def;
                }
            }
        }
    };
}
