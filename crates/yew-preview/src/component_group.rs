use crate::component_item::ComponentItem;

#[derive(Clone, PartialEq)]
pub struct ComponentGroup {
    pub name: String,
    pub components: Vec<ComponentItem>,
}
