use cgmath::Point2;
use rgb;

pub use crate::shape::Shape;
pub use crate::style::Style;

pub mod canvas;
pub mod render;
pub mod shape;
pub mod style;

/// Drawings are stored in a vector; this `usize` is a handle to access the child
pub type DrawingId = usize;

pub type Position = Point2<f32>;
pub type RGB = rgb::RGB<u8>;

pub struct Drawing {
    pub shape: Shape,
    pub display_list: DisplayList,
    pub position: Point2<f32>,
    pub style: Style,
}

impl Drawing {
    pub fn new(shape: Shape) -> Drawing {
        Drawing {
            shape,
            style: Style::default(),
            display_list: DisplayList::new(),
            position: Position::new(0.0, 0.0),
        }
    }
}

pub struct DisplayList {
    drawings: Vec<Drawing>,
}

impl DisplayList {
    fn new() -> DisplayList {
        DisplayList { drawings: vec![] }
    }

    /// Adds a drawing to the top of the display list.
    /// Returns a DrawingId handle that can be used to refer to the drawing in the future.
    pub fn add(&mut self, drawing: Drawing) -> DrawingId {
        let child_id = self.drawings.len();
        self.drawings.push(drawing);
        child_id
    }

    pub fn remove(&mut self, _drawing_id: DrawingId) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::canvas::Canvas;
    use crate::render::svg::SvgRenderer;
    use crate::style::{Fill, Stroke};

    #[test]
    fn basic_end_to_end() {
        // create a canvas to draw on
        let mut canvas = Canvas::new(
            100,
            100,
            Some(Shape::Rectangle {
                width: 100,
                height: 100,
            }),
        );

        // create some drawings of rectangles
        let a = Drawing::new(Shape::Rectangle {
            width: 50,
            height: 50,
        });

        let mut b = Drawing::new(Shape::Rectangle {
            width: 10,
            height: 10,
        });

        // move one around
        b.position.x = 25.0;
        b.position.y = 25.0;

        // give one a cool style
        b.style = Style {
            fill: Some(Fill {
                color: RGB::new(255, 0, 0),
            }),
            stroke: Some(Stroke {
                width: 2,
                color: RGB::new(0, 255, 0),
            }),
        };

        // add those drawings to the canvas
        canvas.display_list.add(a);
        canvas.display_list.add(b);

        // save the canvas as an svg
        render::save(
            &canvas,
            "tests/svg/basic_end_to_end.svg",
            SvgRenderer::new(),
        )
        .expect("Failed to save");
    }
}
