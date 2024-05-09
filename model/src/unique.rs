use rand::RngCore;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Unique(Inner);

const BYTELEN: usize = 32;

type Inner = [u8; BYTELEN];

impl Unique {
    pub fn generate<R>(rng: &mut R) -> Self
    where
        R: RngCore,
    {
        let mut buf = Inner::default();
        rng.fill_bytes(&mut buf);
        Unique(buf)
    }
}
