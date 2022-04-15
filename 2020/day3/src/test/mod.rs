use super::*;

#[test]
fn test_pair_from() {
    let pair: Pair = (11, 22).into();
    assert_eq!(pair.x, 11);
    assert_eq!(pair.y, 22);
}

#[test]
fn test_tile_default() {
    let tile: Tile = Default::default();
    assert_eq!(tile, Tile::Open);
}

#[test]
fn test_tile_debug() {
    let tile = Tile::Tree;
    assert_eq!(format!("{:?}", tile), "#");
    let tile = Tile::Open;
    assert_eq!(format!("{:?}", tile), ".");
}

#[test]
fn test_normalize_pos() {
    let m = RoadMap::new((2, 2).into());
    assert_eq!(m.normalize_pos((0, 0).into()), Some((0, 0).into()));
    assert_eq!(m.normalize_pos((1, 0).into()), Some((1, 0).into()));
    assert_eq!(m.normalize_pos((2, 0).into()), Some((0, 0).into()));
    assert_eq!(m.normalize_pos((-1, 0).into()), Some((1, 0).into()));
    assert_eq!(m.normalize_pos((-2, 0).into()), Some((0, 0).into()));
    assert_eq!(m.normalize_pos((0, -1).into()), None);
    assert_eq!(m.normalize_pos((0, 2).into()), None);
}

#[test]
fn test_index() {
    let m = RoadMap::new((3, 5).into());
    assert_eq!(m.index((0, 0).into()), Some(0));
    assert_eq!(m.index((2, 0).into()), Some(2));
    assert_eq!(m.index((0, 1).into()), Some(3));
    assert_eq!(m.index((2, 1).into()), Some(5));
}
