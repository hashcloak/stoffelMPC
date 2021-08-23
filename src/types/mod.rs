mod public;
mod secret;

use public::{PubFixPt, PubGF2, PubInt};
use secret::{SecFixPt, SecGF2, SecInt};

pub trait Open {
    fn open(self);
}
