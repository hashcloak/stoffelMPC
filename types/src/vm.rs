/// The integer type which will be used in the vm

pub trait Integer: Default + Copy {
    type Representation;
}
