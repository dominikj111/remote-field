use bevy::{color::palettes, prelude::Color};
use renderer::Entity;

#[test]
fn has_new_and_default() {
    let entity = Entity::<()> {
        coordinates: (2, 4),
        ..Default::default()
    };
    assert_eq!(entity.color, Color::BLACK);
    assert_eq!(entity.coordinates, (2, 4));

    let entity2: Entity<()> = Entity::new((4, 21));
    assert_eq!(entity2.coordinates, (4, 21));

    let entity3: Entity<()> = Entity::new_with_color((3, 3), palettes::basic::RED.into());
    assert_eq!(entity3.color, palettes::basic::RED.into());
    assert_eq!(entity3.coordinates, (3, 3));

    let entity4: Entity<i32> = Entity::new_with_data((0, 0), 9);
    assert_eq!(entity4.data, Some(9));

    assert_eq!(Entity::<()>::default(), Entity::new((0, 0)));
}

#[test]
fn allows_to_store_other_data() {
    let entity1 = Entity {
        coordinates: (2, 4),
        data: Some(9),
        ..Default::default()
    };

    assert_eq!(entity1.data, Some(9));

    #[derive(Clone, Copy, Default)]
    struct Person<'a> {
        name: &'a str,
        age: u32,
        is_student: bool,
    }

    #[derive(Clone, Copy, Default)]
    struct MyDataContainer<'a> {
        data_a: Option<i32>,
        data_b: bool,
        data_c: Person<'a>,
    }

    let entity2 = Entity::<MyDataContainer> {
        coordinates: (2, 4),
        data: Some(MyDataContainer {
            data_a: Some(10),
            data_b: true,
            data_c: Person {
                name: "John",
                age: 30,
                is_student: true,
            },
        }),
        ..Default::default()
    };

    assert_eq!(entity2.data.unwrap().data_a.unwrap(), 10);
    assert_eq!(entity2.data.unwrap().data_b, true);
    assert_eq!(entity2.data.unwrap().data_c.name, "John");
}
