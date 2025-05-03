use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    marker::PhantomData,
};

use rand::{distributions::Uniform, prelude::Distribution, Rng};

use super::CollapseQueue;

use crate::{core::common::*, r#gen::collapse::private::CollapseBounds};

use crate::{gen::collapse::tile::CollapsibleTileData, utils::OrderedFloat};

#[derive(Clone, Copy)]
pub(crate) struct EntrophyItem<D: Dimensionality> {
    pos: D::Pos,
    entrophy: OrderedFloat,
}

impl<D: Dimensionality> EntrophyItem<D> {
    pub fn new(pos: D::Pos, entrophy: f32) -> Self {
        Self {
            pos,
            entrophy: entrophy.into(),
        }
    }
}

impl<D: Dimensionality> Eq for EntrophyItem<D> {}

impl<D: Dimensionality> PartialEq for EntrophyItem<D> {
    fn eq(&self, other: &Self) -> bool {
        self.entrophy == other.entrophy && self.pos == other.pos
    }
}

impl<D: Dimensionality> PartialOrd for EntrophyItem<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<D: Dimensionality> Ord for EntrophyItem<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entrophy
            .cmp(&other.entrophy)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

/// Select next position to collapse using smallest entrophy condition.
///
/// Its state will be updated every time after tile entrophy changed by removing some of its options.
pub struct EntrophyQueue<D: Dimensionality, CB: CollapseBounds<D>, Data: CollapsibleTileData<D, CB>>
{
    by_entrophy: BTreeSet<EntrophyItem<D>>,
    by_pos: HashMap<D::Pos, OrderedFloat>,
    phantom: PhantomData<(CB, Data)>,
}

impl<D: Dimensionality, CB: CollapseBounds<D>, Data: CollapsibleTileData<D, CB>>
    EntrophyQueue<D, CB, Data>
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D: Dimensionality, CB: CollapseBounds<D>, Data: CollapsibleTileData<D, CB>> Default
    for EntrophyQueue<D, CB, Data>
{
    fn default() -> Self {
        Self {
            by_entrophy: BTreeSet::new(),
            by_pos: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

impl<D: Dimensionality, CB: CollapseBounds<D>, Data: CollapsibleTileData<D, CB>>
    CollapseQueue<D, CB, Data> for EntrophyQueue<D, CB, Data>
{
    fn get_next_position(&mut self) -> Option<D::Pos> {
        if let Some(item) = self.by_entrophy.pop_first() {
            self.by_pos.remove(&item.pos);
            return Some(item.pos);
        }
        None
    }

    fn update_queue(&mut self, tile: (D::Pos, &Data)) {
        let item = EntrophyItem::new(tile.0, tile.1.calc_entrophy());
        if let Some(existing_entrophy) = self.by_pos.remove(&item.pos) {
            self.by_entrophy
                .remove(&EntrophyItem::new(item.pos, existing_entrophy.into()));
        }
        self.by_pos.insert(item.pos, item.entrophy);
        self.by_entrophy.insert(item);
    }

    fn len(&self) -> usize {
        self.by_entrophy.len()
    }

    fn is_empty(&self) -> bool {
        self.by_entrophy.is_empty()
    }

    fn initialize_queue(&mut self, tiles: &[(D::Pos, Data)]) {
        for element in tiles {
            self.update_queue((element.0, &element.1))
        }
    }
}

pub(crate) struct EntrophyUniform {
    inner: Uniform<u8>,
}

impl EntrophyUniform {
    const MULTIPLIER: u8 = 124;

    pub fn new() -> Self {
        Self {
            inner: Uniform::<u8>::new(0, EntrophyUniform::MULTIPLIER),
        }
    }

    pub fn sample<R: Rng>(&self, rng: &mut R) -> f32 {
        self.inner.sample(rng) as f32 * OrderedFloat::EPSILON
    }
}

impl<D: Dimensionality, CB: CollapseBounds<D>, Data: CollapsibleTileData<D, CB>>
    super::private::Sealed<D, CB, Data> for EntrophyQueue<D, CB, Data>
{
    fn populate_inner_grid<R: Rng>(
        &mut self,
        rng: &mut R,
        grid: &mut impl GridMap<D, Data>,
        positions: &[D::Pos],
        options_data: &CB::PerOption,
    ) {
        let tiles = Data::new_from_frequency_with_entrophy(rng, positions, options_data);

        self.initialize_queue(&tiles);

        for tile in tiles {
            grid.insert_data(&tile.0, tile.1);
        }
    }

    fn needs_update_after_options_change(&self) -> bool {
        true
    }

    fn propagating(&self) -> bool {
        true
    }
}
