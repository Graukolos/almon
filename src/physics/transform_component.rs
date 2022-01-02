pub struct TransformComponent {
    transform: [[f32; 4]; 4]
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        let transform = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];

        TransformComponent {
            transform
        }
    }

    pub fn get_transform(&self) -> [[f32; 4]; 4] {
        self.transform
    }

    pub fn translate(&mut self, shift: [f32; 3]) {
        self.transform[3][0] += shift[0];
        self.transform[3][1] += shift[1];
        self.transform[3][2] += shift[2];
    }
}