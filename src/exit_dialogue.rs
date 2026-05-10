use crossterm::event::KeyCode;

use ratatui_core::layout::{Constraint, Layout, Margin, Offset, Rect};
use ratatui_core::style::Style;
use ratatui_core::terminal::Frame;
use ratatui_core::text::Span;
use ratatui_widgets::block::Block;
use ratatui_widgets::clear::Clear;

use unicode_width::UnicodeWidthStr as _;

use super::{App, Component, INVERTED, centred_horizontally, static_layout, to_u16};

pub struct ExitDialogue {
	state: DialogueState,
	// closest to 'u1' type (only values are 0 or 1)
	selected: bool,
	text: String,
	confirm_text: String,
	deny_text: String,
}

impl Default for ExitDialogue {
	fn default() -> Self {
		Self {
			state: Default::default(),
			selected: false,
			text: "Go back?".into(),
			confirm_text: "[PROCEED]".into(),
			deny_text: "[CANCEL]".into(),
		}
	}
}

static_layout!(Layout::horizontal([
	Constraint::Fill(1),
	Constraint::Fill(1),
]));
static_layout!(
	LAYOUT_VERT,
	Layout::vertical([Constraint::Percentage(75), Constraint::Fill(1)]),
);

#[derive_const(PartialEq, Eq)]
enum DialogueState {
	Hidden(bool),
	Shown,
}

impl const Default for DialogueState {
	fn default() -> Self {
		Self::Hidden(false)
	}
}

impl ExitDialogue {
	pub fn text<T: Into<String>>(&mut self, text: T) -> &mut Self {
		self.text = text.into();
		self
	}

	pub fn confirm_text<T: Into<String>>(&mut self, confirm_text: T) -> &mut Self {
		self.confirm_text = confirm_text.into();
		self
	}

	pub fn deny_text<T: Into<String>>(&mut self, deny_text: T) -> &mut Self {
		self.deny_text = deny_text.into();
		self
	}

	pub const fn show(&mut self) -> &mut Self {
		self.state = DialogueState::Shown;
		self
	}

	#[must_use]
	pub const fn is_shown(&self) -> bool {
		self.state == DialogueState::Shown
	}

	#[must_use]
	pub const fn should_exit(&self) -> bool {
		self.state == DialogueState::Hidden(true)
	}
}

impl ExitDialogue {
	fn render_selection_raw(&self, frame: &mut Frame, area: Rect) {
		let [confirm, deny] = area.layout(&LAYOUT);
		self.render_confirm(frame, confirm);
		self.render_deny(frame, deny);
	}

	fn render_confirm(&self, frame: &mut Frame, area: Rect) {
		frame.render_widget(
			Span::styled(
				&self.confirm_text,
				if self.selected {
					Style::reset()
				} else {
					INVERTED
				},
			),
			centred_horizontally!(area, self.confirm_text.width()),
		);
	}

	fn render_deny(&self, frame: &mut Frame, area: Rect) {
		frame.render_widget(
			Span::styled(
				&self.deny_text,
				if self.selected {
					INVERTED
				} else {
					Style::reset()
				},
			),
			centred_horizontally!(area, self.deny_text.width()),
		);
	}
}

impl<T: App> Component<T> for ExitDialogue {
	fn propagate_events(&mut self, app: &T) -> bool {
		if self.state != DialogueState::Shown {
			return false;
		};

		if app.should_shl() || app.should_shr() {
			self.selected = !self.selected;
			return true;
		};
		if app.is_key_down(KeyCode::Enter) {
			self.state = DialogueState::Hidden(self.selected);
			return true;
		};
		// ExitDialogue consumes all input events as it is the primary focus Component if it is shown
		true
	}

	fn render(&self, frame: &mut Frame, area: Rect, _app: &T) {
		if self.state != DialogueState::Shown {
			return;
		};

		let width: u16 = to_u16!(self.text.width());
		let area: Rect = centred_horizontally!(area, width)
			.centered_vertically(Constraint::Length(3))
			.outer(Margin::new(8, 1))
			.offset(Offset::new(-4, 0));

		frame.render_widget(Clear, area);

		let block: Block = Block::bordered().style(INVERTED);
		frame.render_widget(&block, area);
		let area: Rect = block.inner(area);

		let [top, bottom] = area.layout(&LAYOUT_VERT);
		frame.render_widget(&*self.text, centred_horizontally!(top, width));
		self.render_selection_raw(frame, bottom);
	}
}
