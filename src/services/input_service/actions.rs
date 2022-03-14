use instant::Instant;

// The u8 is the timeout until it can next be activated in ticks. The bool tells us if its triggered
/// The action sheet is the list of actions to perform. This is abstracted to allow all forms of input to modify these properties. It is recreated every frame
pub struct ActionSheet {
    use_item: (Instant, bool),
    jump: (Instant, bool),
    toggle_hud: (Instant, bool),
    toggle_fullscreen: (Instant, bool),
    screenshot: (Instant, bool),
    sprinting: bool,
    /// Whether we should toggle the pause menu
    pause: bool,
}

impl ActionSheet {
    pub fn new() -> ActionSheet {
        ActionSheet {
            use_item: (Instant::now(), false),
            jump: (Instant::now(), false),
            toggle_hud: (Instant::now(), false),
            toggle_fullscreen: (Instant::now(), false),
            screenshot: (Instant::now(), false),
            sprinting: false,
            pause: false,
        }
    }

    pub fn reset(&mut self) {
        self.use_item = (Instant::now(), false);
        self.jump = (Instant::now(), false);
        self.toggle_hud = (Instant::now(), false);
        self.toggle_fullscreen = (Instant::now(), false);
        self.screenshot = (Instant::now(), false);
        self.sprinting = false;
        self.pause = false;
    }

    pub fn set_sprinting(&mut self, sprinting: bool) {
        self.sprinting = sprinting;
    }

    pub fn get_sprinting(&self) -> bool {
        self.sprinting
    }

    pub fn get_jump(&mut self) -> bool {
        if self.jump.1 {
            self.jump.1 = false;
            self.jump.0 = Instant::now();
            return true;
        }
        return false;
    }

    pub fn set_jump(&mut self) {
        if self.jump.0.elapsed().as_millis() < 250 {
            return;
        }
        self.jump.1 = true;
    }

    pub fn get_pause(&self) -> bool {
        self.pause
    }

    pub fn set_pause(&mut self) {
        self.pause = true;
    }
}

impl Default for ActionSheet {
    fn default() -> Self {
        unimplemented!()
    }
}
