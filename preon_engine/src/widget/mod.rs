use crate::canvas::{SubCanvas, Canvas};

pub trait Widget {
    fn view<C: Canvas>(canvas: SubCanvas<C>);
}
