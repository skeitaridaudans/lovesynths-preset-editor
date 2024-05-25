use iced::Point;
use crate::types::PointF;

pub fn make_display_point(point: &PointF) -> Point {
    const SCALE: f32 = 40.0;
    Point::new(point.x * SCALE, point.y * SCALE)
}