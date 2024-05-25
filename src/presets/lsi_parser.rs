use crate::types::{ImageData, PointF};
use nom::combinator::all_consuming;
use nom::multi::{many0, many_m_n};
use nom::number::complete::{le_f32, le_i32};
use nom::sequence::tuple;
use nom::IResult;

fn parse_point_sequence(input: &[u8]) -> IResult<&[u8], Vec<PointF>> {
    let (inp, point_count) = le_i32(input)?;
    let (inp, points) = many_m_n(
        point_count as usize,
        point_count as usize,
        tuple((le_f32, le_f32)),
    )(inp)?;
    let points = points.into_iter().map(PointF::from).collect();

    Ok((inp, points))
}

pub fn parse_lsi_image(data: &[u8]) -> anyhow::Result<ImageData> {
    let (_, res) = all_consuming(many0(parse_point_sequence))(data)
        .map_err(|e| anyhow::format_err!("Error occurred while parsing image: {e:?}"))?;

    Ok(res)
}
