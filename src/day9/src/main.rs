use std::cmp::{max, min};
use std::time::Instant;
use core::math::point2d::Point2D;

struct VerticalLine{
    x: i64,
    y1: i64,
    y2: i64,
}

struct HorizontalLine {
    y: i64,
    x1: i64,
    x2: i64,
}

fn main() {
    let start = Instant::now();
    const PATH: &str = "src/day9/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let mut points = lines
        .map_while(Result::ok)
        .map(|x| Point2D::parse(&x))
        .collect::<Result<Vec<Point2D>, String>>()
        .unwrap();

    let mut horizontal_lines: Vec<HorizontalLine> = vec![];
    let mut vertical_lines: Vec<VerticalLine> = vec![];

    points.push(points[0]);

    for point in points.windows(2) {
        let first = &point[0];
        let second = &point[1];

        let min_x = min(first.x, second.x);
        let max_x = max(first.x, second.x);
        let min_y = min(first.y, second.y);
        let max_y = max(first.y, second.y);

        // This is a vertical line
        if min_x == max_x {
            vertical_lines.push(VerticalLine{
                x: min_x,
                y1: min_y,
                y2: max_y
            });
            continue;
        }

        // This is a horizontal line
        if min_y == max_y {
            horizontal_lines.push(HorizontalLine{
                y: min_y,
                x1: min_x,
                x2: max_x
            });
            continue;
        }

        panic!("Can't parse input, line is not horizontal or vertical!")
    }

    let mut max_area_part1 : u64 = 0;
    let mut max_area_part2 : u64 = 0;
    for outer_idx in 0..points.len()-1 {
        for inner_idx in outer_idx + 1..points.len()-1 {
            let outer = &points[outer_idx];
            let inner = &points[inner_idx];

            let vec = outer - inner;
            let height = vec.y.abs() + 1;
            let width = vec.x.abs() + 1;

            let area = height as u64 * width as u64;

            if area > max_area_part1 {
                max_area_part1 = area;
            }

            if area < max_area_part2 {
                continue;
            }

            let left_border_x = min(outer.x, inner.x);
            let right_border_x = max(outer.x, inner.x);
            let top_border_y = min(outer.y, inner.y);
            let bottom_border_y = max(outer.y, inner.y);

            let horizontal_crossed = horizontal_lines.iter().any(|horizontal_line| {
                if horizontal_line.y <= top_border_y || bottom_border_y <= horizontal_line.y {
                    return false;
                }

                if horizontal_line.x1 <= left_border_x && left_border_x < horizontal_line.x2 {
                    return true;
                }

                if horizontal_line.x1 < right_border_x && right_border_x <= horizontal_line.x2{
                    return true;
                }

                false
            });

            if horizontal_crossed {
                continue;
            }

            let vertical_crossed = vertical_lines.iter().any(|vertical_line| {
                if vertical_line.x <= left_border_x || right_border_x <= vertical_line.x {
                    return false;
                }

                if vertical_line.y1 <= top_border_y && top_border_y < vertical_line.y2 {
                    return true;
                }

                if vertical_line.y1 < bottom_border_y && bottom_border_y <= vertical_line.y2{
                    return true;
                }

                false
            });

            if vertical_crossed {
                continue;
            }

            max_area_part2 = area;
        }
    }
    let duration = start.elapsed();

    println!("Max area part1: {}", max_area_part1);
    println!("Max area part2: {}", max_area_part2);
    println!("Total time: {:?}", duration);
}
