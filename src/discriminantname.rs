pub trait DiscriminantName {
    fn discriminant_name(&self) -> &'static str;
}

macro_rules! disc_from_variant_name {
    ( $t:ty ) => {
        impl DiscriminantName for $t {
            fn discriminant_name(&self) -> &'static str {
                self.variant_name()
            }
        }
    };
}
