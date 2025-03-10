const INPUT_MAX_VALUE: u8 = 100;
const SERVO_MAX_ANGLE: u8 = 90;

#[derive(Clone, Debug)]
pub struct InputData {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
}

pub enum MotorDir {
    Forward,
    Backward,
    Stop,
}

pub struct MotorInput {
    pub throttle: u8,
    pub dir: MotorDir,
}

pub enum ServoDir {
    Left,
    Right,
    Middle,
}

pub struct ServoInput {
    pub angle: u8,
    pub dir: ServoDir,
}

impl InputData {
    pub fn parse_and_split(self) -> (MotorInput, ServoInput) {
        let motor_throttle: u8;
        let motor_dir: MotorDir;
        let servo_angle: u8;
        let servo_dir: ServoDir;

        if self.y1 > INPUT_MAX_VALUE / 2 {
            motor_dir = MotorDir::Forward;
            motor_throttle = (self.y1 - (INPUT_MAX_VALUE / 2)) * 2;
        } else if self.y1 < INPUT_MAX_VALUE / 2 {
            motor_dir = MotorDir::Backward;
            motor_throttle = ((INPUT_MAX_VALUE / 2) - self.y1) * 2;
        } else {
            motor_dir = MotorDir::Stop;
            motor_throttle = 0;
        }

        if self.x2 > INPUT_MAX_VALUE / 2 {
            servo_dir = ServoDir::Left;
            servo_angle = (self.x2 - (INPUT_MAX_VALUE / 2)) * (SERVO_MAX_ANGLE / (INPUT_MAX_VALUE / 2));
        } else if self.x2 < INPUT_MAX_VALUE / 2 {
            servo_dir = ServoDir::Right;
            servo_angle = ((INPUT_MAX_VALUE / 2) - self.x2) * (SERVO_MAX_ANGLE / (INPUT_MAX_VALUE / 2));
        } else {
            servo_dir = ServoDir::Middle;
            servo_angle = 0;
        }

        let motor_input = MotorInput {
            throttle: motor_throttle,
            dir: motor_dir,
        };

        let servo_input = ServoInput {
            angle: servo_angle,
            dir: servo_dir,
        };
        
        /* not used atm */
        _ = (self.x1, self.y2);

        (motor_input, servo_input)
    }
}
