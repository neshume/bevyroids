use bevy::prelude::*;

pub struct BoundaryPlugin;

impl Plugin for BoundaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(boundary_remove_system)
                .with_system(boundary_wrap_system),
        );
    }
}

#[derive(Debug, Component, Default, Clone, Copy, Deref, DerefMut)]
pub struct Bounding(f32);

impl Bounding {
    pub fn from_radius(radius: f32) -> Self {
        Self(radius)
    }
}

#[derive(Debug, Component, Default)]
pub struct BoundaryWrap;

#[derive(Debug, Component, Default)]
pub struct BoundaryRemoval;

fn boundary_wrap_system(
    windows: ResMut<Windows>,
    mut query: Query<(&mut Transform, &Bounding), With<BoundaryWrap>>,
) {
    if let Some(window) = windows.get_primary() {
        for (mut transform, radius) in query.iter_mut() {
            let x = transform.translation.x;
            let y = transform.translation.y;

            let half_width = window.width() / 2.0;
            if x + radius.0 * 2.0 < -half_width {
                transform.translation.x = half_width + radius.0 * 2.0;
            } else if x - radius.0 * 2.0 > half_width {
                transform.translation.x = -half_width - radius.0 * 2.0;
            }

            let half_height = window.height() / 2.0;
            if y + radius.0 * 2.0 < -half_height {
                transform.translation.y = half_height + radius.0 * 2.0;
            } else if y - radius.0 * 2.0 > half_height {
                transform.translation.y = -half_height - radius.0 * 2.0;
            }
        }
    }
}

fn boundary_remove_system(
    mut commands: Commands,
    windows: ResMut<Windows>,
    query: Query<(Entity, &Transform, &Bounding), With<BoundaryRemoval>>,
) {
    if let Some(window) = windows.get_primary() {
        for (entity, transform, radius) in query.iter() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;
            let x = transform.translation.x;
            let y = transform.translation.y;
            if x + radius.0 * 2.0 < -half_width
                || x - radius.0 * 2.0 > half_width
                || y + radius.0 * 2.0 < -half_height
                || y - radius.0 * 2.0 > half_height
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
