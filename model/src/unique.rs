#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Unique(Inner);

const BYTELEN: usize = 32;

type Inner = [u8; BYTELEN];

impl Unique {
    pub fn generate() -> Self {
        use rand::RngCore;

        let mut buf = Inner::default();
        rand::thread_rng().fill_bytes(&mut buf);
        Unique(buf)
    }
}
