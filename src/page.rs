use super::{App, Component};

pub type NewPage<T> = Option<BoxPage<T>>;

pub type BoxPage<T> = Box<dyn Page<T>>;

pub const trait Page<T: App>: Component<T> {
	fn title(&self) -> &str;

	fn propagate_page_events(&mut self, app: &mut T) -> (bool, NewPage<T>);
}
