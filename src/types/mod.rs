mod public;
mod secret;

pub trait Open {
    fn open(self);
}
