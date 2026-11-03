use vstd::prelude::*;
verus! {
fn transfer(from_balance: &mut u64, to_balance: &mut u64, amount: u64)
    requires
        *old(from_balance) >= amount,
        *old(to_balance) + amount <= u64::MAX,
    ensures
        *from_balance == *old(from_balance) - amount,
        *to_balance == *old(to_balance) + amount,
{
    *from_balance = *from_balance - amount;
    *to_balance = *to_balance + amount;
}
fn main() {}
}
