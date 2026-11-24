//! Icons: Lucide SVGs by file name, from gpui-kit's set plus any the app embeds.

use std::borrow::Cow;

use gpui_kit::*;

/// An icon by its Lucide file name (`"corner-up-left"`), tinted with the text colour.
pub fn icon(name: &'static str, size: f32, color: Hsla) -> Svg {
    svg().path(format!("icons/{name}.svg")).size(px(size)).flex_none().text_color(color)
}

/// The app's assets: its own (`E`, e.g. a set made with `gpui_kit::assets::icon_assets!`),
/// then gpui-kit's. For `gpui_kit::application().with_assets(..)`.
pub struct Assets<E>(pub E);

impl<E: AssetSource> AssetSource for Assets<E> {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(b) = self.0.load(path)? {
            return Ok(Some(b));
        }
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut v = gpui_kit::assets::Assets.list(path)?;
        v.extend(self.0.list(path)?);
        v.sort();
        v.dedup();
        Ok(v)
    }
}
