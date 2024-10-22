use crypto_bigint::{Uint, Wrapping, U256, NonZero, rand_core::OsRng, Integer};
use num_integer::Integer as OpInteger;
use num_traits::{Num, Zero, One, NumOps};

/// Group order trait for easy group wrapped operations over the group order
#[repr(transparent)]
#[derive(PartialOrd, Ord, Eq, PartialEq, Zero, One, NumOps)]
pub struct Order<const L: usize>(Wrapping<Uint<L>>);

impl<const L: usize> Num for Order<L>{
}

impl<const L: usize> Order<L> {

    fn is_coprime(&self, other: &Self) {
    }
}

impl<const L: usize> OpInteger for Order<L> {

}
