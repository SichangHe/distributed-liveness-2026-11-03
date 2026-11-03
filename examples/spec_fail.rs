use vstd::prelude::*;
verus! {
fn transfer(from_balance: &mut u64, to_balance: &mut u64, amount: u64)
    requires
        *old(from_balance) >= amount,
    ensures
        *from_balance == *old(from_balance) - amount,
{
    *from_balance = *from_balance - amount;
    // bug: forgot to add amount to to_balance
}
fn main() {}
}
