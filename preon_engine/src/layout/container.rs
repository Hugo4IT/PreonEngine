use crate::components::PreonComponent;

use super::PreonLayoutProvider;

pub(crate) struct PreonContainerLayoutProvider;

impl PreonLayoutProvider for PreonContainerLayoutProvider {
    fn layout(component: &mut PreonComponent) {
        let position = component.get_content_position();
        let size = component.get_content_size();
        for child in component.children.iter_mut() {
            child.set_outer_position(position);
            child.set_outer_size(size);
        }
    }
}