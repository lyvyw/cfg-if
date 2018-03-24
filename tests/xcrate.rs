#[macro_use]
extern crate cfg_match;

fn works() -> bool {
    cfg_match! {
         #[cfg(foo)] => {
            false
        },
         #[cfg(test)] => {
             true
        }
    }
}

fn works_with_default() -> bool {
    cfg_match_with_default! {
             #[cfg(foo)] => {
                false
            },
             #[cfg(bar)] => {
                 false
            },
            _ => {
                true
            }

    }
}

fn works_with_default2() -> bool {
    cfg_match_with_default! {
             #[cfg(foo)] => {
                false
            },
             #[cfg(test)] => {
                 true
            },
            _ => {
                false
            }

    }
}
/*
// uncomment this to get  acompilation  error
fn does_not_compile() -> bool {
    cfg_match! {
        #[cfg(foo)] => {
            false
        },
        #[cfg(bar)] => {
            false
        }
    }
}
*/

#[test]
fn smoke() {
    assert!(works());
    assert!(works_with_default());
    assert!(works_with_default2());
}
