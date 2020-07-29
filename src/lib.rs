//! iced_plotters is an extension of iced allowing for plotte.rs to draw to the
//! canvas.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![forbid(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use iced;
pub use plotters;

use iced::canvas::{self, LineCap, LineJoin, Path, Stroke, Text};
use iced::{Color, Point, Size};

use plotters::{
    drawing::{backend::BackendStyle, DrawingBackend},
    prelude::backend::{BackendCoord, DrawingErrorKind},
    style::{Color as ColorTrait, RGBAColor, TextStyle},
};

use std::error::Error;

/// Trait that must be implemented by types that draw graphs.
pub trait Plottable: std::fmt::Debug {
    /// an example entry point.
    fn draw_plot<'a>(&self, frame: PlotFrame<'a>);
}

/// A wrapper around a canvas which can draw a plotters chart.
#[derive(Debug)]
pub struct PlotFrame<'a>(pub &'a mut canvas::Frame);

impl<'a> DrawingBackend for PlotFrame<'a> {
    type ErrorType = PlotErr;
    fn get_size(&self) -> (u32, u32) {
        (self.0.height() as u32, self.0.width() as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: &RGBAColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p = Path::rectangle(
            Point::new(point.0 as f32, point.1 as f32),
            Size::new(0.6, 0.6),
        );
        let (r, g, b) = color.rgb();
        self.0.fill(&p, Color::from_rgb8(r, g, b));
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p = Path::line(
            Point::new(from.0 as f32, from.1 as f32),
            Point::new(to.0 as f32, to.1 as f32),
        );
        let (r, g, b) = style.as_color().rgb();
        let stroke = Stroke {
            color: Color::from_rgb8(r, g, b),
            width: style.stroke_width() as f32,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
        };
        self.0.stroke(&p, stroke);
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p = Path::rectangle(
            Point::new(bottom_right.0 as f32, bottom_right.1 as f32),
            Size::new(
                (upper_left.0 - bottom_right.0) as f32,
                (upper_left.1 - bottom_right.1) as f32,
            ),
        );
        let (r, g, b) = style.as_color().rgb();
        let color = Color::from_rgb8(r, g, b);
        if fill {
            self.0.fill(&p, color);
        } else {
            let stroke = Stroke {
                color,
                width: style.stroke_width() as f32,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Miter,
            };

            self.0.stroke(&p, stroke);
        }
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let p = Path::circle(Point::new(center.0 as f32, center.1 as f32), radius as f32);
        let (r, g, b) = style.as_color().rgb();
        let color = Color::from_rgb8(r, g, b);

        if fill {
            self.0.fill(&p, color);
        } else {
            let stroke = Stroke {
                color,
                width: style.stroke_width() as f32,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Miter,
            };
            self.0.stroke(&p, stroke);
        }
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: &str,
        style: &TextStyle<'_>,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let (r, g, b) = style.color.rgb();

        self.0.fill_text(Text {
            content: text.into(),
            size: style.font.get_size() as f32,
            position: Point::new(pos.0 as f32, pos.1 as f32),
            color: Color::from_rgb8(r, g, b),
            ..Text::default()
        });
        Ok(())
    }
}

/// An Error as returned by a call to draw to plotting widget
#[derive(Debug)]
pub enum PlotErr {}

impl std::fmt::Display for PlotErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plotting Error")
    }
}

impl Error for PlotErr {}
