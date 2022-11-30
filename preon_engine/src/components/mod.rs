use core::fmt::Debug;
use alloc::{vec::Vec, string::{String, ToString}, vec, borrow::ToOwned};

use crate::{
    rendering::{PreonRenderPass, PreonShape},
    types::{PreonAlignment, PreonVector, PreonColor, PreonRect},
    style::{PreonStyle, PreonBackground},
    layout::{
        PreonLayout,
        rows::PreonRowsLayoutProvider,
        columns::PreonColumnsLayoutProvider,
        container::PreonContainerLayoutProvider
    },
};

pub mod hbox;
pub mod vbox;
pub mod panel;
pub mod label;
pub mod static_texture;

/// A UI component
#[derive(Debug, Clone)]
pub struct PreonComponent {
    pub children: Vec<PreonComponent>,
    pub style: PreonStyle,
    pub text: String,
    pub inner_size: PreonVector<i32>,
    pub inner_position: PreonVector<i32>,
    pub index_updates: Vec<isize>,
    pub id: Option<String>,
    pub id_lookup_cache: Vec<(String, Vec<u16>)>,
    pub hoverable: bool,
}

impl PreonComponent {
    pub fn new() -> PreonComponent {
        PreonComponent {
            children: Vec::new(),
            style: PreonStyle::default(),
            text: String::new(),
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            index_updates: Vec::new(),
            id: None,
            id_lookup_cache: Vec::new(),
            hoverable: false,
        }
    }

    pub fn get_hovered_child(&mut self, position: PreonVector<i32>) -> Option<&mut PreonComponent> {
        if !self.get_border_rect().contains_point(position) {
            None
        } else {
            if self.hoverable {
                Some(self)
            } else {
                self.children
                    .iter_mut()
                    .filter_map(|child| child.get_hovered_child(position))
                    .next()
            }
        }
    }

    fn find_child_by_id_recursive(&self, id: &String) -> Option<Vec<u16>> {
        for (i, child) in self.children.iter().enumerate() {
            if child.id.as_ref() == Some(id) {
                return Some(vec![i as u16])
            }
            
            if let Some(path) = child.find_child_by_id_recursive(id) {
                let mut full_path = vec![i as u16];
                full_path.extend(path);

                return Some(full_path)
            }
        }

        None
    }

    fn find_child_by_id(&self, id: &String) -> Option<Vec<u16>> {
        self
            .find_child_by_id_recursive(id)
            .map(|mut path| { path.reverse(); path })
            .and_then(|path| self.get_child_ref_recursive(path.as_slice()).map(|_| path))
    }

    fn get_child_path_by_id_cached(&self, id: &String) -> Option<Vec<u16>> {
        self
            .id_lookup_cache
            .iter()
            .rev()
            .find_map(|item| if &item.0 == id { Some(item.1.clone()) } else { None })
    }

    pub fn get_child_ref_by_id(&mut self, id: impl ToString) -> Option<&PreonComponent> {
        let id = &id.to_string();
        
        let path = self
            .get_child_path_by_id_cached(id)
            .or_else(|| self.find_child_by_id(id));
        
        if let Some(path) = path {
            self.get_child_ref_recursive(&path[..])
        } else {
            if let Some(path) = self.find_child_by_id(id) {
                self.id_lookup_cache.push((id.clone(), path.clone()));
                self.get_child_ref_recursive(&path[..])
            } else {
                log::warn!("No component found with id {}", id);
                None
            }
        }
    }

    pub fn get_child_ref_mut_by_id(&mut self, id: impl ToString) -> Option<&mut PreonComponent> {
        let id = &id.to_string();

        let path = self
            .get_child_path_by_id_cached(id)
            .or_else(|| self.find_child_by_id(id));
        
        if let Some(path) = path {
            self.get_child_ref_mut_recursive(&path[..])
        } else {
            if let Some(path) = self.find_child_by_id(id) {
                self.id_lookup_cache.push((id.clone(), path.clone()));
                self.get_child_ref_mut_recursive(&path[..])
            } else {
                log::warn!("No component found with id {}", id);
                None
            }
        }
    }

    pub fn get_child_ref_recursive(&self, path: &[u16]) -> Option<&PreonComponent> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref(index)
        } else {
            self.get_child_ref(index)
                .and_then(|child| child.get_child_ref_recursive(&_path))
        }
    }

    pub fn get_child_ref_mut_recursive(&mut self, path: &[u16]) -> Option<&mut PreonComponent> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref_mut(index)
        } else {
            self.get_child_ref_mut(index)
                .and_then(|child| child.get_child_ref_mut_recursive(&_path))
        }
    }

    pub unsafe fn get_child_raw_by_id(&mut self, id: impl ToString) -> Option<*mut PreonComponent> {
        let id = &id.to_string();

        let path = self
            .get_child_path_by_id_cached(id)
            .or_else(|| self.find_child_by_id(id));
        
        if let Some(path) = path {
            self.get_child_raw_recursive(&path[..])
        } else {
            if let Some(path) = self.find_child_by_id(id) {
                self.id_lookup_cache.push((id.clone(), path.clone()));
                self.get_child_raw_recursive(&path[..])
            } else {
                log::warn!("No component found with id {}", id);
                None
            }
        }
    }

    pub unsafe fn get_child_raw_recursive(&mut self, path: &[u16]) -> Option<*mut PreonComponent> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_raw(index)
        } else {
            self.get_child_ref_mut(index)
                .and_then(|child| child.get_child_raw_recursive(&_path))
        }
    }

    pub unsafe fn get_child_raw(&mut self, idx: u16) -> Option<*mut PreonComponent> {
        self.children
            .get_mut(idx as usize)
            .map(|child| child as *mut PreonComponent)
    }

    pub fn get_child_ref(&self, idx: u16) -> Option<&PreonComponent> {
        self.children
            .get(idx as usize)
    }

    pub fn get_child_ref_mut(&mut self, idx: u16) -> Option<&mut PreonComponent> {
        self.children
            .get_mut(idx as usize)
    }

    pub fn add_child(&mut self, child: PreonComponent) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, idx: u16, child: PreonComponent) {
        self.children.insert(idx as usize, child);
    }

    pub fn remove_child(&mut self, idx: u16) {
        self.children.remove(idx as usize);
    }

    #[inline(always)]
    pub fn set_content_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position - self.style.padding.top_left();
    }

    #[inline(always)]
    pub fn get_content_position(&self) -> PreonVector<i32> {
        self.inner_position + self.style.padding.top_left()
    }

    #[inline(always)]
    pub fn get_content_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_content_position(), self.get_content_size())
    }

    #[inline(always)]
    pub fn set_inner_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position;
    }

    #[inline(always)]
    pub fn get_inner_position(&self) -> PreonVector<i32> {
        self.inner_position
    }

    #[inline(always)]
    pub fn get_inner_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_inner_position(), self.get_inner_size())
    }

    #[inline(always)]
    pub fn set_border_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position + self.style.border.top_left();
    }

    #[inline(always)]
    pub fn get_border_position(&self) -> PreonVector<i32> {
        self.inner_position - self.style.border.top_left()
    }

    #[inline(always)]
    pub fn get_border_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_border_position(), self.get_border_size())
    }

    #[inline(always)]
    pub fn set_outer_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position =
            new_position + self.style.border.top_left() + self.style.margin.top_left();
    }

    #[inline(always)]
    pub fn get_outer_position(&self) -> PreonVector<i32> {
        self.inner_position - self.style.border.top_left() - self.style.margin.top_left()
    }

    #[inline(always)]
    pub fn get_outer_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_outer_position(), self.get_outer_size())
    }

    #[inline(always)]
    pub fn set_content_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size + self.style.padding;
    }

    #[inline(always)]
    pub fn get_content_size(&self) -> PreonVector<i32> {
        self.get_inner_size() - self.style.padding
    }

    #[inline(always)]
    pub fn set_inner_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size;
    }

    #[inline(always)]
    pub fn get_inner_size(&self) -> PreonVector<i32> {
        PreonVector::new(
            self.inner_size.x.max(self.style.min_size.x),
            self.inner_size.y.max(self.style.min_size.y),
        )
    }

    #[inline(always)]
    pub fn set_border_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.style.border
    }

    #[inline(always)]
    pub fn get_border_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.style.border
    }

    #[inline(always)]
    pub fn set_outer_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.style.margin - self.style.border;
    }

    #[inline(always)]
    pub fn get_outer_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.style.border + self.style.margin
    }

    #[inline(always)]
    pub fn set_content_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x + self.style.padding.x());
    }

    #[inline(always)]
    pub fn set_inner_size_x(&mut self, new_x: i32) {
        self.inner_size.x = new_x;
    }

    #[inline(always)]
    pub fn set_border_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x - self.style.margin.x());
    }

    #[inline(always)]
    pub fn set_outer_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x - self.style.margin.x() - self.style.border.x());
    }

    #[inline(always)]
    pub fn set_content_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y + self.style.padding.y());
    }

    #[inline(always)]
    pub fn set_inner_size_y(&mut self, new_y: i32) {
        self.inner_size.y = new_y;
    }

    #[inline(always)]
    pub fn set_border_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y - self.style.margin.y());
    }

    #[inline(always)]
    pub fn set_outer_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y - self.style.margin.y() - self.style.border.y());
    }

    pub(crate) fn layout(&mut self) {
        use crate::layout::PreonLayoutProvider;

        self.index_updates.clear();

        match self.style.layout {
            PreonLayout::Rows => PreonRowsLayoutProvider::layout(self),
            PreonLayout::Columns => PreonColumnsLayoutProvider::layout(self),
            PreonLayout::Container => PreonContainerLayoutProvider::layout(self),
        }

        for child in self.children.iter_mut() {
            child.layout();
        }
    }

    pub(crate) fn render(&mut self, pass: &mut PreonRenderPass) {
        let stages = [
            PreonComponentRenderStage::Background {
                position: self.get_inner_position(),
                size: self.get_inner_size(),
            },
            PreonComponentRenderStage::Foreground {
                position: self.get_content_position(),
                size: self.get_content_size(),
            },
        ];

        for stage in stages {
            match stage {
                PreonComponentRenderStage::Background { position, size } => match self.style.background {
                    PreonBackground::Color(color) => pass.push(PreonShape::Rect {
                        position,
                        size,
                        color,
                        index: None,
                        radius: self.style.corner_radius,
                    }),
                    PreonBackground::Image(ref image) => pass.push(PreonShape::Rect {
                        position,
                        size,
                        color: PreonColor::TRANSPARENT_BLACK,
                        index: Some(image.index()),
                        radius: self.style.corner_radius,
                    }),
                    _ => (),
                },
                PreonComponentRenderStage::Foreground { position, size } => if !self.text.is_empty() {
                    pass.push(PreonShape::Text {
                        text_style: self.style.text_style.clone(),
                        color: self.style.foreground_color,
                        position,
                        size,
                        text: self.text.clone(),
                    })
                },
            }
        }

        #[cfg(feature = "debug")]
        {
            pass.push(PreonShape::Rect {
                position: component.get_outer_position(),
                size: component.get_outer_size(),
                color: PreonColor::from_hex("#c0687055"),
            });

            pass.push(PreonShape::Rect {
                position: component.get_border_position(),
                size: component.get_border_size(),
                color: PreonColor::from_hex("#c09b6855"),
            });

            pass.push(PreonShape::Rect {
                position: component.get_inner_position(),
                size: component.get_inner_size(),
                color: PreonColor::from_hex("#68c09355"),
            });

            pass.push(PreonShape::Rect {
                position: component.get_content_position(),
                size: component.get_content_size(),
                color: PreonColor::from_hex("#6891c055"),
            });
        }

        for child in self.children.iter_mut() {
            Self::render(child, pass)
        }
    }
}

impl Default for PreonComponent {
    fn default() -> Self {
        PreonComponent {
            style: PreonStyle {
                align_items: PreonAlignment::Start,
                cross_align_items: PreonAlignment::Center,
                ..Default::default()
            },
            children: Vec::new(),
            text: String::new(),
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            index_updates: Vec::new(),
            id: None,
            id_lookup_cache: Vec::new(),
            hoverable: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PreonComponentRenderStage {
    Background {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
    },
    Foreground {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
    },
}

pub struct PreonComponentBuilder {
    pub stack: Vec<PreonComponent>,
}

#[allow(clippy::new_without_default)]
impl PreonComponentBuilder {
    pub fn new() -> PreonComponentBuilder {
        Self {
            stack: vec![PreonComponent {
                style: PreonStyle {
                    align_items: PreonAlignment::Start,
                    cross_align_items: PreonAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            }],
        }
    }

    pub fn from_component(component: PreonComponent) -> PreonComponentBuilder {
        Self {
            stack: vec![component],
        }
    }

    pub fn with_child(&mut self, child: PreonComponent) -> &mut PreonComponentBuilder {
        self.current_mut().children.push(child);        
        self
    }

    pub fn store_index(&mut self, reference: &mut usize) -> &mut PreonComponentBuilder {
        *reference = self.get_index();

        self
    }

    pub fn store_path(&mut self, reference: &mut Vec<usize>) -> &mut PreonComponentBuilder {
        reference.clear();
        reference.truncate(self.stack.len());
        reference.shrink_to_fit();

        let mut _stack: Vec<PreonComponent> = Vec::with_capacity(self.stack.capacity());

        for _ in 0..self.stack.len() {
            if self.stack.len() >= 2 {
                reference.push(self.get_index());
            }
            _stack.push(self.stack.pop().take().unwrap());
        }

        for _ in 0.._stack.len() {
            self.stack.push(_stack.pop().take().unwrap());
        }

        self
    }

    fn get_index(&mut self) -> usize {
        if self.stack.len() == 1 {
            return 0;
        }

        self.stack
            .get(self.stack.len() - 2)
            .unwrap()
            .children
            .len()
    }

    pub fn inherited_style(&self) -> PreonStyle {
        PreonStyle::inherit_from(&self.current().style)
    }

    pub fn hoverable(&mut self) -> &mut PreonComponentBuilder  {
        self.stack.last_mut().unwrap().hoverable = true;
        self
    }

    pub fn override_hoverable(&mut self, hoverable: bool) -> &mut PreonComponentBuilder  {
        self.stack.last_mut().unwrap().hoverable = hoverable;
        self
    }

    pub fn id(&mut self, id: &str) -> &mut PreonComponentBuilder {
        self.stack.last_mut().unwrap().id = Some(id.to_string());
        self
    }

    pub fn id_string(&mut self, id: String) -> &mut PreonComponentBuilder {
        self.stack.last_mut().unwrap().id = Some(id);
        self
    }

    pub fn end(&mut self) -> &mut PreonComponentBuilder {
        let child = self.stack.pop().unwrap();
        self.with_child(child);
        self
    }

    pub fn build(&mut self) -> PreonComponent {
        self.stack.pop().unwrap()
    }

    pub fn current(&self) -> &PreonComponent {
        self.stack.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut PreonComponent {
        self.stack.last_mut().unwrap()
    }
}