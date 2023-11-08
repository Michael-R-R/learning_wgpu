use super::Vertex;

pub fn _triangle() -> [Vertex; 3] {
    [
        Vertex { position: [0.0, 0.5, 0.0], color: [0.4, 0.2, 0.5], },
        Vertex { position: [-0.5, -0.5, 0.0], color: [0.4, 0.2, 0.5], },
        Vertex { position: [0.5, -0.5, 0.0], color: [0.4, 0.2, 0.5], },
    ]
}

pub fn plane() -> [Vertex; 4] {
    [
        Vertex { position: [1.0, 1.0, 0.0], color: [0.4, 0.2, 0.5], },
        Vertex { position: [-1.0, 1.0, 0.0], color: [0.4, 0.2, 0.5], }, 
        Vertex { position: [-1.0, -1.0, 0.0], color: [0.4, 0.2, 0.5], },
        Vertex { position: [1.0, -1.0, 0.0], color: [0.4, 0.2, 0.5], },
    ]
}

pub fn plane_indices() -> [u16; 6] {
    [
        0, 1, 2,
        0, 2, 3,
    ]
}