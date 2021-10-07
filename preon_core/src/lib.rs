use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};

pub const SU8: usize = 1usize;
pub const SU16: usize = 2usize;
pub const SU32: usize = 4usize;
pub const SU64: usize = 8usize;
pub const SU128: usize = 16usize;
pub const SI8: usize = 1usize;
pub const SI16: usize = 2usize;
pub const SI32: usize = 4usize;
pub const SI64: usize = 8usize;
pub const SI128: usize = 16usize;
pub const SF32: usize = 4usize;
pub const SF64: usize = 8usize;
pub const SBOOL: usize = 1usize;
pub const SBOOLS: usize = 1usize;

pub const SF_HORIZONTAL_FILL: u8 = 0b00000001;
pub const SF_HORIZONTAL_EXPAND: u8 = 0b00000010;
pub const SF_VERTICAL_FILL: u8 = 0b00000100;
pub const SF_VERTICAL_EXPAND: u8 = 0b00001000;

pub const SF_VERTICAL_FILL_EXPAND: u8 = SF_VERTICAL_FILL + SF_VERTICAL_EXPAND;
pub const SF_HORIZONTAL_FILL_EXPAND: u8 = SF_HORIZONTAL_FILL + SF_HORIZONTAL_EXPAND;

pub const SF_EXPAND: u8 = SF_HORIZONTAL_EXPAND + SF_VERTICAL_EXPAND;
pub const SF_FILL: u8 = SF_HORIZONTAL_FILL + SF_VERTICAL_FILL;
pub const SF_FILL_EXPAND: u8 = SF_FILL + SF_EXPAND;

pub struct PreonCore {
    pub root: Box<PreonVertical>,
    pub layout: PreonLayout,

    pub on_resize: PreonEvent<Vector2<u32>>,

    window_inner_size: Vector2<u32>,
    _window_inner_size: Vector2<u32>,
}

impl PreonCore {
    pub fn init() -> Self {
        Self {
            root: PreonVertical::new(),
            layout: PreonLayout {
                margin: m(0),
                padding: p(0),
                min_size: vector2(0, 0),
                size_flags: SF_FILL,
            },
            on_resize: PreonEvent::new::<Vector2<u32>>(),
            window_inner_size: vector2(0, 0),
            _window_inner_size: vector2(0, 0)
        }
    }

    pub fn update(&mut self) {
        let root_layout = self.root.layout(self.layout);

        self.window_inner_size = root_layout.get_min_size();
        if self._window_inner_size != self.window_inner_size {
            self.resize(self.window_inner_size);
        }
    }

    pub fn resize(&mut self, new_size: Vector2<u32>) {
        self._window_inner_size = new_size;
    }
}

pub struct PreonEvent<U: Copy + Clone + Sized> {
    handlers: Vec<fn(U)>
}

impl<U: Copy + Clone + Sized> PreonEvent<U> {
    pub fn new<T>() -> Self {
        Self {
            handlers: Vec::new()
        }
    }

    pub fn fire(&self, args: U) {
        for handler in self.handlers.iter() {
            handler(args);
        }
    }

    pub fn subscribe(&mut self, handler: fn(U)) {
        self.handlers.push(handler);
    }
}

pub struct PreonRect {
    pub layout: PreonLayout,
    pub color: (f32, f32, f32, f32),
}

impl PreonRect {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            layout: PreonLayout {
                margin: m(0),
                padding: p(0),
                min_size: vector2(0, 0),
                size_flags: SF_FILL_EXPAND,
            },
            color: color(0xda0037ff),
        })
    }
}

impl PreonComponent for PreonRect {
    fn add_child(&mut self, _new_child: Box<dyn PreonComponent>) {
        panic!("PreonRect is not made to hold children!")
    }

    fn layout(&mut self, _parent: PreonLayout) -> PreonLayout {
        self.layout
    }
}

pub struct PreonVertical {
    pub layout: PreonLayout,
    pub children: Vec<Box<dyn PreonComponent>>,
    pub expanding_children: u32,
}

impl PreonVertical {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            layout: PreonLayout {
                margin: m(0),
                padding: p(0),
                min_size: vector2(0, 0),
                size_flags: SF_FILL,
            },
            children: Vec::new(),
            expanding_children: 0,
        })
    }
}

impl PreonComponent for PreonVertical {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>) {
        self.children.push(new_child);
    }

    fn layout(&mut self, _parent: PreonLayout) -> PreonLayout {
        self.layout.min_size = vector2(0, 0);

        for child in self.children.iter_mut() {
            let child_hints = child.layout(self.layout);
            let child_minsize = child_hints.get_min_size();

            if self.layout.has_size_flag(SF_VERTICAL_FILL) {
                if child_minsize.y > self.layout.min_size.y {
                    self.layout.min_size.y = child_minsize.y;
                }
            }
            if self.layout.has_size_flag(SF_HORIZONTAL_FILL) {
                if child_minsize.x > self.layout.min_size.x {
                    self.layout.min_size.x = child_minsize.x;
                }
            }
        }

        self.layout
    }
}

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
        vector2(
            self.margin.left + self.margin.right,
            self.margin.top + self.margin.bottom,
        ) + vector2(
            self.padding.left + self.padding.right,
            self.padding.top + self.padding.bottom,
        ) + self.min_size
    }
}

#[inline(always)]
#[cfg(target_endian = "little")]
pub fn color(c: u32) -> (f32, f32, f32, f32) {
    (
        f32::from((c >> 24) as u8) / 255f32,
        f32::from((c >> 16) as u8) / 255f32,
        f32::from((c >> 8) as u8) / 255f32,
        f32::from(c as u8) / 255f32,
    )
}

#[inline(always)]
#[cfg(target_endian = "big")]
pub fn color(c: u32) -> (f32, f32, f32, f32) {
    (
        f32::from(c as u8) / 255f32,
        f32::from((c >> 8) as u8) / 255f32,
        f32::from((c >> 16) as u8) / 255f32,
        f32::from((c >> 24) as u8) / 255f32,
    )
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    pub x: T,
    pub y: T,
}

impl<T> Add for Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Div for Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

pub fn vector2<T>(x: T, y: T) -> Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    Vector2 { x, y }
}

impl<T> Display for Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub trait PreonRenderer {
    fn start(&mut self, core: &PreonCore);
    fn update(&mut self, core: &mut PreonCore) -> bool;
    fn render(&mut self, core: &PreonCore);
}

pub trait PreonComponent {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>);
    fn layout(&mut self, parent: PreonLayout) -> PreonLayout;
}

// Used by PreonRenderers to make their own trait
pub trait PreonRenderableComponent<T: PreonRenderer> {}