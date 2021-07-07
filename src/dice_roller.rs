use crate::dice_roller::Mode::ConfigurationMode;
use crate::mini_rng::Rng;

#[derive(Eq, PartialEq)]
pub enum Mode {
    NormalMode,
    ConfigurationMode,
}

pub struct Dice {
    dice: [u8; 7],
    index: usize,
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            dice: [4, 6, 8, 10, 12, 20, 100],
            index: 0,
        }
    }

    fn cycle(&mut self) {
        if self.index < 6 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    fn get(&self) -> u8 {
        self.dice[self.index]
    }
}

pub struct DiceRoller {
    pub mode: Mode,
    throws: u8,
    dice: Dice,
    roll: u16,
    pub rolling: bool,
}

impl DiceRoller {
    pub fn new() -> DiceRoller {
        DiceRoller {
            mode: ConfigurationMode,
            throws: 1,
            dice: Dice::new(),
            roll: 0,
            rolling: false,
        }
    }

    pub fn change_dice_amount(&mut self) {
        if self.throws < 9 {
            self.throws += 1;
        } else {
            self.throws = 1;
        }
    }

    pub fn get_dice_amount(&self) -> u8 {
        self.throws
    }

    pub fn change_dice(&mut self) {
        self.dice.cycle();
    }

    pub fn get_dice(&self) -> u8 {
        self.dice.get()
    }

    pub(crate) fn roll(&mut self, mut rng: &mut Rng) {
        self.roll = 0;
        for _ in 0..self.throws {
            self.roll += (rng.rand_range(self.get_dice()) + 1) as u16;
        }
    }

    pub fn get_roll(&self) -> u16 {
        self.roll
    }
}
