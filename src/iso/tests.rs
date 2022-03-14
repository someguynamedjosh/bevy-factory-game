#![cfg(test)]

use super::*;

#[test]
fn pointiness() {
    assert!(IsoPos::new(0, 0).points_left());
    assert!(IsoPos::new(0, 1).points_right());
    assert!(IsoPos::new(1, 0).points_right());
    assert!(IsoPos::new(-1, 0).points_right());
    assert!(IsoPos::new(-2, 0).points_left());
    for i in -10..10 {
        assert!(IsoPos::new(i, i).points_left());
    }
}

#[test]
fn axis_offsets() {
    assert_eq!(IsoPos::new(0, 0).offset_a(1), IsoPos::new(0, 1));
    assert_eq!(IsoPos::new(0, 0).offset_a(-2), IsoPos::new(0, -2));

    assert_eq!(IsoPos::new(0, 0).offset_b(4), IsoPos::new(-2, -2));
    assert_eq!(IsoPos::new(0, 0).offset_b(5), IsoPos::new(-2, -3));
    assert_eq!(IsoPos::new(-1, 0).offset_b(5), IsoPos::new(-4, -2));
    assert_eq!(IsoPos::new(1, 0).offset_b(-2), IsoPos::new(2, 1));

    assert_eq!(IsoPos::new(0, 0).offset_c(4), IsoPos::new(2, -2));
    assert_eq!(IsoPos::new(0, 0).offset_c(5), IsoPos::new(3, -2));
    assert_eq!(IsoPos::new(-1, 0).offset_c(5), IsoPos::new(1, -3));
    assert_eq!(IsoPos::new(1, 0).offset_c(-2), IsoPos::new(0, 1));

    let mut direction = IsoDirection::PosA;
    let mut pos = IsoPos::origin();
    // Move an equal number of steps in each of the 6 directions, should take us
    // back to the place we started from.
    for _ in 0..6 {
        pos = pos.offset_direction(direction, 12);
        direction = direction.clockwise();
    }
    assert_eq!(direction, IsoDirection::PosA);
    assert_eq!(pos, IsoPos::origin());
}

#[test]
fn perp_axis_offsets() {
    assert_eq!(IsoPos::new(0, 0).offset_perp_a(1), IsoPos::new(-1, 0));

    assert_eq!(IsoPos::new(0, 0).offset_perp_b(-1), IsoPos::new(0, 1));
    assert_eq!(IsoPos::new(0, 0).offset_perp_b(-3), IsoPos::new(-1, 4));
    assert_eq!(IsoPos::new(1, 0).offset_perp_b(-2), IsoPos::new(0, 3));
    assert_eq!(IsoPos::new(1, 0).offset_perp_b(2), IsoPos::new(2, -3));

    assert_eq!(IsoPos::new(0, 0).offset_perp_c(1), IsoPos::new(1, 2));
    assert_eq!(IsoPos::new(0, 0).offset_perp_c(3), IsoPos::new(2, 5));
    assert_eq!(IsoPos::new(1, 0).offset_perp_c(2), IsoPos::new(2, 3));
    assert_eq!(IsoPos::new(1, 0).offset_perp_c(-2), IsoPos::new(0, -3));
}
