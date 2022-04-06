use super::*;

pub fn phase2(code: &str) -> String {
  let mut code = code.replace("\\\n", "");
  if !code.ends_with("\n") {
    code.push('\n');
  }
  code
}
