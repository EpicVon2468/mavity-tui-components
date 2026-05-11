use crossterm::event::KeyCode;

pub const trait App {
	type Event;

	fn is_key_down(&self, key: KeyCode) -> bool;

	fn was_key_down(&self, key: KeyCode) -> bool;

	fn should_exit(&self) -> bool;

	fn should_shl(&self) -> bool;

	fn should_shr(&self) -> bool;

	fn submit(&mut self, event: Self::Event, closure: fn()) -> bool;
}
