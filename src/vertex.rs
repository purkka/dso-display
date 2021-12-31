#[derive(Copy, Clone)]
pub struct Vertex<T> {
    pub position: [T; 2],
    pub tex_coords: [T; 2],
}

impl<T> Vertex<T> {
    pub fn new((x, y): (T, T), (tx, ty): (T, T)) -> Self {
        Self {
            position: [x, y],
            tex_coords: [tx, ty],
        }
    }
}
