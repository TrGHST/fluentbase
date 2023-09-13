pub use crate::exec_step::{ExecStep, GadgetError};
use crate::{
    runtime_circuit::{constraint_builder::OpConstraintBuilder, execution_state::ExecutionState},
    util::Field,
};
use halo2_proofs::circuit::Region;

// mod op_bin;
pub(crate) mod op_bin;
pub(crate) mod op_bin_shift;
pub(crate) mod op_bitwise;
pub(crate) mod op_break;
pub(crate) mod op_call;
pub(crate) mod op_const;
pub(crate) mod op_conversion;
pub(crate) mod op_drop;
pub(crate) mod op_extend;
pub(crate) mod op_global;
pub(crate) mod op_load;
pub(crate) mod op_local;
pub(crate) mod op_memory;
pub(crate) mod op_reffunc;
pub(crate) mod op_rel;
pub(crate) mod op_select;
// pub(crate) mod op_shift;
pub(crate) mod op_store;
pub(crate) mod op_test;
pub(crate) mod op_unary;
pub(crate) mod table_ops;

#[macro_export]
macro_rules! bail_illegal_opcode {
    ($trace:expr) => {
        unreachable!("illegal opcode place {:?}", $trace)
    };
}

pub trait ExecutionGadget<F: Field> {
    const NAME: &'static str;

    const EXECUTION_STATE: ExecutionState;

    fn configure(cb: &mut OpConstraintBuilder<F>) -> Self;

    fn assign_exec_step(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        trace: &ExecStep,
    ) -> Result<(), GadgetError>;
}
