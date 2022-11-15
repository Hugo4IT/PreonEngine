#[derive(Debug, Clone, Copy)]
pub struct PreonImage {
    index: usize,
}

impl PreonImage {
    pub fn from_static(index: usize) -> PreonImage {
        // TODO: Reimplement images
        PreonImage { index }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}