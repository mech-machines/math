#![allow(warnings)]
extern crate mech_core;
extern crate mech_utilities;
extern crate libm;
#[macro_use]
extern crate lazy_static;
use mech_core::*;
use mech_utilities::*;
use libm::{sinf, cosf, fmodf, roundf, floorf};
use std::cell::RefCell;
use std::rc::Rc;

static PI: f32 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286;

lazy_static! {
  static ref ANGLE: u64 = hash_str("angle");
  static ref TABLE: u64 = hash_str("table");
}

#[derive(Debug)]
pub struct MathSinRadVV {
  pub col: (ColumnV<F32>,usize,usize), pub out: ColumnV<F32>
}

impl MechFunction for MathSinRadVV {
  fn solve(&self) {
    let (col,six,eix) = &self.col;
    self.out.borrow_mut()
            .iter_mut()
            .zip(col.borrow()[*six..=*eix].iter())
            .for_each(|(out, rhs)| *out = F32::new(sinf(rhs.unwrap()))); 
  }
  fn to_string(&self) -> String { format!("{:#?}", self)}
}


pub struct MathSin{}
impl MechFunctionCompiler for MathSin {
  fn compile(&self, block: &mut Block, arguments: &Vec<Argument>, out: &(TableId, TableIndex, TableIndex)) -> std::result::Result<(),MechError> {
    println!("!!!!!!!!!!!!!!!!!!!!!!!!{:?}", arguments);
    if arguments.len() > 1 {
      return Err(MechError{id: 1347, kind: MechErrorKind::TooManyInputArguments(arguments.len(),1)});
    }
    let arg_dims = block.get_arg_dims(&arguments)?;
    println!("{:?}", arg_dims);
    let (arg_name,arg_table_id,_) = arguments[0];
    println!("123");
    let (out_table_id, _, _) = out;
    let out_table = block.get_table(out_table_id)?;
    println!("456");
    let mut out_brrw = out_table.borrow_mut();
    println!("456");
    println!("456");
    if arg_name == *ANGLE {
      match arg_dims[0] {
        TableShape::Scalar => {
          println!("1");
          let arg = block.get_arg_columns(arguments)?[0].clone();
          println!("2");
          out_brrw.resize(1,1);
          out_brrw.set_kind(ValueKind::F32);
          println!("e3");
          if let Column::F32(out_col) = out_brrw.get_column_unchecked(0) {
            match arg {
              (_,Column::F32(col),ColumnIndex::Index(_)) |
              (_,Column::F32(col),ColumnIndex::All) => block.plan.push(
                MathSinRadVV{col: (col.clone(),0,0), out: out_col.clone()}
              ),
              (_,Column::Angle(col),ColumnIndex::All) => block.plan.push(
                MathSinRadVV{col: (col.clone(),0,0), out: out_col.clone()}
              ),
              x => {
                println!("{:?}",x);
                return Err(MechError{id: 1348, kind: MechErrorKind::GenericError(format!("{:?}",x))});}
            }
          }
        }
        TableShape::Column(rows) => {
          let arg = block.get_arg_columns(arguments)?[0].clone();
          out_brrw.resize(rows,1);
          if let Column::F32(out_col) = out_brrw.get_column_unchecked(0) {
            match arg {
              (_,Column::F32(col),ColumnIndex::All) => block.plan.push(MathSinRadVV{col: (col.clone(),0,col.len()), out: out_col.clone()}),
              x => {return Err(MechError{id: 1349, kind: MechErrorKind::GenericError(format!("{:?}",x))});}
            }
          }
        }
        x => {return Err(MechError{id: 1350, kind: MechErrorKind::GenericError(format!("{:?}",x))});},
      }
    } else {
      return Err(MechError{id: 1351, kind: MechErrorKind::UnknownFunctionArgument(arg_name)});
    }
    Ok(())
  }
}

export_mech_function!(math_sin, math_sin_reg);

extern "C" fn math_sin_reg(registrar: &mut dyn MechFunctionRegistrar) {
  registrar.register_mech_function(hash_str("math/sin"),Box::new(MathSin{}));
}
/*
#[no_mangle]
pub extern "C" fn math_cos(arguments:  &mut Vec<Rc<RefCell<Argument>>>) {
  let arg = arguments[0].borrow();
  let in_arg_name = arg.name;
  let vi = arg.iterator.clone();
  let mut out = arguments.last().unwrap().borrow().iterator.clone();
  if in_arg_name == *ANGLE {
    out.resize(vi.rows(),vi.columns());
    let mut flag: bool = false;
    for ((value, changed), out_ix) in vi.clone().zip(out.linear_index_iterator()) {
      match value.as_quantity() {
        Some(x) => {
          let result = match fmodf(x.as_f32().unwrap(), 360.0) {
            0.0 => 1.0,
            90.0 => 0.0,
            180.0 => -1.0,
            270.0 => 0.0,
            _ => cosf(x.as_f32().unwrap() * PI / 180.0),
          };
          out.set_unchecked_linear(out_ix, Value::from_f32(result));
        },
        _ => (), // TODO Alert user that there was an error
      }
    }
  } else {
    // TODO Warn about unknown argument
  }
}

#[no_mangle]
pub extern "C" fn math_round(arguments:  &mut Vec<Rc<RefCell<Argument>>>) {
  let arg = arguments[0].borrow();
  let in_arg_name = arg.name;
  let vi = arg.iterator.clone();
  let mut out = arguments.last().unwrap().borrow().iterator.clone();
  if in_arg_name == *TABLE {
    out.resize(vi.rows(),vi.columns());
    let mut flag: bool = false;
    for ((value, changed), out_ix) in vi.clone().zip(out.linear_index_iterator()) {
      match value.as_f32() {
        Some(x) => {
          out.set_unchecked_linear(out_ix, Value::from_f32(roundf(x)));
        },
        _ => (), // TODO Alert user that there was an error
      }
    }
  } else {
    // TODO Warn about unknown argument
  }
}

#[no_mangle]
pub extern "C" fn math_floor(arguments:  &mut Vec<Rc<RefCell<Argument>>>) {
  let arg = arguments[0].borrow();
  let in_arg_name = arg.name;
  let vi = arg.iterator.clone();
  let mut out = arguments.last().unwrap().borrow().iterator.clone();
  if in_arg_name == *TABLE {
    out.resize(vi.rows(),vi.columns());
    let mut flag: bool = false;
    for ((value, changed), out_ix) in vi.clone().zip(out.linear_index_iterator()) {
      match value.as_f32() {
        Some(x) => {
          out.set_unchecked_linear(out_ix, Value::from_f32(floorf(x)));
        },
        _ => (), // TODO Alert user that there was an error
      }
    }
  } else {
    // TODO Warn about unknown argument
  }
}
*/