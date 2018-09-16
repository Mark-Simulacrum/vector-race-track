use std::cmp::Ordering;
use {Point, Vector2};

#[derive(Clone, Debug)]
pub struct Step {
    points: Vec<Point>,
}

impl Step {
    pub fn with_vector(mut self, vector: Vector2) -> Self {
        let position = self.position();
        self.points.push(position + vector);
        Step {
            points: self.points,
        }
    }

    pub fn from_point(pt: Point) -> Self {
        Step {
            points: vec![pt],
        }
    }

    pub fn last_vector(&self) -> Option<Vector2> {
        let mut pts_rev = self.points.iter().rev();
        let last_point = pts_rev.next().unwrap();
        let prev_last_point = pts_rev.next()?;
        Some(Vector2 {
            x: last_point.x - prev_last_point.x,
            y: last_point.y - prev_last_point.y,
        })
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn points(&self) -> impl Iterator<Item=Point> + '_ {
        self.points.iter().cloned()
    }

    pub fn into_points(self) -> Vec<Point> {
        self.points
    }

    pub fn position(&self) -> Point {
        *self.points.last().unwrap()
    }
}

impl Eq for Step {}
impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.points.len() == other.points.len() && self.position() == other.position()
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points.len().cmp(&other.points.len())
    }
}


