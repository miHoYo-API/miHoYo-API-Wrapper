use std::collections::HashMap;
use once_cell::sync::Lazy;



pub(crate) static LANGUAGES: Lazy<HashMap<&'static str,
 &'static str>> = Lazy::new(||
     HashMap::from(
       [
         ("zh-cn", "简体中文"),
         ("zh-tw", "繁體中文"),
         ("de-de", "Deutsch"),
         ("en-us", "English"),
         ("es-es", "Español"),
         ("fr-fr", "Français"),
         ("id-id", "Indonesia"),
         ("it-it", "Italiano"),
         ("ja-jp", "日本語"),
         ("ko-kr", "한국어"),
         ("pt-pt", "Português"),
         ("ru-ru", "Pусский"),
         ("th-th", "ภาษาไทย"),
         ("vi-vn", "Tiếng Việt"),
         ("tr-tr", "Türkçe"),
       ])
);

