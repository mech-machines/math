use crate::*;
use mech_core::*;
use libm::{atanh, atanhf};

// Atanh Macros
macro_rules! atanh_op {
  ($arg:expr, $out:expr) => {
    unsafe { (*$out).0 = atanh((*$arg).0); }
  };
}

macro_rules! atanh_vec_op {
  ($arg:expr, $out:expr) => {
    unsafe {
      for i in 0..(*$arg).len() {
        ((*$out)[i]).0 = atanh(((*$arg)[i]).0);
      }
    }
  };
}

macro_rules! atanhf_op {
  ($arg:expr, $out:expr) => {
    unsafe { (*$out).0 = atanhf((*$arg).0); }
  };
}

macro_rules! atanhf_vec_op {
  ($arg:expr, $out:expr) => {
    unsafe {
      for i in 0..(*$arg).len() {
        ((*$out)[i]).0 = atanhf(((*$arg)[i]).0);
      }
    }
  };
}

impl_math_urop!(MathAtanh, F32, atanhf);
impl_math_urop!(MathAtanh, F64, atanh);

fn impl_atanh_fxn(lhs_value: Value) -> Result<Box<dyn MechFunction>, MechError> {
  impl_urnop_match_arms2!(
    MathAtanh,
    (lhs_value),
    F32 => MatrixF32, F32, F32::zero(), "F32";
    F64 => MatrixF64, F64, F64::zero(), "F64";
  )
}

pub struct MathAtanh {}

impl NativeFunctionCompiler for MathAtanh {
  fn compile(&self, arguments: &Vec<Value>) -> MResult<Box<dyn MechFunction>> {
    if arguments.len() != 1 {
      return Err(MechError {
        file: file!().to_string(),
        tokens: vec![],
        msg: "".to_string(),
        id: line!(),
        kind: MechErrorKind::IncorrectNumberOfArguments,
      });
    }
    let input = arguments[0].clone();
    match impl_atanh_fxn(input.clone()) {
      Ok(fxn) => Ok(fxn),
      Err(_) => match input {
        Value::MutableReference(input) => impl_atanh_fxn(input.borrow().clone()),
        _ => Err(MechError {
          file: file!().to_string(),
          tokens: vec![],
          msg: "".to_string(),
          id: line!(),
          kind: MechErrorKind::UnhandledFunctionArgumentKind,
        }),
      },
    }
  }
}
