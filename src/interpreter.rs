use regex::Regex;

fn interpreter(code: String) {
  let symbols: &str = "+\\-*/\\^%=><\\\\&|@;:,.!?'\\\"~`";
  let num_regex: Regex = Regex::new(r"((((\d+(_(\d+))*)+\.)?(\d+(_(\d+))*)+i?([eE]\d+)?)|(0b[01]*\.?[01]+i?)|(0o[0-7]*\.?[0-7]+i?)|(0x[0-9a-fA-F]*\.?[0-9a-fA-F]+i?))").unwrap();
  let identifier_regex: Regex = Regex::new(&(r"([^0-9".to_owned()+symbols+r"()\[\]{}#][^\s"+symbols+r"()\[\]{}]*)")).unwrap();
  let string_regex: Regex = Regex::new(r"(`.*`)").unwrap();
  let symbol_regex: Regex = Regex::new(&("([{".to_owned()+symbols+"}]+)")).unwrap();
  let comment_regex: Regex = Regex::new(r"((##.*##)|(#.*\n))").unwrap();
  let space_regex: Regex = Regex::new(r"(\s*\n\s*)").unwrap();
  let token_regex: Regex = Regex::new(num_regex.as_str()+"|"+identifier_regex.as_str()+"|"+string_regex.as_str()+"|"+symbol_regex.as_str()+"|"+comment_regex.as_str()+"|"+space_regex.as_str());

  let matches: Vec<_> = re.find_iter(hay).map(|m| m.as_str()).collect();
}
