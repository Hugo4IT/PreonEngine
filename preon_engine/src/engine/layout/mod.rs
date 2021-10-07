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

#[inline(always)]
pub fn m4(t: u32, r: u32, b: u32, l: u32) -> PreonMargin {
    PreonMargin {
        top: t,
        right: r,
        bottom: b,
        left: l,
    }
}

#[inline(always)]
pub fn m2(x: u32, y: u32) -> PreonMargin {
    PreonMargin {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

#[inline(always)]
pub fn mx(x: u32) -> PreonMargin {
    PreonMargin {
        top: 0,
        right: x,
        bottom: 0,
        left: x,
    }
}

#[inline(always)]
pub fn my(y: u32) -> PreonMargin {
    PreonMargin {
        top: y,
        right: 0,
        bottom: y,
        left: 0,
    }
}

#[inline(always)]
pub fn m(v: u32) -> PreonMargin {
    PreonMargin {
        top: v,
        right: v,
        bottom: v,
        left: v,
    }
}

#[inline(always)]
pub fn margin(v: u32) -> PreonMargin {
    m(v)
}
#[inline(always)]
pub fn margin_x(x: u32) -> PreonMargin {
    mx(x)
}
#[inline(always)]
pub fn margin_y(y: u32) -> PreonMargin {
    my(y)
}
#[inline(always)]
pub fn margin_xy(x: u32, y: u32) -> PreonMargin {
    m2(x, y)
}
#[inline(always)]
pub fn margin_trbl(t: u32, r: u32, b: u32, l: u32) -> PreonMargin {
    m4(t, r, b, l)
}

#[derive(Clone, Copy, Debug)]
pub struct PreonPadding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

#[inline(always)]
pub fn p4(t: u32, r: u32, b: u32, l: u32) -> PreonPadding {
    PreonPadding {
        top: t,
        right: r,
        bottom: b,
        left: l,
    }
}

#[inline(always)]
pub fn p2(x: u32, y: u32) -> PreonPadding {
    PreonPadding {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

#[inline(always)]
pub fn px(x: u32) -> PreonPadding {
    PreonPadding {
        top: 0,
        right: x,
        bottom: 0,
        left: x,
    }
}

#[inline(always)]
pub fn py(y: u32) -> PreonPadding {
    PreonPadding {
        top: y,
        right: 0,
        bottom: y,
        left: 0,
    }
}

#[inline(always)]
pub fn p(v: u32) -> PreonPadding {
    PreonPadding {
        top: v,
        right: v,
        bottom: v,
        left: v,
    }
}

#[inline(always)]
pub fn padding(v: u32) -> PreonPadding {
    p(v)
}
#[inline(always)]
pub fn padding_x(x: u32) -> PreonPadding {
    px(x)
}
#[inline(always)]
pub fn padding_y(y: u32) -> PreonPadding {
    py(y)
}
#[inline(always)]
pub fn padding_xy(x: u32, y: u32) -> PreonPadding {
    p2(x, y)
}
#[inline(always)]
pub fn padding_trbl(t: u32, r: u32, b: u32, l: u32) -> PreonPadding {
    p4(t, r, b, l)
}
