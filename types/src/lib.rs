

pub struct cint(int64);
pub struct sint(int64);
pub struct regint(int64);
pub struct cfloat(float64);
pub struct sfloat(float64);
pub struct cgf2n(int64);
pub struct sgf2n(int64);
pub struct cbit(bool);
pub struct sbit(bool);


pub trait ClearType {
    pub fn add();
    pub fn mul();

}

pub trait SecretSharedType: ClearType {
    pub fn open(&self) -> ClearType;
}


impl ClearType for cint {

}

impl ClearType for cfloat {

}

impl ClearType for cgf2n {

}

impl ClearType for cbit {

}

impl SecretSharedType for sint {

}

impl SecretSharedType for sfloat {

}

impl SecretSharedType for sgf2n {

}

impl SecretSharedType for sbit {

}