pub struct DropGlueStruct {
    pub a: usize,
    b: DropPrivate,
}

pub struct NoDropStruct {
    pub a: usize,
}

pub enum NoDropEnum {
    A(usize),
    B(usize),
}

#[derive(Clone, Copy)]
pub struct CopyStruct {
    pub a: usize,
}

#[derive(Clone, Copy)]
pub enum CopyEnum {
    A(usize),
    B(usize),
}

struct DropPrivate {
    a: usize,
}

impl Drop for DropPrivate {
    fn drop(&mut self) {}
}

pub struct DropPublic {
    a: usize,
}

impl Drop for DropPublic {
    fn drop(&mut self) {}
}
