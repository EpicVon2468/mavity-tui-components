use ratatui_core::layout::Rect;
use ratatui_core::terminal::Frame;

use super::App;

pub const trait Component<T: App> {
	/// Pre-[`render`][`Component::render`] input/logic checks.
	///
	/// Returns whether the [`Component`] has 'consumed' the input event (and thusly, whether any parent components should be allowed to process input).
	///
	/// Implementations are expected to follow a bottom-up hierarchy of evaluation, where a [`Component`] may only process events if all its children have first been processed, and none have returned `true`.
	#[allow(unused_variables)]
	fn propagate_events(&mut self, app: &T) -> bool {
		false
	}

	fn render(&self, frame: &mut Frame, area: Rect, app: &T);
}
