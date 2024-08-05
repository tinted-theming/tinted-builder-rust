use tinted_builder::{SchemeType, Template, TemplateContent, TmScheme};

fn main() {
    let template = r#"<?xml version="1.0" encoding="UTF-8"?>
        <plist version="1.0">
        <dict>
            <key>name</key>
            <string>Extended Example Theme</string>
            <key>settings</key>
            <array>
                <dict>
                    <key>settings</key>
                    <dict>
                        <key>background</key>
                        <string>#0F1419</string>
                        <key>foreground</key>
                        <string>#E6E1CF</string>
                        <key>caret</key>
                        <string>#F29718</string>
                        <key>invisibles</key>
                        <string>#3E4B59</string>
                        <key>lineHighlight</key>
                        <string>#0D1016</string>
                        <key>selection</key>
                        <string>#243342</string>
                        <key>selectionBorder</key>
                        <string>#F29718</string>
                        <key>findHighlight</key>
                        <string>#FFE792</string>
                        <key>findHighlightForeground</key>
                        <string>#000000</string>
                        <key>activeGuide</key>
                        <string>#C0C0C0</string>
                        <key>bracketsForeground</key>
                        <string>#FFD700</string>
                        <key>bracketContentsForeground</key>
                        <string>#FFD700</string>
                        <key>highlight</key>
                        <string>#2D2D2D</string>
                        <key>guide</key>
                        <string>#3A3A3A</string>
                        <key>stackGuide</key>
                        <string>#404040</string>
                        <key>underline</key>
                        <string>false</string>
                        <key>tagsOptions</key>
                        <dict>
                            <key>underline</key>
                            <string>true</string>
                            <key>bold</key>
                            <string>true</string>
                        </dict>
                        <key>bracketsOptions</key>
                        <dict>
                            <key>underline</key>
                            <string>true</string>
                        </dict>
                        <key>bracketsContentOptions</key>
                        <dict>
                            <key>bold</key>
                            <string>true</string>
                        </dict>
                    </dict>
                </dict>
                <dict>
                    <key>name</key>
                    <string>Comment</string>
                    <key>scope</key>
                    <string>comment</string>
                    <key>settings</key>
                    <dict>
                        <key>foreground</key>
                        <string>#5C6773</string>
                        <key>fontStyle</key>
                        <string>italic</string>
                    </dict>
                </dict>
                <dict>
                    <key>name</key>
                    <string>String</string>
                    <key>scope</key>
                    <string>string</string>
                    <key>settings</key>
                    <dict>
                        <key>foreground</key>
                        <string>#AAD94C</string>
                        <key>background</key>
                        <string>#1E272C</string>
                        <key>fontStyle</key>
                        <string>italic</string>
                    </dict>
                </dict>
            </array>
            <key>uuid</key>
            <string>07320680-9C7D-4A82-8AA3-EB0B168C2E92</string>
        </dict>
        </plist>
    "#;
    let scheme_type = SchemeType::TmTheme(TmScheme {
        system: "TmTheme".to_string(),
    });
    let template = Template::new(TemplateContent::TmTheme(template.to_string())).unwrap();
    let output = template.render(&scheme_type).unwrap();
    // dbg!(output);
    // let scheme: Scheme = serde_yaml::from_str(&scheme_str).unwrap();
    // let scheme_type = SchemeType::Yaml(scheme);
    // let output = template.render(&scheme_type).unwrap();
}
