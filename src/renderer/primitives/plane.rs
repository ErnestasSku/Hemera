use super::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Plane {
    pub vertices: Vec<Vertex>,
}

trait PlaneVertex {
    fn move_x_position(&mut self, x: f32) -> Self;
    fn move_y_position(&mut self, y: f32) -> Self;
}

impl Plane {
    pub fn new(scale: f32) -> Self {
        let vertices = vec![
            Vertex {
                position: [-scale, -scale, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [scale, -scale, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [scale, scale, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-scale, scale, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];
        Plane { vertices }
    }

    pub fn new_with_offset(scale: f32, x_offset: f32, y_offset: f32) -> Self {
        let vertices = vec![
            Vertex {
                position: [-scale + x_offset, -scale + y_offset, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [scale + x_offset, -scale + y_offset, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [scale + x_offset, scale + y_offset, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-scale + x_offset, scale + y_offset, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];
        Plane { vertices }
    }

    #[allow(dead_code)]
    pub fn move_x_offset(&mut self, x_offset: f32) -> &mut Self {
        self.vertices = self
            .vertices
            .iter_mut()
            .map(|v| v.move_x_position(x_offset))
            .collect::<Vec<Vertex>>();
        self
    }

    #[allow(dead_code)]
    pub fn move_y_offset(&mut self, y_offset: f32) -> &mut Self {
        self.vertices = self
            .vertices
            .iter_mut()
            .map(|v| v.move_y_position(y_offset))
            .collect::<Vec<Vertex>>();
        self
    }

    #[allow(dead_code)]
    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn get_indices(&self) -> Vec<u16> {
        vec![0, 1, 2, 0, 2, 3]
    }
}

impl PlaneVertex for Vertex {
    fn move_x_position(&mut self, x: f32) -> Self {
        self.position[0] += x;
        *self
    }

    fn move_y_position(&mut self, y: f32) -> Self {
        self.position[1] += y;
        *self
    }
}
