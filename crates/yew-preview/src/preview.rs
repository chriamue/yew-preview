use crate::component_item::ComponentItem;

pub trait Preview {
    fn preview() -> ComponentItem;
}
