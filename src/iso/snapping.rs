use super::*;

pub enum Snapping {
    None,
    Points(PointsDirection),
    AlongLine { through: IsoPos, axis: IsoAxis },
    AlongAnyLine { through: IsoPos },
}

impl Default for Snapping {
    fn default() -> Self {
        Self::None
    }
}

impl Snapping {
    pub fn require_vertex_pointing_in(direction: IsoDirection) -> Self {
        // Negative directions point in the same directions as the vertices
        // on left-pointing triangles.
        Self::Points(PointsDirection::pick(direction.is_negative()))
    }

    pub fn require_edge_pointing_in(direction: IsoDirection) -> Self {
        // The opposite of _vertex_
        Self::Points(PointsDirection::pick(direction.is_positive()))
    }
}
