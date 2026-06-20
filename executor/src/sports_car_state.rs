use crate::action::{self, Action};
use crate::assembler::Assembler;

#[derive(Default, Copy, Clone)]

pub(crate) struct SportsCarState {
    is_reverse: bool,
    is_fast: bool,
}

impl Assembler for SportsCarState {
    fn move_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let direction = if self.is_reverse { -1 } else { 1 };

        actions.push(Action::Forward(direction));
        actions.push(Action::Forward(direction));

        if self.is_fast {
            actions.push(Action::Forward(direction));
            actions.push(Action::Forward(direction));
        }

        actions
    }
    fn turn_left_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let direction = if self.is_reverse { -1 } else { 1 };

        if self.is_fast {
            actions.push(Action::Forward(direction));
        }

        actions.push(if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        });

        actions.push(Action::Forward(direction));

        actions
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let direction = if self.is_reverse { -1 } else { 1 };

        if self.is_fast {
            actions.push(Action::Forward(direction));
        }

        actions.push(if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        });

        actions.push(Action::Forward(direction));

        actions
    }

    fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    fn be_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }
}
