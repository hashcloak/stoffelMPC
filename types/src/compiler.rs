/// Marker trait to tag compiler types which represent numbers
pub trait SkeletonType: Default + std::fmt::Debug + 'static {}

#[derive(Clone, Debug, Default)]
pub struct Integer(String);
impl SkeletonType for Integer {}

#[derive(Clone, Debug, Default)]
pub struct Fixed(String);
impl SkeletonType for Fixed {}

#[derive(Clone, Debug, Default)]
pub struct Float(String);
impl SkeletonType for Float {}

#[derive(Clone, Debug, Default)]
pub struct Bit(String);
impl SkeletonType for Bit {}

#[derive(Clone, Debug, Default)]
pub struct Gf2(String);
impl SkeletonType for Gf2 {}
