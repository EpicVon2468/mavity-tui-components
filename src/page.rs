use anyhow::Result;

use super::{App, Component};

type NewPage<T> = Option<Box<dyn Page<T>>>;

pub const trait Page<T: App>: Component<T> {
	fn title(&self) -> &str;

	fn propagate_page_events(&mut self, app: &T) -> Result<(bool, NewPage<T>)>;
}
