//! Safe Rust over the ArkUI NDK, via `shim.c`.
//!
//! The whole widget tree is built here — created, configured, mounted and
//! event-wired from native code. ArkTS never sees an individual widget; it only
//! hands over one `NodeContent` slot at startup.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

pub type NodeHandle = *mut c_void;
pub type NodeContentHandle = *mut c_void;

extern "C" {
    fn octoscript_arkui_init() -> c_int;
    fn octoscript_create_node(ty: c_int) -> NodeHandle;
    fn octoscript_dispose_node(n: NodeHandle);
    fn octoscript_add_child(parent: NodeHandle, child: NodeHandle) -> c_int;
    fn octoscript_set_string(n: NodeHandle, attr: c_int, s: *const c_char) -> c_int;
    fn octoscript_set_i32_string(n: NodeHandle, attr: c_int, v: i32, s: *const c_char) -> c_int;
    fn octoscript_set_f32(n: NodeHandle, attr: c_int, v: f32) -> c_int;
    fn octoscript_get_f32(n: NodeHandle, attr: c_int, index: c_int, out: *mut f32) -> c_int;
    fn octoscript_get_string(n: NodeHandle, attr: c_int, out: *mut c_char, cap: c_int) -> c_int;
    fn octoscript_set_i32(n: NodeHandle, attr: c_int, v: i32) -> c_int;
    fn octoscript_set_u32(n: NodeHandle, attr: c_int, v: u32) -> c_int;
    fn octoscript_set_f32v(n: NodeHandle, attr: c_int, v: *const f32, count: c_int) -> c_int;
    fn octoscript_set_gradient(
        n: NodeHandle,
        attr: c_int,
        dir: c_int,
        colors: *const u32,
        stops: *const f32,
        count: c_int,
    ) -> c_int;
    fn octoscript_layout_size(n: NodeHandle, w: *mut i32, h: *mut i32) -> c_int;
    fn octoscript_content_add(content: NodeContentHandle, root: NodeHandle) -> c_int;
    fn octoscript_register_event(n: NodeHandle, event_type: c_int, id: i32) -> c_int;
    fn octoscript_animate(
        anchor: NodeHandle,
        duration_ms: c_int,
        curve: c_int,
        update: extern "C" fn(*mut std::ffi::c_void),
        user: *mut std::ffi::c_void,
    ) -> c_int;
}

/// ArkUI enum values, evaluated by the C++ compiler against the real headers
/// (see `shim.cpp`). Hand-transcribing them produced a SIGSEGV: the enums are
/// mostly implicit and the per-component blocks are `1000 * ARKUI_NODE_X + n`,
/// so a single wrong value silently addresses a different attribute.
mod raw {
    extern "C" {
        pub static octoscript_a_id: i32;
        pub static octoscript_a_width: i32;
        pub static octoscript_a_height: i32;
        pub static octoscript_a_bg: i32;
        pub static octoscript_a_padding: i32;
        pub static octoscript_a_margin: i32;
        pub static octoscript_a_border_width: i32;
        pub static octoscript_a_border_radius: i32;
        pub static octoscript_a_blur: i32;
        pub static octoscript_a_border_color: i32;
        pub static octoscript_a_checkbox_select: i32;
        pub static octoscript_a_checkbox_color: i32;
        pub static octoscript_a_radio_checked: i32;
        pub static octoscript_a_toggle_value: i32;
        pub static octoscript_a_toggle_color: i32;
        pub static octoscript_a_textpicker_range: i32;
        pub static octoscript_a_position: i32;
        pub static octoscript_a_hit_test: i32;
        pub static octoscript_a_text_shadow: i32;
        pub static octoscript_a_translate: i32;
        pub static octoscript_a_scale: i32;
        pub static octoscript_a_zindex: i32;
        pub static octoscript_a_clip: i32;
        pub static octoscript_a_stack_align: i32;
        pub static octoscript_a_linear_gradient: i32;
        pub static octoscript_a_image_src: i32;
        pub static octoscript_a_image_fit: i32;
        pub static octoscript_a_alignment: i32;
        pub static octoscript_a_opacity: i32;
        pub static octoscript_a_visibility: i32;
        pub static octoscript_a_text_content: i32;
        pub static octoscript_a_font_color: i32;
        pub static octoscript_a_font_size: i32;
        pub static octoscript_a_font_weight: i32;
        pub static octoscript_a_text_align: i32;
        pub static octoscript_a_button_label: i32;
        pub static octoscript_a_progress_value: i32;
        pub static octoscript_a_progress_total: i32;
        pub static octoscript_a_input_placeholder: i32;
        pub static octoscript_a_input_text: i32;
        pub static octoscript_a_width_percent: i32;
        pub static octoscript_a_height_percent: i32;
        pub static octoscript_a_font_family: i32;
        pub static octoscript_a_row_align: i32;
        pub static octoscript_a_row_justify: i32;
        pub static octoscript_a_col_align: i32;
        pub static octoscript_a_col_justify: i32;
        pub static octoscript_a_shadow: i32;
        pub static octoscript_a_layout_weight: i32;
        pub static octoscript_a_slider_value: i32;
        pub static octoscript_a_slider_min: i32;
        pub static octoscript_a_slider_max: i32;
        pub static octoscript_a_slider_selected: i32;
        pub static octoscript_a_slider_block: i32;
        pub static octoscript_a_slider_track: i32;
        pub static octoscript_a_checkbox_shape: i32;
        pub static octoscript_a_loading_color: i32;
        pub static octoscript_a_progress_color: i32;
        pub static octoscript_a_scroll_offset: i32;

        pub static octoscript_t_text: i32;
        pub static octoscript_t_image: i32;
        pub static octoscript_t_toggle: i32;
        pub static octoscript_t_loading: i32;
        pub static octoscript_t_input: i32;
        pub static octoscript_t_textarea: i32;
        pub static octoscript_t_button: i32;
        pub static octoscript_t_progress: i32;
        pub static octoscript_t_checkbox: i32;
        pub static octoscript_t_datepicker: i32;
        pub static octoscript_t_slider: i32;
        pub static octoscript_t_radio: i32;
        pub static octoscript_t_stack: i32;
        pub static octoscript_t_scroll: i32;
        pub static octoscript_t_list: i32;
        pub static octoscript_t_column: i32;
        pub static octoscript_t_row: i32;
        pub static octoscript_t_flex: i32;
        pub static octoscript_t_timepicker: i32;
        pub static octoscript_t_textpicker: i32;
        pub static octoscript_t_swiper: i32;
        pub static octoscript_t_grid: i32;
        pub static octoscript_t_waterflow: i32;
        pub static octoscript_t_refresh: i32;
        pub static octoscript_e_click: i32;
        pub static octoscript_e_touch: i32;
        pub static octoscript_e_appear: i32;
        pub static octoscript_e_input_change: i32;
        pub static octoscript_e_slider_change: i32;
        pub static octoscript_e_did_scroll: i32;
    }
}

macro_rules! arkui_consts {
    ($($fn_name:ident => $sym:ident),* $(,)?) => {
        $(#[inline] pub fn $fn_name() -> i32 { unsafe { raw::$sym } })*
    };
}

/// Node types.
pub mod ty {
    use super::raw;
    arkui_consts! {
        text => octoscript_t_text, image => octoscript_t_image, toggle => octoscript_t_toggle,
        loading => octoscript_t_loading, input => octoscript_t_input, textarea => octoscript_t_textarea,
        button => octoscript_t_button, progress => octoscript_t_progress, checkbox => octoscript_t_checkbox,
        datepicker => octoscript_t_datepicker, slider => octoscript_t_slider, radio => octoscript_t_radio,
        stack => octoscript_t_stack, scroll => octoscript_t_scroll, list => octoscript_t_list,
        column => octoscript_t_column, row => octoscript_t_row, flex => octoscript_t_flex,
        timepicker => octoscript_t_timepicker, textpicker => octoscript_t_textpicker,
        swiper => octoscript_t_swiper, grid => octoscript_t_grid,
        waterflow => octoscript_t_waterflow, refresh => octoscript_t_refresh,
    }
}

/// Attribute ids.
pub mod attr {
    use super::raw;
    arkui_consts! {
        id => octoscript_a_id,
        width => octoscript_a_width, height => octoscript_a_height, bg => octoscript_a_bg,
        padding => octoscript_a_padding, margin => octoscript_a_margin,
        border_width => octoscript_a_border_width, border_radius => octoscript_a_border_radius,
        blur => octoscript_a_blur,
        border_color => octoscript_a_border_color, alignment => octoscript_a_alignment,
        opacity => octoscript_a_opacity, visibility => octoscript_a_visibility,
        text_content => octoscript_a_text_content, font_color => octoscript_a_font_color,
        font_size => octoscript_a_font_size, font_weight => octoscript_a_font_weight,
        text_align => octoscript_a_text_align, button_label => octoscript_a_button_label,
        progress_value => octoscript_a_progress_value, progress_total => octoscript_a_progress_total,
        input_placeholder => octoscript_a_input_placeholder, input_text => octoscript_a_input_text,
        checkbox_select => octoscript_a_checkbox_select,
        checkbox_color => octoscript_a_checkbox_color,
        radio_checked => octoscript_a_radio_checked,
        toggle_value => octoscript_a_toggle_value,
        toggle_color => octoscript_a_toggle_color,
        image_src => octoscript_a_image_src,
        image_fit => octoscript_a_image_fit,
        textpicker_range => octoscript_a_textpicker_range,
        position => octoscript_a_position, hit_test => octoscript_a_hit_test, text_shadow => octoscript_a_text_shadow, translate => octoscript_a_translate,
        scale => octoscript_a_scale, zindex => octoscript_a_zindex, clip => octoscript_a_clip,
        stack_align => octoscript_a_stack_align,
        linear_gradient => octoscript_a_linear_gradient,
        width_percent => octoscript_a_width_percent,
        height_percent => octoscript_a_height_percent,
        font_family => octoscript_a_font_family,
        row_align => octoscript_a_row_align,
        row_justify => octoscript_a_row_justify,
        col_align => octoscript_a_col_align,
        col_justify => octoscript_a_col_justify,
        shadow => octoscript_a_shadow,
        layout_weight => octoscript_a_layout_weight,
        slider_value => octoscript_a_slider_value,
        slider_min => octoscript_a_slider_min,
        slider_max => octoscript_a_slider_max,
        slider_selected => octoscript_a_slider_selected,
        slider_block => octoscript_a_slider_block,
        slider_track => octoscript_a_slider_track,
        checkbox_shape => octoscript_a_checkbox_shape,
        loading_color => octoscript_a_loading_color,
        progress_color => octoscript_a_progress_color,
        scroll_offset => octoscript_a_scroll_offset,
    }
}

/// Event ids.
pub mod event {
    use super::raw;
    arkui_consts! { click => octoscript_e_click, touch => octoscript_e_touch, appear => octoscript_e_appear,
    input_change => octoscript_e_input_change, slider_change => octoscript_e_slider_change,
    did_scroll => octoscript_e_did_scroll }
}

/// Initialise the node API. Safe to call repeatedly.
pub fn init() -> Result<(), &'static str> {
    if unsafe { octoscript_arkui_init() } == 0 {
        Ok(())
    } else {
        Err("OH_ArkUI_GetModuleInterface(ARKUI_NATIVE_NODE) failed")
    }
}

/// An ArkUI component owned by native code.
pub struct Node {
    raw: NodeHandle,
    /// Children are kept alive here — ArkUI does not own them for us, and
    /// dropping a parent must not leave dangling child handles.
    children: Vec<Node>,
}

impl Node {
    pub fn new(ty: i32) -> Option<Self> {
        let raw = unsafe { octoscript_create_node(ty as c_int) };
        if raw.is_null() {
            return None;
        }
        Some(Self {
            raw,
            children: Vec::new(),
        })
    }

    /// Set an i32 attribute on a handle this `Node` does not own.
    ///
    /// # Safety
    /// `raw` must be a live node handle.
    pub unsafe fn set_i32_raw(raw: NodeHandle, a: i32, v: i32) {
        unsafe { octoscript_set_i32(raw, a, v) };
    }

    /// Set a string attribute on a handle this `Node` does not own.
    ///
    /// # Safety
    /// `raw` must be a live node handle.
    pub unsafe fn set_string_raw(raw: NodeHandle, a: i32, v: &str) {
        if let Ok(c) = std::ffi::CString::new(v) {
            unsafe { octoscript_set_string(raw, a, c.as_ptr()) };
        }
    }

    /// Read a string attribute, or `None` if it has none.
    ///
    /// # Safety
    /// `raw` must be a live node handle.
    pub unsafe fn get_string(raw: NodeHandle, a: i32) -> Option<String> {
        let mut buf = [0u8; 512];
        let n = unsafe {
            octoscript_get_string(raw, a, buf.as_mut_ptr() as *mut c_char, buf.len() as c_int)
        };
        if n < 0 {
            return None;
        }
        String::from_utf8(buf[..n as usize].to_vec()).ok()
    }

    /// Read one f32 out of an attribute, or `None` if it has none.
    ///
    /// # Safety
    /// `raw` must be a live node handle.
    pub unsafe fn get_f32(raw: NodeHandle, a: i32, index: i32) -> Option<f32> {
        let mut out = 0.0f32;
        if unsafe { octoscript_get_f32(raw, a, index, &mut out) } == 0 {
            Some(out)
        } else {
            None
        }
    }

    pub fn raw(&self) -> NodeHandle {
        self.raw
    }

    pub fn child(mut self, c: Node) -> Self {
        unsafe { octoscript_add_child(self.raw, c.raw) };
        self.children.push(c);
        self
    }

    pub fn text(self, s: &str) -> Self {
        self.string_attr(attr::text_content(), s)
    }

    pub fn label(self, s: &str) -> Self {
        self.string_attr(attr::button_label(), s)
    }

    pub fn string_attr(self, a: i32, s: &str) -> Self {
        if let Ok(c) = CString::new(s) {
            unsafe { octoscript_set_string(self.raw, a, c.as_ptr()) };
        }
        self
    }

    /// For attributes that carry a number *and* a string in the same item —
    /// the text picker's range being the one this exists for.
    pub fn i32_string_attr(self, a: i32, v: i32, s: &str) -> Self {
        if let Ok(c) = CString::new(s) {
            unsafe { octoscript_set_i32_string(self.raw, a, v, c.as_ptr()) };
        }
        self
    }

    /// Set a float attribute on a raw handle, bypassing the builder. Only for
    /// the benchmark, which needs to time `setAttribute` on its own without a
    /// `Node` move in the loop.
    ///
    /// # Safety
    /// `raw` must be a live node handle owned by the caller.
    pub unsafe fn set_f32_attr_raw(raw: NodeHandle, a: i32, v: f32) {
        unsafe { octoscript_set_f32(raw, a, v) };
    }

    pub fn f32_attr(self, a: i32, v: f32) -> Self {
        unsafe { octoscript_set_f32(self.raw, a, v) };
        self
    }

    pub fn i32_attr(self, a: i32, v: i32) -> Self {
        unsafe { octoscript_set_i32(self.raw, a, v) };
        self
    }

    pub fn u32_attr(self, a: i32, v: u32) -> Self {
        unsafe { octoscript_set_u32(self.raw, a, v) };
        self
    }

    /// Set an f32 vector on a handle this `Node` does not own.
    ///
    /// The scroll-offset restore needs it: by the time the offset is written
    /// the tree is mounted and owned by the app, so there is no `Node` to
    /// consume.
    ///
    /// # Safety
    /// `raw` must be a live node handle.
    pub unsafe fn set_f32v_raw(raw: NodeHandle, a: i32, v: &[f32]) {
        unsafe { octoscript_set_f32v(raw, a, v.as_ptr(), v.len() as c_int) };
    }

    pub fn f32v_attr(self, a: i32, v: &[f32]) -> Self {
        unsafe { octoscript_set_f32v(self.raw, a, v.as_ptr(), v.len() as c_int) };
        self
    }

    /// A linear gradient over this node's background.
    ///
    /// `dir` is `ArkUI_LinearGradientDirection`; `stops` run 0..1. Colours are
    /// 0xAARRGGBB like everywhere else here.
    pub fn gradient(self, dir: i32, colors: &[u32], stops: &[f32]) -> Self {
        let n = colors.len().min(stops.len());
        unsafe {
            octoscript_set_gradient(
                self.raw,
                attr::linear_gradient(),
                dir,
                colors.as_ptr(),
                stops.as_ptr(),
                n as c_int,
            )
        };
        self
    }

    // --- conveniences the backend leans on ---
    pub fn width(self, v: f32) -> Self {
        self.f32_attr(attr::width(), v)
    }
    pub fn height(self, v: f32) -> Self {
        self.f32_attr(attr::height(), v)
    }
    pub fn bg(self, argb: u32) -> Self {
        self.u32_attr(attr::bg(), argb)
    }
    pub fn font_color(self, argb: u32) -> Self {
        self.u32_attr(attr::font_color(), argb)
    }
    pub fn font_size(self, v: f32) -> Self {
        self.f32_attr(attr::font_size(), v)
    }
    pub fn font_weight(self, w: i32) -> Self {
        self.i32_attr(attr::font_weight(), w)
    }
    pub fn padding(self, p: f32) -> Self {
        self.f32v_attr(attr::padding(), &[p, p, p, p])
    }
    pub fn radius(self, r: f32) -> Self {
        self.f32v_attr(attr::border_radius(), &[r, r, r, r])
    }
    pub fn margin(self, m: f32) -> Self {
        self.f32v_attr(attr::margin(), &[m, m, m, m])
    }

    pub fn on_event(self, event_type: i32, id: i32) -> Self {
        unsafe { octoscript_register_event(self.raw, event_type, id) };
        self
    }

    /// Mount and hand the node back, so the caller can detach it later on a
    /// rebuild. `mount` forgets the tree; this keeps ownership.
    ///
    /// # Safety
    /// `content` must be the live `NodeContent` handle ArkTS passed to `mount`.
    pub unsafe fn mount_keep(self, content: NodeContentHandle) -> Result<Node, &'static str> {
        let r = unsafe { octoscript_content_add(content, self.raw) };
        if r == 0 {
            Ok(self)
        } else {
            Err("OH_ArkUI_NodeContent_AddNode failed")
        }
    }

    /// Mount as the root of an ArkTS-provided slot. Consumes self and leaks it
    /// deliberately: the tree must outlive this call for the page's lifetime.
    ///
    /// # Safety
    /// `content` must be the live `NodeContent` handle ArkTS passed to `mount`.
    pub unsafe fn mount(self, content: NodeContentHandle) -> Result<(), &'static str> {
        let r = unsafe { octoscript_content_add(content, self.raw) };
        std::mem::forget(self);
        if r == 0 {
            Ok(())
        } else {
            Err("OH_ArkUI_NodeContent_AddNode failed")
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // Children first — ArkUI does not cascade.
        self.children.clear();
        unsafe { octoscript_dispose_node(self.raw) };
    }
}

/// The size a mounted node was actually given, in px.
///
/// # Safety
/// `raw` must be a live, mounted node handle.
pub unsafe fn layout_size(raw: NodeHandle) -> Option<(i32, i32)> {
    let (mut w, mut h) = (0i32, 0i32);
    if unsafe { octoscript_layout_size(raw, &mut w, &mut h) } == 0 && w > 0 && h > 0 {
        Some((w, h))
    } else {
        None
    }
}

/// `ArkUI_AnimationCurve::ARKUI_CURVE_EASE_OUT`, which is what Wonderous uses
/// for the moves this animates.
pub const CURVE_EASE_OUT: i32 = 3;

/// Run `f`, tweening whatever it changes.
///
/// `anchor` only has to be some node that is mounted — it is where the UI
/// context is read from, not what gets animated. Everything `f` touches moves.
///
/// If the animation module is unavailable `f` still runs, unanimated: a screen
/// that does not tween is a much smaller problem than one that does not update.
///
/// # Safety
/// `anchor` must be a live, mounted node handle.
pub unsafe fn animate<F: FnOnce()>(anchor: NodeHandle, duration_ms: i32, curve: i32, f: F) {
    extern "C" fn trampoline(user: *mut std::ffi::c_void) {
        // Exactly once, before octoscript_animate returns. The shim guards it with
        // a flag and calls it itself on every path where the animation could
        // not be set up -- including a rejected animateTo, which previously
        // leaked the box and dropped the update it was carrying.
        let b: Box<Box<dyn FnOnce()>> = unsafe { Box::from_raw(user as *mut _) };
        (*b)();
    }
    let boxed: Box<Box<dyn FnOnce()>> = Box::new(Box::new(f));
    unsafe {
        octoscript_animate(
            anchor,
            duration_ms,
            curve,
            trampoline,
            Box::into_raw(boxed) as *mut std::ffi::c_void,
        )
    };
}
