/// Holds the render-relevant metadata of a cell
///
/// * `position`: cell position
#[repr(C)]
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct CellInstance {
    pub position: [i32; 2],
}

impl CellInstance {
    pub fn new(position: [i32; 2]) -> Self {
        Self { position: position }
    }
}
