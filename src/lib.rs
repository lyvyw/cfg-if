#![no_std]
#![cfg_attr(test, deny(warnings))]

//! Based on https://github.com/alexcrichton/cfg-if
//! A macro for defining #[cfg] match statements.
//!
//! The macro provided by this crate, `cfg_match` is similar to the `cfg_if`
//! The difference is that this can be used inside a function or similar
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate cfg_match;
//!
//! fn foo_bar() -> bool {
//!    cfg_match_with_default! {
//!         #[cfg(foo)] => {
//!            false
//!        },
//!         #[cfg(bar)] => {
//!            false
//!        },
//!        _ => {
//!            true
//!        }
//!    }
//! }
//!
//! # fn main() {}
//! ```
//!

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_match_fail {
    () => {
        compile_error!("cfg_match: #[cfg(<check desired feature case>)] was not implemented.");
    };
}

#[macro_export]
macro_rules! cfg_match {
    ($(
        #[cfg($($meta:meta),*)] =>  $($it:block)*
    )*) => {
        __cfg_match_blocks! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            (
                 () ({cfg_match_fail!();})
            ),
        }
    };
     (
        #[cfg($($i_met:meta),*)] =>  $($i_it:block)*
        $(
            , #[cfg($($e_met:meta),*)] =>  $($e_it:block)*
        )*
    ) => {
        __cfg_match_blocks! {
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            (
                 () ({cfg_match_fail!();})
            ),
        }
    }
}

#[macro_export]
macro_rules! cfg_match_with_default {
    ($(
        #[cfg($($meta:meta),*)] =>  $($it:block)*
    )*) => {
        __cfg_match_blocks! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            (
                 () (cfg_match_fail!();)
            ),
        }
    };
    ($(
        #[cfg($($meta:meta),*)] =>  $($it:block)* ,
    )* _=>
        $($it2:block)*
    ) => {
        __cfg_match_blocks! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    };
     (
        #[cfg($($i_met:meta),*)] =>  $($i_it:block)*
        $(
            , #[cfg($($e_met:meta),*)] =>  $($e_it:block)*
        )*
    ) => {
        __cfg_match_blocks! {
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            (
                 () (cfg_match_fail!();)
            ),
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cfg_match_blocks {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($m:meta),*) ($($it:block)*) ), $($rest:tt)*) => {
        __cfg_match_apply! { cfg(all($($m,)* not(any($($not),*)))), $($it)* }

        __cfg_match_blocks! { ($($not,)* $($m,)*) ; $($rest)* }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cfg_match_apply {
    ($m:meta, $($it:block)*) => {
        $(#[$m] $it)*
    }
}

#[cfg(test)]
mod tests {
    use core::option::Option as Option2;
    fn works1() -> Option2<u32> {
        cfg_match_with_default! {
        #[cfg(test)] => {
            Some(1)
        },
        _ => {
            None
           }
        }
    }

    fn works2() -> bool {
        cfg_match_with_default! {
             #[cfg(foo)] => {
                false
            },
            #[cfg(test)] => {
                true
            },_=>{
                false
            }
        }
    }

    fn works3() -> bool {
        cfg_match_with_default! {
             #[cfg(foo)] => {
                 false
            }, _ => {
                true
            }
        }
    }

    use core::option::Option as Option3;
    fn works4() -> Option3<u32> {
        cfg_match! {
             #[cfg(test)] => {
                 Some(1)
            }
        }
    }

    fn works5() -> bool {
        cfg_match! {
             #[cfg(foo)] => {
              false
            },
            #[cfg(test)] => {
                true
            }
        }
    }

    fn works6() -> bool {
        cfg_match_with_default! {
            #[cfg(foo)] => {
                false
            },
            #[cfg(foo2)] => {
                false
            },
            _ => {
                true
            }
        }
    }

    // this case should fail ()
    // add new line after ">>>" to compile
    // >>> 
    /*
    fn works7() -> bool {
        cfg_match! {
            #[cfg(foo)] => {
                false
            },
            #[cfg(foo2)] => {
                false
            }
        }
    }
    // */

    #[test]
    fn it_works() {
        assert!(works1().is_some());
        assert!(works2());
        assert!(works3());
        assert!(works4().is_some());
        assert!(works5());
        assert!(works6());
    }
}
