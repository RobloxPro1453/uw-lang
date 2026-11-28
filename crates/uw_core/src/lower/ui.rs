//! The HUD (CORE-v0 §5.3, §6; §4): every always-visible `ui.*` part of every feature
//! lives in ONE hidden presentation, `prsnt_uw_hud` (`prsntf_read_only | prsntf_manual_end_only`).
//!
//! - load: `set_fixed_point_multiplier 1000`, then each part's overlays (ids kept in `$uw_hud_<part>_*`
//!   globals), placed once; `presentation_set_duration 999999` keeps it up.
//! - run (every frame): each part's bound values (`value`, `max`, `visible`) are read again and the overlays
//!   updated.
//! - start: the mission dispatcher (`script_uw_mission_frame`) starts it whenever it is not active, which
//!   also brings it back after a full-screen window wiped it (maden ui-presentations, pattern 12).
//!
//! Screen units: x 0..1000, y 0..750 (Native presentations put the top edge at 750). A `mesh_white_plane`
//! overlay is 1/50 of a screen unit at size 1 (Native scales it by 50 per unit: `16500` → 330 units wide,
//! module_presentations.py, multiplayer stats table).

use crate::ast::{Expr, Ident, Span, TextPart};
use crate::{Arg, Error, Op};

use super::Ctx;

/// The HUD presentation (without the `prsnt_` prefix).
pub const HUD: &str = "uw_hud";
/// Screen size in presentation units.
pub const SCREEN_W: i64 = 1000;
pub const SCREEN_H: i64 = 750;
/// Gap between an anchored part and the screen edge.
pub const MARGIN: i64 = 20;
/// `mesh_white_plane` size per screen unit.
pub const PLANE_SCALE: i64 = 50;
/// Space between a bar and its label.
const LABEL_GAP: i64 = 4;

/// The overlays and per-frame updates of one part.
#[derive(Debug, Clone, Default)]
pub(crate) struct HudPart {
    pub name: String,
    pub load: Vec<Op>,
    pub run: Vec<Op>,
}

/// Properties a part accepts (CORE-v0 §6, the ones lowered in v0).
const BAR_PROPS: &[&str] = &["value", "max", "position", "size", "color", "alpha", "background", "label", "visible"];
const TEXT_PROPS: &[&str] = &["position", "color", "alpha", "label", "font_size", "visible"];

fn op(name: &str, args: Vec<Arg>) -> Op {
    Op { name: name.to_owned(), args, neg: false, this_or_next: false }
}

fn pos1() -> Arg {
    Arg::Name("pos1".to_owned())
}

/// `overlay_set_position`/`overlay_set_size` through `pos1` (screen units are the fixed-point values).
fn place(ops: &mut Vec<Op>, overlay: &Arg, setter: &str, x: Arg, y: Arg) {
    ops.push(op("position_set_x", vec![pos1(), x]));
    ops.push(op("position_set_y", vec![pos1(), y]));
    ops.push(op(setter, vec![overlay.clone(), pos1()]));
}

impl Ctx<'_> {
    /// `ui.<part> <name> { … }`.
    pub(crate) fn ui_item(&mut self, part: &Ident, name: &Ident, props: &[(Ident, Expr)], span: Span) -> Result<(), Error> {
        let allowed = match part.text.as_str() {
            "bar" => BAR_PROPS,
            "text" => TEXT_PROPS,
            "image" | "button" | "panel" => {
                return Err(self.error(part.span, &format!("`ui.{}` is not lowered yet (v0 has bar and text)", part.text)))
            }
            other => {
                return Err(self.error(part.span, &format!("unknown ui part `{other}` (bar, text, image, button, panel)")))
            }
        };
        let mut seen: Vec<&str> = Vec::new();
        for (key, _) in props {
            if !allowed.contains(&key.text.as_str()) {
                return Err(self.error(
                    key.span,
                    &format!("`ui.{}` has no property `{}` (it has: {})", part.text, key.text, allowed.join(", ")),
                ));
            }
            if seen.contains(&key.text.as_str()) {
                return Err(self.error(key.span, &format!("`{}` is given twice", key.text)));
            }
            seen.push(&key.text);
        }
        if self.hud.iter().any(|p| p.name == name.text) {
            return Err(self.error(name.span, &format!("a ui part named `{}` already exists", name.text)));
        }
        let prop = |key: &str| props.iter().find(|(k, _)| k.text == key).map(|(_, e)| e);
        let built = match part.text.as_str() {
            "bar" => self.bar(name, &prop, span)?,
            _ => self.text_part(name, &prop, span)?,
        };
        self.hud.push(built);
        Ok(())
    }

    fn bar<'e>(&mut self, name: &Ident, prop: &dyn Fn(&str) -> Option<&'e Expr>, span: Span) -> Result<HudPart, Error> {
        let Some(value) = prop("value") else {
            return Err(self.error(span, &format!("`ui.bar {}` needs `value = …`", name.text)));
        };
        let (w, h) = match prop("size") {
            Some(e) => self.ui_pair(e, "size")?,
            None => (200, 12),
        };
        let (x, y) = self.ui_position(prop("position"), w, h)?;
        let color = match prop("color") {
            Some(e) => self.ui_color(e)?,
            None => 0x3FA34D,
        };
        let background = match prop("background") {
            Some(e) => self.ui_color(e)?,
            None => 0x000000,
        };
        let alpha = match prop("alpha") {
            Some(e) => self.ui_alpha(e)?,
            None => 0xFF,
        };
        let plane = self.literal("mesh_white_plane", span)?;
        let g = |suffix: &str| Arg::Global(format!("$uw_hud_{}_{suffix}", name.text));
        let (bg, fill, label) = (g("bg"), g("fill"), g("label"));
        let mut load = Vec::new();
        load.push(op("create_mesh_overlay", vec![bg.clone(), plane.clone()]));
        load.push(op("overlay_set_color", vec![bg.clone(), Arg::Int(background)]));
        load.push(op("overlay_set_alpha", vec![bg.clone(), Arg::Int(alpha / 2)]));
        place(&mut load, &bg, "overlay_set_position", Arg::Int(x), Arg::Int(y));
        place(&mut load, &bg, "overlay_set_size", Arg::Int(w * PLANE_SCALE), Arg::Int(h * PLANE_SCALE));
        load.push(op("create_mesh_overlay", vec![fill.clone(), plane]));
        load.push(op("overlay_set_color", vec![fill.clone(), Arg::Int(color)]));
        load.push(op("overlay_set_alpha", vec![fill.clone(), Arg::Int(alpha)]));
        place(&mut load, &fill, "overlay_set_position", Arg::Int(x), Arg::Int(y));
        let mut overlays = vec![bg, fill.clone()];
        let mut live = None;
        if let Some(text) = prop("label") {
            let (prep, text, parts) = self.ui_text(text)?;
            live = parts;
            load.extend(prep);
            load.push(op("create_text_overlay", vec![label.clone(), text]));
            load.push(op("overlay_set_color", vec![label.clone(), Arg::Int(0xFFFFFF)]));
            place(&mut load, &label, "overlay_set_position", Arg::Int(x), Arg::Int(y + h + LABEL_GAP));
            overlays.push(label.clone());
        }

        // Every frame: fill width = w × value ÷ max, clamped to the bar.
        self.begin_block();
        let result = self
            .bar_run(value, prop("max"), &fill, w, h, prop("visible"), &overlays)
            .and_then(|()| self.live_text(&label, live.as_deref(), span));
        let run = self.end_block();
        result?;
        Ok(HudPart { name: name.text.clone(), load, run })
    }

    #[allow(clippy::too_many_arguments)]
    fn bar_run(
        &mut self,
        value: &Expr,
        max: Option<&Expr>,
        fill: &Arg,
        w: i64,
        h: i64,
        visible: Option<&Expr>,
        overlays: &[Arg],
    ) -> Result<(), Error> {
        let span = value.span();
        self.emit("try_begin", Vec::new(), span)?;
        let value = self.expr(value)?;
        let max = match max {
            Some(max) => self.expr(max)?,
            None => Arg::Int(100_000), // 100 in thousandths
        };
        let width = self.temp();
        self.emit("store_mul", vec![width.clone(), value, Arg::Int(w * PLANE_SCALE)], span)?;
        // A max of 0 would stop the frame's operations; the bar then shows empty.
        self.emit("try_begin", Vec::new(), span)?;
        self.emit("neq", vec![max.clone(), Arg::Int(0)], span)?;
        self.emit("val_div", vec![width.clone(), max], span)?;
        self.emit("else_try", Vec::new(), span)?;
        self.emit("assign", vec![width.clone(), Arg::Int(0)], span)?;
        self.emit("try_end", Vec::new(), span)?;
        self.emit("val_clamp", vec![width.clone(), Arg::Int(0), Arg::Int(w * PLANE_SCALE + 1)], span)?;
        self.emit("position_set_x", vec![pos1(), width], span)?;
        self.emit("position_set_y", vec![pos1(), Arg::Int(h * PLANE_SCALE)], span)?;
        self.emit("overlay_set_size", vec![fill.clone(), pos1()], span)?;
        self.emit("try_end", Vec::new(), span)?;
        self.visibility(visible, overlays)
    }

    fn text_part<'e>(&mut self, name: &Ident, prop: &dyn Fn(&str) -> Option<&'e Expr>, span: Span) -> Result<HudPart, Error> {
        let Some(text) = prop("label") else {
            return Err(self.error(span, &format!("`ui.text {}` needs `label = \"…\"`", name.text)));
        };
        let (prep, text, live) = self.ui_text(text)?;
        let (x, y) = self.ui_position(prop("position"), 200, 20)?;
        let color = match prop("color") {
            Some(e) => self.ui_color(e)?,
            None => 0xFFFFFF,
        };
        let overlay = Arg::Global(format!("$uw_hud_{}_text", name.text));
        let mut load = prep;
        load.push(op("create_text_overlay", vec![overlay.clone(), text]));
        load.push(op("overlay_set_color", vec![overlay.clone(), Arg::Int(color)]));
        if let Some(alpha) = prop("alpha") {
            let alpha = self.ui_alpha(alpha)?;
            load.push(op("overlay_set_alpha", vec![overlay.clone(), Arg::Int(alpha)]));
        }
        place(&mut load, &overlay, "overlay_set_position", Arg::Int(x), Arg::Int(y));
        if let Some(size) = prop("font_size") {
            let Expr::Percent { value, .. } = size else {
                return Err(self.error(size.span(), "`font_size` is a percent of the normal size (`80%`)"));
            };
            place(&mut load, &overlay, "overlay_set_size", Arg::Int(value * 10), Arg::Int(value * 10));
        }
        self.begin_block();
        let result = self
            .visibility(prop("visible"), std::slice::from_ref(&overlay))
            .and_then(|()| self.live_text(&overlay, live.as_deref(), span));
        let run = self.end_block();
        result?;
        Ok(HudPart { name: name.text.clone(), load, run })
    }

    /// `visible = expr`: every frame, each overlay of the part is shown (1) or hidden (0).
    fn visibility(&mut self, visible: Option<&Expr>, overlays: &[Arg]) -> Result<(), Error> {
        let Some(visible) = visible else { return Ok(()) };
        let flag = self.cond_value(visible)?;
        for overlay in overlays {
            self.emit("overlay_set_display", vec![overlay.clone(), flag.clone()], visible.span())?;
        }
        Ok(())
    }

    /// A position: an anchor word (`bottom_left`, `top_right`, `center`, …) or `(x, y)` in screen units.
    fn ui_position(&self, e: Option<&Expr>, w: i64, h: i64) -> Result<(i64, i64), Error> {
        let Some(e) = e else { return Ok((MARGIN, MARGIN)) };
        if let Expr::Path(path) = e {
            if path.parts.len() == 1 {
                let left = MARGIN;
                let right = SCREEN_W - MARGIN - w;
                let middle_x = (SCREEN_W - w) / 2;
                let bottom = MARGIN;
                let top = SCREEN_H - MARGIN - h;
                let middle_y = (SCREEN_H - h) / 2;
                let at = match path.parts[0].text.as_str() {
                    "bottom_left" => (left, bottom),
                    "bottom_right" => (right, bottom),
                    "bottom_center" => (middle_x, bottom),
                    "top_left" => (left, top),
                    "top_right" => (right, top),
                    "top_center" => (middle_x, top),
                    "center_left" => (left, middle_y),
                    "center_right" => (right, middle_y),
                    "center" => (middle_x, middle_y),
                    _ => (i64::MIN, 0),
                };
                if at.0 != i64::MIN {
                    return Ok(at);
                }
            }
        }
        if let Expr::Tuple { .. } = e {
            return self.ui_pair(e, "position");
        }
        Err(self.error(
            e.span(),
            "`position` is an anchor (bottom_left, bottom_center, bottom_right, center_left, center, center_right, \
             top_left, top_center, top_right) or `(x, y)` in screen units (x 0..1000, y 0..750)",
        ))
    }

    /// `(a, b)` of plain numbers (constants allowed), as whole screen units.
    fn ui_pair(&self, e: &Expr, what: &str) -> Result<(i64, i64), Error> {
        if let Expr::Tuple { items, .. } = e {
            if items.len() == 2 {
                let a = self.const_value(&items[0])?;
                let b = self.const_value(&items[1])?;
                return Ok((a / 1000, b / 1000));
            }
        }
        Err(self.error(e.span(), &format!("`{what}` is a pair of numbers: `(220, 14)`")))
    }

    fn ui_color(&self, e: &Expr) -> Result<i64, Error> {
        match self.resolve_const(e) {
            Expr::Color { rgb, .. } => Ok(i64::from(rgb)),
            _ => Err(self.error(e.span(), "a color is written `#3FA34D`")),
        }
    }

    /// `alpha = 80%` → 0..255.
    fn ui_alpha(&self, e: &Expr) -> Result<i64, Error> {
        match self.resolve_const(e) {
            Expr::Percent { value, .. } if (0..=100).contains(&value) => Ok(value * 255 / 100),
            _ => Err(self.error(e.span(), "`alpha` is a percent from 0% to 100%")),
        }
    }

    /// A label: its first text for the HUD's opening (with the operations that fill its values) and, when
    /// it shows values (`"Stamina: {stamina}"`), its parts, so every frame can show it again.
    fn ui_text(&mut self, e: &Expr) -> Result<(Vec<Op>, Arg, Option<Vec<TextPart>>), Error> {
        match self.resolve_const(e) {
            Expr::Text { parts, span } => {
                self.begin_block();
                let text = self.text_value(&parts, span);
                let prep = self.end_block();
                let live = parts.iter().any(|part| matches!(part, TextPart::Expr(_))).then_some(parts);
                Ok((prep, text?, live))
            }
            _ => Err(self.error(e.span(), "a label is a text: `\"Stamina\"`")),
        }
    }

    /// Every frame: a label with values is written again, so it follows them.
    fn live_text(&mut self, overlay: &Arg, parts: Option<&[TextPart]>, span: Span) -> Result<(), Error> {
        let Some(parts) = parts else { return Ok(()) };
        let text = self.text_value(parts, span)?;
        self.emit("overlay_set_text", vec![overlay.clone(), text], span)
    }

    /// The HUD presentation and the ops that keep it up, or `None` when no feature declares a part.
    pub(crate) fn hud_presentation(&self) -> Option<(crate::Presentation, Vec<Op>)> {
        if self.hud.is_empty() {
            return None;
        }
        let mut on_load = vec![op("set_fixed_point_multiplier", vec![Arg::Int(1000)])];
        let mut on_run = vec![op("set_fixed_point_multiplier", vec![Arg::Int(1000)])];
        for part in &self.hud {
            on_load.extend(part.load.iter().cloned());
            on_run.extend(part.run.iter().cloned());
        }
        on_load.push(op("presentation_set_duration", vec![Arg::Int(999_999)]));
        let presentation = crate::Presentation {
            name: HUD.to_owned(),
            flags: vec!["prsntf_read_only".to_owned(), "prsntf_manual_end_only".to_owned()],
            mesh: None,
            on_load,
            on_run,
        };
        let prsnt = Arg::Name(format!("prsnt_{HUD}"));
        let keep_up = vec![
            op("try_begin", vec![]),
            Op { name: "is_presentation_active".to_owned(), args: vec![prsnt.clone()], neg: true, this_or_next: false },
            op("start_presentation", vec![prsnt]),
            op("try_end", vec![]),
        ];
        Some((presentation, keep_up))
    }
}
