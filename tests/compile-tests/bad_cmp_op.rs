use gurobi::*;
mod utils;

fn main() -> Result<()> {
  let _g = gag::Gag::stdout();
  create_model!(_g, m, x, y, z);
  c!(x + y != 1 - z);
  c!(x + y < 1 - z);
  c!(x + y > 1 - z);
  Ok(())
}
