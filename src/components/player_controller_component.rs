pub struct PlayerControllerComponent {
    pub moving_right: bool,
    pub moving_left: bool,
    pub moving_up: bool,
    pub moving_down: bool,
}

impl PlayerControllerComponent {
    pub fn new() -> Self {
        PlayerControllerComponent {
            moving_right: false,
            moving_left: false,
            moving_up: false,
            moving_down: false,
        }
    }

    pub fn set_moving_left(&mut self, state: bool) {
        self.moving_left = state
    }
    pub fn set_moving_right(&mut self, state: bool) {
        self.moving_right = state
    }
    pub fn set_moving_up(&mut self, state: bool) {
        self.moving_up = state
    }
    pub fn set_moving_down(&mut self, state: bool) {
        self.moving_down = state
    }
}
