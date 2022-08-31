#[derive(Debug)]
pub enum Action<'i> {
    None,
    CreateController,
    DirectInput(&'i DirectInput),
}

impl<'i> Action<'i> {
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0; 32];
        match self {
            Action::None => bytes,
            Action::CreateController => {
                bytes[0] = 1;
                bytes
            },
            Action::DirectInput(input) => {
                bytes[0] = 2;
                bytes[1..21].copy_from_slice(&input.to_bytes());
                bytes
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct DirectInput {
    // buttons
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,

    // dpad
    pub dpad_up: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub dpad_down: bool,

    // triggers and shoulder buttons
    pub l: bool,
    pub zl: bool,
    pub r: bool,
    pub zr: bool,

    // meta buttons
    pub plus: bool,
    pub minus: bool,
    pub home: bool,
    pub capture: bool,

    // sticks
    pub l_stick: StickDirectInput,
    pub r_stick: StickDirectInput,
}

impl DirectInput {
    pub fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0; 20];
        // store in the first byte the a, b, x, y and dpad states
        bytes[0] = (self.a as u8) << 7
            | (self.b as u8) << 6
            | (self.x as u8) << 5
            | (self.y as u8) << 4
            | (self.dpad_up as u8) << 3
            | (self.dpad_left as u8) << 2
            | (self.dpad_right as u8) << 1
            | (self.dpad_down as u8);

        bytes[1] = (self.l as u8) << 7
            | (self.zl as u8) << 6
            | (self.r as u8) << 5
            | (self.zr as u8) << 4
            | (self.plus as u8) << 3
            | (self.minus as u8) << 2
            | (self.home as u8) << 1
            | (self.capture as u8);

        bytes[2..11].copy_from_slice(&self.l_stick.to_bytes());
        bytes[11..20].copy_from_slice(&self.r_stick.to_bytes());

        bytes
    }
}

#[derive(Default, Debug)]
pub struct StickDirectInput {
    pub pressed: bool,
    pub x: f32,
    pub y: f32,
}

impl StickDirectInput {
    pub fn to_bytes(&self) -> [u8; 9] {
        let mut bytes = [0; 9];
        bytes[0] = self.pressed as u8;
        bytes[1..5].copy_from_slice(&self.x.to_be_bytes());
        bytes[5..9].copy_from_slice(&self.y.to_be_bytes());
        bytes
    }
}
