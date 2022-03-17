/// Marker trait to tag compiler types which represent numbers
pub trait CompilerType: Default + std::fmt::Debug + 'static {}

#[derive(Clone, Debug, Default)]
pub struct Integer(String);
impl CompilerType for Integer {}

#[derive(Clone, Debug, Default)]
pub struct Fixed(String);
impl CompilerType for Fixed {}

#[derive(Clone, Debug, Default)]
pub struct Float(String);
impl CompilerType for Float {}

#[derive(Clone, Debug, Default)]
pub struct Bit(String);
impl CompilerType for Bit {}

#[derive(Clone, Debug, Default)]
pub struct Gf2(String);
impl CompilerType for Gf2 {}
