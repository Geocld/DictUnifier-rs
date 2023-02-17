pub fn generate_plist(dict_name: &str, dict_id: &str) -> String {
  format!("
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>zh-Hans</string>
    <key>CFBundleDisplayName</key>
    <string>{}</string>
    <key>CFBundleIdentifier</key>
    <string>com.apple.dictionary.{}</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>DCSDictionaryCopyright</key>
    <string>GNU General Public License</string>
    <key>DCSDictionaryManufacturerName</key>
    <string>stardict</string>
    <key>DCSDictionaryUseSystemAppearance</key>
    <true/>
</dict>
</plist>
  ", dict_name, dict_id, dict_name)
}

pub fn generate_css() -> String {
  String::from("@charset \"UTF-8\";
@namespace d url(http://www.apple.com/DTDs/DictionaryService-1.0.rng);

d|entry {
}

h1 {
    font-size: 150%;
}

html.apple_client-panel h1 {
    font-size: 100%;
}

h3 {
    font-size: 100%;
}

pre {
    /* overflow: auto; */
    white-space: pre-wrap;
}

html.apple_client-panel pre {
    white-space: normal;
}

span.column {
  display: block;
  border: solid 2px #c0c0c0;
  margin-left: 2em;
  margin-right: 2em;
  margin-top: 0.5em;
  margin-bottom: 0.5em;
  padding: 0.5em;
}

div.y:before {
    content: \"/\";
}

div.y:after {
    content: \"/\";
}

div.y {
    margin-top: 0.5em;
    margin-bottom: 0.5em;
    font-size: 120%;
    color: #333;
}

@media (prefers-dark-interface)
{
    body {
        color: white;
    }
}")
}