mod deeplll;
mod parse;

use deeplll::{deep_lll, lll, pot_lll, s2_lll, mu::Mu};
use parse::matrix_parse;

use ndarray::prelude::*;
use rug::Rational;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use std::time::{Instant};

macro_rules! measure {
  ($str: expr, $result_f: expr, $x: expr) => {
    {
      let start = Instant::now();
      let result = $x;
      let end = start.elapsed();
      writeln!($result_f, "{}: {}.{:03} sec", $str, end.as_secs(), end.subsec_nanos() / 1_000_000)?;
      result
    }
  };
}

fn experiment_mat(mat_path_str: &str) {
  let path = Path::new(mat_path_str);
  let b = matrix_parse(path);
  for ndim in &[10, 15, 20, 25, 30] {
    let path_str_base = mat_path_str.split("/").last().unwrap().split(".").next().unwrap();
    for rat in &[Rational::from(1), Rational::from((99, 100))] {

      macro_rules! experiment {
        ($dir_name: expr, $f: expr) => {
          let result_path_str = format!("results/{}/{}dim{}delta{}.txt", $dir_name, path_str_base, *ndim, rat.to_f32());
          eprintln!("\n{}", &result_path_str);
          experiment_unit(b.slice(s![0..(*ndim), 0..(*ndim)]), &result_path_str, rat.to_owned(), $f).unwrap();
        };
      }

      // experiment!("deeplll", deep_lll);

      experiment!("lll", lll);

      experiment!("potlll", pot_lll);

      experiment!("s2lll", s2_lll);
    }
  }
}

fn experiment_unit<T: std::fmt::Debug>(b: ArrayView2<Rational>, result_path_str: &str, delta: Rational, f: impl Fn(Array2<Rational>, Rational, bool, usize) -> (
  Array2<Rational>,
  Array1<Rational>,
  Mu,
  Vec<T>,
  usize,
)) -> std::io::Result<()> {
  let mut result_f = File::create(result_path_str)?;
  let (new_b, v, mu, hist, cnt) =
    measure!(result_path_str, result_f, f(b.to_owned(), delta, true, 100));
  writeln!(result_f, "b: {:?}\nv_norms: {:?}\n{:?}\n(hist.len, cnt): {:?}\n{:?}", new_b, v, mu, (hist.len(), cnt), hist)?;
  Ok(())
}

fn main() {
  for i in 0..5 {
    let mat_path_str = format!("matrices/svp/svpchallengedim40seed{}.txt", i);
    experiment_mat(&mat_path_str);
  }
}
