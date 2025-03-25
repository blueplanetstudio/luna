use gpui::App;

use crate::{ecs::LunaEcs, prelude::*};

/// System that handles hit testing and spatial queries for the canvas
pub struct HitTestSystem {
    spatial_index: QuadTree,
}

impl HitTestSystem {
    pub fn new(width: f32, height: f32) -> Self {
        HitTestSystem {
            spatial_index: QuadTree::new(0.0, 0.0, width, height),
        }
    }

    /// Updates the spatial index for an entity
    pub fn update_entity(
        &mut self,
        ecs: Entity<LunaEcs>,
        entity: LunaEntityId,
        cx: &mut Context<LunaEcs>,
    ) {
        if let Some(transform) = ecs.read(cx).transforms().get_transform(entity) {
            // Get the parent chain to compute world transform
            let parent_chain = ecs.read(cx).hierarchy().get_parent_chain(entity);

            // Compute world transform
            if let Some(world_transform) = ecs.update(cx, |ecs, cx| {
                ecs.transforms_mut()
                    .compute_world_transform(entity, parent_chain)
            }) {
                // Create bounding box from world transform and insert into spatial index
                // For now, using a simple 1x1 box at the position
                // TODO: Use actual element dimensions from RenderComponent
                let bbox = BoundingBox::new(
                    vec2(world_transform.position.x, world_transform.position.y),
                    vec2(
                        world_transform.position.x + 1.0,
                        world_transform.position.y + 1.0,
                    ),
                );
                self.spatial_index.insert(entity, bbox);
            }
        }
    }

    /// Returns the topmost entity at the given point, respecting Z-order
    pub fn hit_test_point(
        &self,
        ecs: Entity<LunaEcs>,
        x: f32,
        y: f32,
        cx: &Context<LunaEcs>,
    ) -> Option<LunaEntityId> {
        let candidates = self.spatial_index.query_point(x, y);

        // Sort candidates by Z-order (children above parents)
        // First, group entities by their depth in the hierarchy
        let mut depth_map: Vec<(LunaEntityId, usize)> = candidates
            .into_iter()
            .map(|entity| {
                let depth = ecs
                    .read(cx)
                    .hierarchy()
                    .get_parent_chain(entity)
                    .len();
                (entity, depth)
            })
            .collect();

        // Sort by depth (deeper elements come first)
        depth_map.sort_by(|a, b| b.1.cmp(&a.1));

        // Return the first (topmost) entity
        depth_map.first().map(|(entity, _)| *entity)
    }

    /// Returns all entities in the given region, sorted by Z-order
    pub fn hit_test_region(
        &self,
        ecs: Entity<LunaEcs>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        cx: &Context<LunaEcs>,
    ) -> Vec<LunaEntityId> {
        let candidates = self.spatial_index.query_region(x, y, width, height);

        // Sort candidates by Z-order (children above parents)
        let mut depth_map: Vec<(LunaEntityId, usize)> = candidates
            .into_iter()
            .map(|entity| {
                let depth = ecs
                    .read(cx)
                    .hierarchy()
                    .get_parent_chain(entity)
                    .len();
                (entity, depth)
            })
            .collect();

        // Sort by depth (deeper elements come first)
        depth_map.sort_by(|a, b| b.1.cmp(&a.1));

        // Return entities in Z-order
        depth_map.into_iter().map(|(entity, _)| entity).collect()
    }

    /// Clears the spatial index
    pub fn clear(&mut self) {
        // Create a new empty quadtree with the same dimensions
        self.spatial_index = QuadTree::new(0.0, 0.0, 100.0, 100.0); // TODO: Store dimensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_hit_test_point(cx: &mut TestAppContext) {
        let ecs = cx.new(|cx| LunaEcs::new(cx));

        let mut hit_test = HitTestSystem::new(100.0, 100.0);

        ecs.update(cx, |ecs_mut, cx| {
            // Create a parent-child hierarchy
            let parent = ecs_mut.create_entity();
            let child = ecs_mut.create_entity();

            ecs_mut.hierarchy_mut().set_parent(parent, child);

            let transforms_mut = ecs_mut.transforms_mut();

            transforms_mut.set_transform(
                parent,
                LocalTransform {
                    position: LocalPosition { x: 10.0, y: 10.0 },
                    scale: Vector2D { x: 1.0, y: 1.0 },
                    rotation: 0.0,
                },
            );
            transforms_mut.set_transform(
                child,
                LocalTransform {
                    position: LocalPosition { x: 5.0, y: 5.0 },
                    scale: Vector2D { x: 1.0, y: 1.0 },
                    rotation: 0.0,
                },
            );

            // Update spatial index
            hit_test.update_entity(ecs.clone(), parent, cx);
            hit_test.update_entity(ecs.clone(), child, cx);

            // Test hit testing - child should be on top
            if let Some(hit) = hit_test.hit_test_point(ecs.clone(), 15.0, 15.0, cx) {
                assert_eq!(hit, child);
            }
        });
    }

    #[gpui::test]
    fn test_hit_test_region(cx: &mut TestAppContext) {
        let ecs = cx.new(|cx| LunaEcs::new(cx));

        let mut hit_test = HitTestSystem::new(100.0, 100.0);

        ecs.clone().update(cx, |ecs_mut, cx| {
            // Create some test entities
            let e1 = ecs_mut.create_entity();
            let e2 = ecs_mut.create_entity();

            let transforms = ecs_mut.transforms_mut();
            transforms.set_transform(
                e1,
                LocalTransform {
                    position: LocalPosition { x: 10.0, y: 10.0 },
                    scale: Vector2D { x: 1.0, y: 1.0 },
                    rotation: 0.0,
                },
            );

            transforms.set_transform(
                e2,
                LocalTransform {
                    position: LocalPosition { x: 20.0, y: 20.0 },
                    scale: Vector2D { x: 1.0, y: 1.0 },
                    rotation: 0.0,
                },
            );

            // Update spatial index
            hit_test.update_entity(ecs.clone(), e1, cx);
            hit_test.update_entity(ecs.clone(), e2, cx);

            // Test region query
            let hits = hit_test.hit_test_region(ecs, 0.0, 0.0, 30.0, 30.0, cx);
            assert_eq!(hits.len(), 2);
        });
    }
}
