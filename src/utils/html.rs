use regex::Regex;

pub fn html2text(html: &str) -> String{
  let empty_reg = Regex::new(r"<style([\s\S]*?)</style>|<script([\s\S]*?)</script>|<[^>]+>").expect("empty regex failure");
  let li_reg = Regex::new(r"<li([\s\S]*?)>").expect("li regex failure");
  let line_reg = Regex::new(r"</div>|</li>|</ul>|</p>|<br\s*[/]?>").expect("line regex failure");

  let result = empty_reg.replace_all(html, "");
  let result = li_reg.replace_all(&result, "  *  ");
  let result = line_reg.replace_all(&result, "\n");

  result.to_string()
}

pub fn clean_xml(text: &str) -> String {
  // let not_safe_xml_reg = Regex::new(r"[^\x09\x0A\x0D\x20-\xFF\x85\xA0-\uD7FF\uE000-\uFDCF\uFDE0-\uFFFD]").expect("xml regex failure");
  // let not_safe_xml_reg = Regex::new(r"").expect("xml regex failure");
  let result = text.to_string()
                        .replace("&", "&amp;")
                        .replace("<", "&lt;")
                        .replace(">", "&gt;")
                        .replace("\"", "&quot;")
                        .replace("'", "&#039;");
  result
}