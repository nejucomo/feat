pub mod task {
    use crate::Unique;

    #[derive(Debug)]
    pub struct Create;

    #[derive(Debug)]
    pub struct SetTitle(pub Unique, pub String);
}
