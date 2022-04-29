mod lib;

fn main() {
  let result = lib::validate_host();
  println!(" Result {:?}", result);
}
