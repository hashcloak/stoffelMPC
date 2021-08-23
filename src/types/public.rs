pub struct PubInt(i32);

pub struct PubFixPt {
    value: i32,
    scale: i32,
}

pub struct PubGF2<const N: usize>([u8; N]);
