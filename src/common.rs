// common
use crate::xml::Tag;
use std::fmt::Display;

pub trait Chart {
    /// get x and y position
    fn get_pos(&self) -> (u32, u32);

    /// set x and y position
    fn set_pos(&mut self, x: u32, y: u32);

    /// get width and height
    fn get_size(&self) -> (u32, u32);

    /// set the with and height
    fn set_size(&mut self, width: u32, height: u32);

    /// the builder
    fn build_trait(self) -> Tag;
}

pub trait Data {
    /// get the labels
    fn get_labels(&self) -> Vec<impl Display>;
    /// get the colors
    fn get_colors(&self) -> Vec<impl Display>;
}

pub trait Values {
    /// get the values
    fn get_values(&self) -> Vec<u32>;
}
