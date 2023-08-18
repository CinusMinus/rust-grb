#![deny(deprecated)]

use grb::prelude::*;
use grb::callback::*;

fn callback(w: Where) -> CbResult {
  match w {
    Where::MIPSol(ctx) => {
      ctx.add_cut(c!(0 == 0))?;
    },
    Where::MIPNode(ctx) => {
      ctx.add_cut(c!(0 == 0))?;
    }
    _ => unimplemented!()
  }
  Ok(())
}

fn main() {}

