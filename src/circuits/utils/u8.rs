use super::Context;
use crate::{circuits::rtable::RangeTableConfig, curr};
use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, VirtualCells},
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct U8Config<F: FieldExt> {
    pub value: Column<Advice>,
    _mark: PhantomData<F>,
}

impl<F: FieldExt> U8Config<F> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        cols: &mut impl Iterator<Item = Column<Advice>>,
        rtable: &RangeTableConfig<F>,
        enable: impl Fn(&mut VirtualCells<'_, F>) -> Expression<F>,
    ) -> Self {
        let value = cols.next().unwrap();

        rtable.configure_in_u8_range(meta, "u8", |meta| curr!(meta, value.clone()) * enable(meta));
        Self {
            value,
            _mark: PhantomData,
        }
    }

    pub fn assign(&self, ctx: &mut Context<F>, value: u64) -> Result<(), Error> {
        ctx.region.assign_advice(
            || "u8 value",
            self.value.clone(),
            ctx.offset,
            || Ok(value.into()),
        )?;

        Ok(())
    }
}