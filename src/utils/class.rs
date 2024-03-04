use bevy::{ecs::world::unsafe_world_cell::UnsafeWorldCell, prelude::*, utils::all_tuples};

pub trait Class<Marker = ()>: Copy + Send + 'static {
    fn apply(self, entity: Entity, world: UnsafeWorldCell);
}
pub trait ClassParam {
    type Param<'w>: ClassParam;
    fn get_param<'w>(entity: Entity, world: UnsafeWorldCell<'w>) -> Self::Param<'w>;
}

pub trait StyleCommand {
    fn set_style<P>(&mut self, entity: Entity, style: impl Class<P>) -> &mut Self;
}

pub trait ChildCommand {
    fn insert_resource<R: Resource>(&mut self, res: R) -> &mut Self;

    fn parent_insert(&mut self, bundle: impl Bundle) -> &mut Self;

    fn entity_insert(&mut self, parent: Entity, bundle: impl Bundle) -> &mut Self;
}

impl ChildCommand for ChildBuilder<'_> {
    fn insert_resource<R: Resource>(&mut self, res: R) -> &mut Self {
        self.add_command(|world: &mut World| {
            world.insert_resource(res);
        });
        self
    }

    fn parent_insert(&mut self, bundle: impl Bundle) -> &mut Self {
        let entity = self.parent_entity();
        self.add_command(move |world: &mut World| {
            world.entity_mut(entity).insert(bundle);
        });
        self
    }

    fn entity_insert(&mut self, entity: Entity, bundle: impl Bundle) -> &mut Self {
        self.add_command(move |world: &mut World| {
            world.entity_mut(entity).insert(bundle);
        });
        self
    }
}

impl StyleCommand for ChildBuilder<'_> {
    fn set_style<P>(&mut self, entity: Entity, style: impl Class<P>) -> &mut Self {
        self.add_command(move |world: &mut World| {
            style.apply(entity, world.as_unsafe_world_cell());
        });
        self
    }
}

impl StyleCommand for Commands<'_, '_> {
    fn set_style<P>(&mut self, entity: Entity, style: impl Class<P>) -> &mut Self {
        self.add(move |world: &mut World| {
            style.apply(entity, world.as_unsafe_world_cell());
        });
        self
    }
}

pub type ClassParamItem<'w, P> = <P as ClassParam>::Param<'w>;

impl<R: Resource> ClassParam for Res<'_, R> {
    type Param<'w> = Res<'w, R>;
    fn get_param<'w>(_entity: Entity, world: UnsafeWorldCell<'w>) -> Self::Param<'w> {
        unsafe { world.get_resource_ref::<R>().unwrap() }
    }
}

impl<T: Component> ClassParam for Mut<'_, T> {
    type Param<'w> = Mut<'w, T>;
    fn get_param<'w>(entity: Entity, world: UnsafeWorldCell<'w>) -> Self::Param<'w> {
        unsafe { world.get_entity(entity).unwrap().get_mut::<T>().unwrap() }
    }
}

macro_rules! impl_class_tuple {
    ($($P: ident),*) => {
        impl<F, $($P),*> Class<($($P,)*)> for F
        where
            F: Send + 'static + Copy + FnOnce($($P), *) + FnOnce($(ClassParamItem<$P>),*),
            $($P: ClassParam,)*
        {
            #[allow(unused_variables)]
            fn apply(self, entity: Entity, world: UnsafeWorldCell) {
                self($($P::get_param(entity, world),)*);
            }
        }
    }
}

all_tuples!(impl_class_tuple, 0, 10, P);

macro_rules! impl_class_more_tuple {
    ($(($P: ident, $p: ident)),*) => {
        #[allow(non_snake_case)]
        impl<'a, $($P, $p),*> Class<($($P,)*)> for ($($p,)*)
        where
            $($p: Class<$P>,)*
        {
            #[allow(unused_variables)]
            fn apply(self, entity: Entity, world: UnsafeWorldCell) {
                let ($($p,)*) = self;
                $($p.apply(entity, world);)*
            }
        }
    };
}

all_tuples!(impl_class_more_tuple, 0, 10, P, S);
