pub struct ExhaustiveStruct {
    pub pub_: (),
    priv_: (),
}

pub struct Unit;

pub struct TupleEmpty();
pub struct TuplePrivate(usize, usize);
pub struct TuplePublic(pub usize, pub usize);
pub struct TuplePublicPrivate(pub usize, usize);

pub struct NormalEmpty {}
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
