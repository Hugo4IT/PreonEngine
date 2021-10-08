use super::types::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct PreonLayout {
    pub margin: PreonMargin,
    pub padding: PreonPadding,
    pub min_size: Vector2<u32>,
    pub size_flags: u8,
}

impl PreonLayout {
    #[inline(always)]
    pub fn has_size_flag(&self, sf: u8) -> bool {
        (self.size_flags & sf) == sf
    }

    #[inline(always)]
    pub fn get_min_size(&self) -> Vector2<u32> {
        Vector2 {
            x: self.margin.left
                + self.margin.right
                + self.padding.left
                + self.padding.right
                + self.min_size.x,
            y: self.margin.top
                + self.margin.bottom
                + self.padding.top
                + self.padding.bottom
                + self.min_size.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PreonMargin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

/// Returns a PreonMargin with all sides set to `v`.
#[inline(always)]
pub fn margin(v: u32) -> PreonMargin {
    PreonMargin {
        top: v,
        right: v,
        bottom: v,
        left: v,
    }
}

/// Returns a PreonMargin with the left and right sides set to `x` and top and bottom set to 0.
#[inline(always)]
pub fn margin_x(x: u32) -> PreonMargin {
    PreonMargin {
        top: 0,
        right: x,
        bottom: 0,
        left: x,
    }
}

/// Returns a PreonMargin with the top and bottom set to `x` and the right and left set to 0.
#[inline(always)]
pub fn margin_y(y: u32) -> PreonMargin {
    PreonMargin {
        top: y,
        right: 0,
        bottom: y,
        left: 0,
    }
}

/// Returns a PreonMargin with the left and right set to `x` and the top and bottom set to `y`
#[inline(always)]
pub fn margin_xy(x: u32, y: u32) -> PreonMargin {
    PreonMargin {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

/// Arguments in order: Top, Right, Bottom, Left
#[inline(always)]
pub fn margin_trbl(t: u32, r: u32, b: u32, l: u32) -> PreonMargin {
    PreonMargin {
        top: t,
        right: r,
        bottom: b,
        left: l,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PreonPadding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

/// Returns a PreonPadding with all sides set to `v`.
#[inline(always)]
pub fn padding(v: u32) -> PreonPadding {
    PreonPadding {
        top: v,
        right: v,
        bottom: v,
        left: v,
    }
}

/// Returns a PreonPadding with the left and right sides set to `x` and top and bottom set to 0.
#[inline(always)]
pub fn padding_x(x: u32) -> PreonPadding {
    PreonPadding {
        top: 0,
        right: x,
        bottom: 0,
        left: x,
    }
}

/// Returns a PreonPadding with the top and bottom set to `x` and the right and left set to 0.
#[inline(always)]
pub fn padding_y(y: u32) -> PreonPadding {
    PreonPadding {
        top: y,
        right: 0,
        bottom: y,
        left: 0,
    }
}

/// Returns a PreonPadding with the left and right set to `x` and the top and bottom set to `y`
#[inline(always)]
pub fn padding_xy(x: u32, y: u32) -> PreonPadding {
    PreonPadding {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

/// Arguments in order: Top, Right, Bottom, Left
#[inline(always)]
pub fn padding_trbl(t: u32, r: u32, b: u32, l: u32) -> PreonPadding {
    PreonPadding {
        top: t,
        right: r,
        bottom: b,
        left: l,
    }
}