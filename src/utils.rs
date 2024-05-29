use iced::Point;
use crate::types::PointF;

pub fn make_display_point(point: &PointF, frame_size: f32) -> Point {
    Point::new(point.x * frame_size, point.y * frame_size)
}