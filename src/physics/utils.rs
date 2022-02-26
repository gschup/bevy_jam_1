use bevy::{
    ecs::query::{Fetch, FilterFetch, QueryEntityError, WorldQuery},
    prelude::*,
};

pub trait QueryExt<Q: WorldQuery> {
    /// Get mutable access to the components of a pair entities in this query
    fn get_pair_mut(
        &mut self,
        a: Entity,
        b: Entity,
    ) -> Result<(<Q::Fetch as Fetch>::Item, <Q::Fetch as Fetch>::Item), QueryEntityError>;
}

impl<'w, 's, Q: WorldQuery, F: WorldQuery> QueryExt<Q> for Query<'w, 's, Q, F>
where
    F::Fetch: FilterFetch,
{
    fn get_pair_mut(
        &mut self,
        a: Entity,
        b: Entity,
    ) -> Result<(<Q::Fetch as Fetch>::Item, <Q::Fetch as Fetch>::Item), QueryEntityError> {
        let (res_a, res_b) = unsafe {
            // Ensure safety
            assert!(a != b);
            (self.get_unchecked(a), self.get_unchecked(b))
        };
        match (res_a, res_b) {
            (Ok(res_a), Ok(res_b)) => Ok((res_a, res_b)),
            _ => Err(QueryEntityError::QueryDoesNotMatch),
        }
    }
}
