use godot::builtin::Vector2i;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct DrawTile {
    source_id: i32,
    atlas_coords: Vector2i,
    alternative_tile: i32,
}

impl DrawTile {
    /// Minimal constructor (alt tile defaults to 0).
    pub(crate) const fn new(source_id: i32, atlas_coords: Vector2i) -> Self {
        Self { source_id, atlas_coords, alternative_tile: 0 }
    }

    /// Full constructor when you want to set the alternative immediately.
    pub(crate) const fn new_with_alt(source_id: i32, atlas_coords: Vector2i, alternative_tile: i32) -> Self {
        Self { source_id, atlas_coords, alternative_tile }
    }

    /// Return a copy with a different alternative tile.
    pub(crate) const fn with_alt(self, alt: i32) -> Self {
        Self { alternative_tile: alt, ..self }
    }

    /// Useful sentinel check (e.g., source_id < 0 means “no tile”).
    pub(crate) const fn is_valid(&self) -> bool { self.source_id >= 0 }

    // --- kept getters (drop-in compatible) ---
    #[inline] pub(crate) const fn get_source_id(&self) -> i32 { self.source_id }
    #[inline] pub(crate) const fn get_atlas_coords(&self) -> Vector2i { self.atlas_coords }
    #[inline] pub(crate) const fn get_alternative_tile(&self) -> i32 { self.alternative_tile }
}

/// NESW mask bits for 4‑way autotiling.
pub(crate) mod mask {
    pub const N: u8 = 1 << 0;
    pub const E: u8 = 1 << 1;
    pub const S: u8 = 1 << 2;
    pub const W: u8 = 1 << 3;
}

/// A tiny helper to pick atlas coords based on a 4‑way bitmask (NESW).
#[derive(Copy, Clone, Debug)]
pub(crate) struct AutoTileSet {
    source_id: i32,
    /// Index by mask (0..=15). Each entry is an atlas coord for that mask.
    mapping: [Vector2i; 16],
}

impl AutoTileSet {
    pub(crate) const fn new(source_id: i32, mapping: [Vector2i; 16]) -> Self {
        Self { source_id, mapping }
    }

    /// Create a DrawTile from a 4‑way mask (bits: NESW). Alt defaults to 0.
    #[inline]
    pub(crate) const fn pick(&self, mask: u8) -> DrawTile {
        let idx = (mask & 0b1111) as usize;
        DrawTile::new(self.source_id, self.mapping[idx])
    }
}

// Const-friendly shorthand for Vector2i literals.
const fn v(x: i32, y: i32) -> Vector2i { Vector2i::from_tuple((x, y)) }

/// Example 4×4 atlas layout (index = NESW mask). Adjust to your tileset layout.
///
///  0..3 in row 0, 4..7 in row 1, etc.
pub(crate) const FOUR_WAY_MAPPING: [Vector2i; 16] = [
    v(0,0), v(1,0), v(2,0), v(3,0),
    v(0,1), v(1,1), v(2,1), v(3,1),
    v(0,2), v(1,2), v(2,2), v(3,2),
    v(0,3), v(1,3), v(2,3), v(3,3),
];

/// Ready-to-go autotile set for source 0 (change source_id if needed).
pub(crate) const FOUR_WAY_AUTOTILESET: AutoTileSet = AutoTileSet::new(0, FOUR_WAY_MAPPING);

/// Kept constants (now built via const constructors).
pub(crate) const FOUR_WAY_DRAW_TILE: DrawTile = DrawTile::new(0, v(1, 0));
pub(crate) const BASE_DRAW_TILE: DrawTile     = DrawTile::new(0, v(0, 0));

/// Optional sentinel for “no tile”.
pub(crate) const EMPTY_DRAW_TILE: DrawTile    = DrawTile::new_with_alt(-1, v(0, 0), 0);
