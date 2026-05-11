use crossterm::event::KeyCode;

pub const trait App {
	type Event;

	fn is_key_down(&self, key: KeyCode) -> bool;

	fn was_key_down(&self, key: KeyCode) -> bool;

	fn should_exit(&self) -> bool {
		self.was_key_down(KeyCode::Char(':')) && self.is_key_down(KeyCode::Char('q'))
	}

	fn should_shl(&self) -> bool {
		self.is_key_down(KeyCode::Left) || self.is_key_down(KeyCode::BackTab)
	}

	fn should_shr(&self) -> bool {
		self.is_key_down(KeyCode::Right) || self.is_key_down(KeyCode::Tab)
	}

	fn submit(&mut self, event: Self::Event, closure: fn()) -> bool;
}
