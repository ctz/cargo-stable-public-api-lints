#[cfg_attr(spal, allow(spal::exhaustive_struct))]
pub struct Unit;

#[cfg_attr(spal, allow(spal::exhaustive_struct))]
pub struct TupleEmpty();
#[cfg_attr(spal, allow(spal::exhaustive_struct))]
pub struct TuplePublic(pub usize, pub usize);

#[cfg_attr(spal, allow(spal::exhaustive_struct))]
pub struct NormalEmpty {}
#[cfg_attr(spal, allow(spal::exhaustive_struct))]
pub struct NormalPublic {
    pub a: usize,
    pub b: usize,
}
