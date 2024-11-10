// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.
use std::cmp::{ PartialEq};

// create wrapper class for u32 for avoid coherence rule
struct MyU32(u32);

impl PartialEq for MyU32 {
    fn eq(&self, _other: &Self) -> bool {
        &self == &_other
    }
}
