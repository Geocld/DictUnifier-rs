use regex::Regex;
use lazy_static::lazy_static;

pub fn html2text(html: &str) -> String{
  lazy_static!{
    static ref EMPTY_REGEX: Regex = Regex::new(r"<style([\s\S]*?)</style>|<script([\s\S]*?)</script>|<[^>]+>").expect("empty regex failure");
    static ref LI_REGEX: Regex = Regex::new(r"<li([\s\S]*?)>").expect("li regex failure");
    static ref LINE_REGEX: Regex = Regex::new(r"</div>|</li>|</ul>|</p>|<br\s*[/]?>").expect("line regex failure");
  }

  let result = EMPTY_REGEX.replace_all(html, "");
  let result = LI_REGEX.replace_all(&result, "  *  ");
  let result = LINE_REGEX.replace_all(&result, "\n");

  result.to_string()
}

// clean un safe xml: https://www.w3.org/TR/xml/#charsets
pub fn clean_xml(text: &str) -> String {
  lazy_static!{
    static ref NOT_SAFE_XML_REGEX: Regex = Regex::new(r"[^\x09\x0A\x0D\x20-\xFF\x85\xA0-\uD7FF\uE000-\uFDCF\uFDE0-\uFFFD]").expect("xml regex failure");
  }
  let safe_text = NOT_SAFE_XML_REGEX.replace_all(text, "");
  let result = safe_text.to_string()
                        .replace("&", "&amp;")
                        .replace("<", "&lt;")
                        .replace(">", "&gt;")
                        .replace("\"", "&quot;")
                        .replace("'", "&#039;");
  result
}