use lazy_regex::regex_replace;

pub fn html2text(html: &str) -> String {
  // let text = regex_replace!(r#"<style([\s\S]*?)</style>"#i, html, |_, name, digit| format!("F<{}>{}", name, digit));
  // text = regex_replace_all!(r#"<script([\s\S]*?)<\/script>"#gi, "");
  // text = regex_replace_all!(r#"<\/div>"#gi, "\n");
  // text = regex_replace_all!(r#"<\/li>"#gi, "\n");
  // text = regex_replace_all!(r#"<li>"#gi, "  *  ");
  // text = regex_replace_all!(r#"<\/ul>"#gi, "\n");
  // text = regex_replace_all!(r#"<\/p>"#gi, "\n");
  // text = regex_replace_all!(r#"<br\s*[\/]?>"#gi, "\n");
  // text = regex_replace_all!(r#"<[^>]+>"#gi, "");
  html.to_string()
}