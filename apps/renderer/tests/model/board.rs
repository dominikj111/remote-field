use bevy::render::color::Color;
use renderer::{Board, Entity};

#[test]
fn has_new_and_default() {
    let board1: Board<usize> = Board::new();
    let board2: Board<usize> = Board::default();

    assert_eq!(board1, board2);
}

#[test]
fn out_of_bounds_coordinates_return_none() {
    let board: Board<()> = Board::new();
    assert_eq!(board.get(3, 1), None);
    assert_eq!(board.dimension(), (0, 0));
}

#[test]
fn maximum_provided_coordinates_defines_the_board_dimension() {
    let mut board = Board::new();
    let entity = Entity::<()> {
        coordinates: (3, 1),
        color: Color::GREEN,
        ..Default::default()
    };
    assert_eq!(board.dimension(), (0, 0));
    board.set(entity);
    assert_eq!(board.dimension(), (4, 2));
}

#[test]
fn allows_to_set_and_get_entities() {
    let mut board = Board::new();
    let entity1 = Entity::<()> {
        coordinates: (1, 1),
        color: Color::GREEN,
        ..Default::default()
    };
    let entity2 = Entity::<()> {
        coordinates: (1, 2),
        color: Color::RED,
        ..Default::default()
    };
    let entity3 = Entity::<()> {
        coordinates: (2, 3),
        color: Color::BLUE,
        ..Default::default()
    };

    assert_eq!(board.get(1, 1), None);

    board.set(entity1);
    board.set(entity2);
    board.set(entity3);

    assert_eq!(board.get(1, 1), Some(entity1));
    assert_eq!(board.get(1, 2).unwrap().color, Color::RED);
    assert_eq!(board.get(2, 3).unwrap().color, Color::BLUE);

    assert_eq!(board.dimension(), (3, 4));
}

#[test]
fn allows_to_clear_entities() {
    let mut board = Board::new();
    let entity1 = Entity::<()> {
        coordinates: (2, 4),
        color: Color::GREEN,
        ..Default::default()
    };

    assert_eq!(board.get(2, 4), None);

    board.set(entity1);

    assert_eq!(board.get(2, 4), Some(entity1));

    board.clear(2, 4);

    assert_eq!(board.get(2, 4), None);
}

#[test]
#[should_panic]
fn doesnt_override_existing_entities() {
    let mut board = Board::new();
    let entity1 = Entity::<()> {
        coordinates: (1, 1),
        color: Color::GREEN,
        ..Default::default()
    };
    let entity2 = Entity::<()> {
        coordinates: (1, 1),
        color: Color::RED,
        ..Default::default()
    };

    board.set(entity1);
    board.set(entity2);
}

#[test]
fn allows_to_override_existing_entities_if_flag_is_set() {
    let mut board = Board::new();
    let entity1 = Entity::<()> {
        coordinates: (1, 1),
        color: Color::GREEN,
        ..Default::default()
    };
    let entity2 = Entity::<()> {
        coordinates: (1, 1),
        color: Color::RED,
        ..Default::default()
    };

    board.set_force(entity1, true);
    board.set_force(entity2, true);

    assert_eq!(board.get(1, 1), Some(entity2));
}
