use super::{
    animate::Animator,
    class::{ChildCommand, Class},
};
use bevy::prelude::*;

macro_rules! widget_bundle {
    ($name:ident, $bundle:ident) => {
        pub fn $name<P>(
            class: impl Class<P>,
            commands: &mut ChildBuilder,
            relate: impl Bundle,
        ) -> Entity {
            commands
                .spawn(($bundle::default(), relate))
                .add(move |entity: Entity, world: &mut World| {
                    class.apply(entity, world.as_unsafe_world_cell());
                })
                .id()
        }
    };
}

macro_rules! widget_bundle_children {
    ($name:ident, $bundle:ident) => {
        pub fn $name<P>(
            class: impl Class<P>,
            commands: &mut ChildBuilder,
            relate: impl Bundle,
            children: impl FnOnce(&mut ChildBuilder),
        ) -> Entity {
            commands
                .spawn(($bundle::default(), relate))
                .add(move |entity: Entity, world: &mut World| {
                    class.apply(entity, world.as_unsafe_world_cell());
                })
                .with_children(children)
                .id()
        }
    };
}

macro_rules! widget_bundle_root {
    ($name:ident, $bundle:ident) => {
        pub fn $name<P>(
            class: impl Class<P>,
            mut commands: Commands,
            relate: impl Bundle,
            children: impl FnOnce(&mut ChildBuilder),
        ) -> Entity {
            commands
                .spawn(($bundle::default(), relate))
                .add(move |entity: Entity, world: &mut World| {
                    class.apply(entity, world.as_unsafe_world_cell());
                })
                .with_children(children)
                .id()
        }
    };
}

widget_bundle!(node, NodeBundle);
widget_bundle!(button, ButtonBundle);
widget_bundle!(image, ImageBundle);
widget_bundle!(atlas_image, AtlasImageBundle);
widget_bundle!(sprite, SpriteBundle);
widget_bundle!(sprite_sheet, SpriteSheetBundle);
widget_bundle_children!(node_children, NodeBundle);
widget_bundle_children!(button_children, ButtonBundle);
widget_bundle_children!(image_children, ImageBundle);
widget_bundle_children!(atlas_image_children, AtlasImageBundle);
widget_bundle_children!(sprite_children, SpriteBundle);
widget_bundle_children!(sprite_sheet_children, SpriteSheetBundle);
widget_bundle_root!(node_root, NodeBundle);
widget_bundle_root!(sprite_root, SpriteBundle);

pub fn text<P>(
    text_list: impl IntoIterator<Item = impl Into<String>>,
    class: impl Class<P>,
    commands: &mut ChildBuilder,
    relate: impl Bundle,
) -> Entity {
    commands
        .spawn((
            TextBundle::from_sections(text_list.into_iter().map(|v| TextSection::from(v.into()))),
            relate,
        ))
        .add(move |entity: Entity, world: &mut World| {
            class.apply(entity, world.as_unsafe_world_cell());
        })
        .id()
}

pub fn node_text<P>(
    text: impl Into<String>,
    class: impl Class<P>,
    commands: &mut ChildBuilder,
    relate: impl Bundle,
) -> Entity {
    commands
        .spawn((TextBundle::from_section(text, TextStyle::default()), relate))
        .add(move |entity: Entity, world: &mut World| {
            class.apply(entity, world.as_unsafe_world_cell());
        })
        .id()
}

pub fn node_text_a<P>(
    text: impl Into<String>,
    class: impl Class<P>,
    commands: &mut ChildBuilder,
    relate: impl Bundle,
) -> Entity {
    commands
        .spawn((
            TextBundle::from_section(text, TextStyle::default()),
            Animator::default(),
            relate,
        ))
        .add(move |entity: Entity, world: &mut World| {
            class.apply(entity, world.as_unsafe_world_cell());
        })
        .id()
}

pub fn text2d<P>(
    text: impl Into<String>,
    class: impl Class<P>,
    commands: &mut ChildBuilder,
    relate: impl Bundle,
) -> Entity {
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(text, TextStyle::default()),
                ..default()
            },
            relate,
        ))
        .add(move |entity: Entity, world: &mut World| {
            class.apply(entity, world.as_unsafe_world_cell());
        })
        .id()
}

#[derive(Component, Debug)]
pub struct GridItemInfo(pub usize, pub usize);

#[derive(Resource, Debug)]
pub struct GridInfo(pub usize, pub usize, pub f32);

pub fn grid<P, C>(
    rows: usize,
    cols: usize,
    item_height: f32,
    custom_grid_class: impl Class<P>,
    custom_grid_item_class: impl Class<C>,
    commands: &mut ChildBuilder,
    relate: impl Bundle,
    mut children: impl FnMut(&mut ChildBuilder, usize, usize),
) -> Entity {
    commands
        .insert_resource(GridInfo(rows, cols, item_height))
        .spawn((NodeBundle::default(), relate))
        .add(move |entity: Entity, world: &mut World| {
            (custom_grid_class, grid_class).apply(entity, world.as_unsafe_world_cell());
        })
        .with_children(|parent| {
            for row in 0..rows {
                for col in 0..cols {
                    node_children(
                        (grid_item_class, custom_grid_item_class),
                        parent,
                        GridItemInfo(row, col),
                        |gc| {
                            children(gc, row, col);
                        },
                    );
                }
            }
        })
        .id()
}

fn grid_class(mut style: Mut<Style>) {
    style.display = Display::Flex;
    style.flex_wrap = FlexWrap::Wrap;
}

fn grid_item_class(mut style: Mut<Style>, gird_info: Res<GridInfo>) {
    style.width = Val::Percent(100.0 / gird_info.1 as f32);
    style.height = Val::Percent(100.0 / gird_info.0 as f32);
    style.justify_content = JustifyContent::Center;
    style.overflow = Overflow::clip();
    style.align_items = AlignItems::Center;
}
