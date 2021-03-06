use crate::{List, Point, Vector2};

#[derive(Clone, Debug)]
pub struct Step {
    length: usize,
    points: List<Point>,
}

impl Step {
    #[must_use]
    pub fn with_vector(&self, vector: Vector2) -> Self {
        let position = self.position();
        Step {
            length: self.length + 1,
            points: self.points.push(position + vector),
        }
    }

    pub fn from_point(pt: Point) -> Self {
        Step {
            points: List::new().push(pt),
            length: 1,
        }
    }

    pub fn last_vector(&self) -> Option<Vector2> {
        let mut pts = self.points();

        let last_point = pts.next()?;
        let prev_last_point = pts.next()?;
        Some(Vector2 {
            x: last_point.x - prev_last_point.x,
            y: last_point.y - prev_last_point.y,
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn points(&self) -> impl Iterator<Item=Point> + '_ {
        self.points.iter()
    }

    pub fn into_points(self) -> Vec<Point> {
        let mut pts: Vec<_> = self.points.iter().collect();
        // the points are stored in reverse order
        pts.reverse();
        pts
    }

    pub fn position(&self) -> Point {
        self.points.iter().next().unwrap()
    }
}
