

pub type cint = int64;
pub type sint = int64;
pub type cfloat = float64;
pub type sfloat = float64;
pub type cgf2n = int64;
pub type sgf2n = int64;
pub type cbit = bool;
pub type sbit = bool;

pub trait MPCType {

}

pub trait ClearType: MPCType {

}

pub trait SecretSharedType: MPCType {
    
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