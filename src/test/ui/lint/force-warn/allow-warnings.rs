// --force-warn $LINT causes $LINT (which is warn-by-default) to warn
// despite allowing all warnings in module
// compile-flags: --force-warn dead_code -Zunstable-options
// check-pass

#![allow(warnings)]

fn dead_function() {}
//~^ WARN function is never used

fn main() {}
