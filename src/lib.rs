#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod uair;
mod bair;
mod dair;
mod fair;
mod nair;
mod dash;
mod ftilt;
mod utilt;
mod dtilt;
mod jab;
mod upsmash;
mod fsmash;
mod dsmash;
mod nb;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    uair::install();
	bair::install();
    dair::install();
    fair::install();
    nair::install();
	dash::install();
	ftilt::install();
	utilt::install();
	dtilt::install();
	jab::install();
	upsmash::install();
	fsmash::install();
	dsmash::install();
	nb::install();
}