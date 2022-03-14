/// The distance from any vertex of a grid cell to its centroid.
pub const GRID_TRIANGLE_RADIUS: f32 = 1.0;

// cos 30deg = (edge / 2) / radius
// cos 30deg = 0.8660254
/// How long each edge of a grid cell is.
pub const GRID_EDGE_LENGTH: f32 = 2.0 * GRID_TRIANGLE_RADIUS * 0.8660254;

// median^2 + (edge / 2)^2 = edge^2
// median^2 = edge^2 - edge^2 * 0.25
// median = sqrt(edge^2 * 0.75)
// median = edge * sqrt(0.75)
// sqrt(0.75) = 0.8660254, huh.
/// The length of the median running between the midpoint of an edge and the
/// opposite vertex.
pub const GRID_MEDIAN_LENGTH: f32 = GRID_EDGE_LENGTH * 0.8660254;

/// The difference in x coordinate between the centroids of two cells
/// stacked on top of each other.
pub const OPPOSING_DISTANCE: f32 = GRID_TRIANGLE_RADIUS * 2.0 - GRID_MEDIAN_LENGTH;

pub const CENTROID_TO_MEDMID_DISTANCE: f32 = GRID_MEDIAN_LENGTH / 2.0 - GRID_TRIANGLE_RADIUS;
