extern crate recs;

pub use recs::Ecs;

pub trait EcsRetrievable{
    type T;

    fn retrieve(ecs: &Ecs) -> Option<&Self::T>;

    fn retrieve_mut(ecs: &mut Ecs) -> Option<&mut Self::T>;
}