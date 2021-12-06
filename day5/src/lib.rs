use std::fmt;


/// Point compose of two usize coordinates
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(point: (usize, usize)) -> Point {
        Point {
            x: point.0,
            y: point.1,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// Segment composed of two points
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Segment {
    pub p1: Point,
    pub p2: Point,
}

impl Segment {
    pub fn new(segment: (Point, Point)) -> Segment {       
        Segment {
            p1: segment.0,
            p2: segment.1,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn is_45_degrees(&self) -> bool {
        (self.p1.x as i32 - self.p2.x as i32).abs() == (self.p1.y as i32 - self.p2.y as i32).abs()
    }

    /// List points of the segment
    pub fn list_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        
        let range;
        // TODO: List 45 degrees points
        if self.is_horizontal() {
            if self.p1.x > self.p2.x {
                range = self.p2.x..=self.p1.x;
            }
            else {
                range = self.p1.x..=self.p2.x;
            }
            for x in range {
                points.push(Point::new((x, self.p1.y)));
            }
        }
        else if self.is_vertical() {
            if self.p1.y > self.p2.y {
                range = self.p2.y..=self.p1.y;
            }
            else {
                range = self.p1.y..=self.p2.y;
            }

            for y in range {
                points.push(Point::new((self.p1.x, y)));
            }
        }
        points
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_horizontal() {
        let seg_horiz = Segment::new((Point::new((0, 9)), Point::new((5, 9))));
        let seg_vertic = Segment::new((Point::new((5, 19)), Point::new((5, 9))));
        let seg_diag = Segment::new((Point::new((5, 5)), Point::new((10, 10))));

        assert_eq!(true, seg_horiz.is_horizontal());
        assert_eq!(false, seg_vertic.is_horizontal());
        assert_eq!(false, seg_diag.is_horizontal());
    }

    #[test]
    fn test_is_vertical() {
        let seg_horiz = Segment::new((Point::new((0, 9)), Point::new((5, 9))));
        let seg_vertic = Segment::new((Point::new((5, 19)), Point::new((5, 9))));
        let seg_diag = Segment::new((Point::new((5, 5)), Point::new((10, 10))));

        assert_eq!(true, seg_vertic.is_vertical());
        assert_eq!(false, seg_horiz.is_vertical());
        assert_eq!(false, seg_diag.is_vertical());
    }

    #[test]
    fn test_is_45_degree() {
        let seg_horiz = Segment::new((Point::new((0, 9)), Point::new((5, 9))));
        let seg_vertic = Segment::new((Point::new((5, 19)), Point::new((5, 9))));
        let seg_diag1 = Segment::new((Point::new((5, 5)), Point::new((10, 10))));
        let seg_diag2 = Segment::new((Point::new((5, 0)), Point::new((0, 5))));
        let seg_diag3 = Segment::new((Point::new((0, 5)), Point::new((5, 0))));
        let seg_diag4 = Segment::new((Point::new((8, 0)), Point::new((5, 3))));

        assert_eq!(false, seg_horiz.is_45_degrees());
        assert_eq!(false, seg_vertic.is_45_degrees());
        assert_eq!(true, seg_diag1.is_45_degrees());
        assert_eq!(true, seg_diag2.is_45_degrees());
        assert_eq!(true, seg_diag3.is_45_degrees());
        assert_eq!(true, seg_diag4.is_45_degrees());
    }
}