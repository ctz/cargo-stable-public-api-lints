#[non_exhaustive]
pub struct ExhaustiveStruct {
    pub pub_: (),
    priv_: (),
}

#[non_exhaustive]
pub struct Unit;

#[non_exhaustive]
pub struct TupleEmpty();
pub struct TuplePrivate(usize, usize);
#[non_exhaustive]
pub struct TuplePublic(pub usize, pub usize);
pub struct TuplePublicPrivate(pub usize, usize);

#[non_exhaustive]
pub struct NormalEmpty {}
#[non_exhaustive]
pub struct NormalPublic {
    pub a: usize,
    pub b: usize,
}
pub struct NormalPublicPrivate {
    pub a: usize,
    b: usize,
}
pub struct NormalPrivate {
    a: usize,
    b: usize,
}
