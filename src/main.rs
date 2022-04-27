mod lib;

fn main() {
  let result = lib::mc_compliance::validate_host();
  println!(" Result {:?}", result);
}
