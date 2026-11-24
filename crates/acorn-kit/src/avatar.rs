//! Round avatars: the picture once it's ready, a quiet tinted disc until then. Pictures that
//! just arrived fade in; ones already seen (scrolled back into view) just appear.

use std::sync::Arc;

use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

use gpui_kit::*;

thread_local! {
    static READY_AT: RefCell<HashMap<usize, Instant>> = RefCell::new(HashMap::new());
}

/// Note that `img` just finished loading, so the next [`avatar`] (or anything asking
/// [`just_arrived`]) fades it in.
pub fn mark_ready(img: &Arc<RenderImage>) {
    let now = Instant::now();
    READY_AT.with(|m| {
        let mut m = m.borrow_mut();
        m.retain(|_, t| now.duration_since(*t) < Duration::from_secs(2));
        m.insert(Arc::as_ptr(img) as usize, now);
    });
}

/// Whether `img` finished loading a moment ago (so showing it should fade in; an image
/// already on screen before, scrolled back into view, should just appear).
pub fn just_arrived(img: &Arc<RenderImage>) -> bool {
    READY_AT.with(|m| m.borrow().get(&(Arc::as_ptr(img) as usize)).is_some_and(|t| t.elapsed() < Duration::from_millis(250)))
}

pub fn avatar(img: Option<Arc<RenderImage>>, size: f32, color: Hsla, _name: &str) -> Div {
    let d = div().size(px(size)).flex_none().rounded_full().overflow_hidden();
    match img {
        Some(i) => {
            let fresh = just_arrived(&i);
            let id = ElementId::Integer(Arc::as_ptr(&i) as u64);
            let pic = gpui_kit::img(ImageSource::Render(i)).object_fit(ObjectFit::Cover).size_full().rounded_full();
            if fresh { d.bg(color.opacity(0.25)).child(crate::motion::arrive(id, pic, 0.)) } else { d.child(pic) }
        }
        None => d.bg(color.opacity(0.25)),
    }
}

/// A presence-style dot at the avatar's lower right, ringed in `ring` (the surface colour).
pub fn with_status(av: Div, size: f32, dot: Hsla, ring: Hsla) -> Div {
    let d = (size * 0.3).max(10.);
    div().relative().flex_none().child(av).child(
        div()
            .absolute()
            .right(px(-2.))
            .bottom(px(-2.))
            .size(px(d + 4.))
            .rounded_full()
            .bg(ring)
            .flex()
            .items_center()
            .justify_center()
            .child(div().size(px(d)).rounded_full().bg(dot)),
    )
}
