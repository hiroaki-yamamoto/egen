use ::console::Style;
use ::similar::{ChangeTag, TextDiff};

pub fn assert_txt_eq(result: &str, correct: &str) {
  let diff = TextDiff::from_lines(correct, result)
    .iter_all_changes()
    .map(|change| {
      let (sign, style) = match change.tag() {
        ChangeTag::Insert => ("+", Style::new().green()),
        ChangeTag::Delete => ("-", Style::new().red()),
        ChangeTag::Equal => (" ", Style::new()),
      };
      format!("{}{}", style.apply_to(sign), style.apply_to(change))
    })
    .collect::<Vec<String>>()
    .join("");
  assert_eq!(result, correct, "{}", diff);
}
