use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Clone)]
pub enum TileAppearance {
    Color(Color),
    Texture(Handle<Image>),
    None,
}

pub struct TileSpec {
    pub pos: IVec2,
    pub appearance: TileAppearance,
}

#[derive(Component)]
pub struct SolidCollider;

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct SolidTile {
    #[bundle]
    pub sprite: SpriteBundle,
    pub collider: SolidCollider,
    pub tile: Tile,
}

impl SolidTile {
    pub fn from_spec(spec: TileSpec) -> Self {
        let mut tile = SolidTile {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(spec.pos.x as f32, spec.pos.y as f32, 0.),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                ..default()
            },
            collider: SolidCollider {},
            tile: Tile {},
        };

        if let TileAppearance::Color(color) = spec.appearance {
            tile.sprite.sprite.color = color;
        }
        if let TileAppearance::Texture(texture) = spec.appearance {
            tile.sprite.texture = texture;
        }

        tile
    }
}
