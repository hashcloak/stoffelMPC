use super::Open;

pub struct SecInt<T: Open>(T);

struct Shamir(i32);

impl Open for Shamir {
    fn open(self) -> PubInt {
        todo!()
    }
}

struct Masked(i32);

impl Open for Masked {
    fn open(self) -> PubInt {
        todo!()
    }
}
