/// The integer type which will be used in the vm

pub trait Integer: Default + Copy {
    type Representation;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: Vec<u8>) -> Self;
}
