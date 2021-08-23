use super::Open;

pub struct SecInt<T: Open>(T);
pub struct SecFixPt<T: Open>(T);
pub struct SecGF2<T: Open, const N: usize>(T);

struct BGW(Shamir);

impl Open for BGW {
    fn open(self) {
        todo!()
    }
}

struct GMW(i32);

impl Open for Masked {
    fn open(self) {
        todo!()
    }
}
