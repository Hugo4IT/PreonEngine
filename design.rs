use preon::prelude::*;

mod button {
    use preon::prelude::*;

    pub enum Variant {
        Primary,
        Secondary,
        Normal,
    }

    impl Default for Variant {
        fn default() -> Self {
            Self::Normal
        }
    }

    #[derive(PreonComponent)]
    struct Button {
        variant: Variant,
        pressed: bool,
    }
    
    impl Events for Button {
        // #[subscribe] on mouse_down adds this:
        //      fn subscribe_mouse_down(&self) -> bool { true }
    
        // Mouse down on the Button
        #[subscribe]
        fn mouse_down(&mut self, event: Event<MouseDown>) {
            self.pressed = true;
        }
        
        // Global MouseUp event handler
        #[subscribe]
        fn window_mouse_up(&mut self, event: Event<MouseUp>) {
            self.pressed = false;
        }
    }
    
    impl Layout for Button {
        fn layout(&mut self, children: &mut Array<Rect>) {
            
        }
    }
    
    impl View for Button {
        
    }
    
    impl 
    
    impl AdvancedView for Button {
        fn advanced_view(&self, rect: Rect, context: WgpuContext) {
            // Draw stuff with Wgpu
        }
    }
}

fn main() {
    
}
