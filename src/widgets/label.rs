use crate::style::Style;
use bevy_egui::egui::*;

pub struct BorderedLabel {
    text: WidgetText,
    min_size: Vec2,
    style: Option<Style>,
}

impl BorderedLabel {
    pub fn new(text: WidgetText, style: Option<Style>) -> Self {
        Self {
            text,
            min_size: Vec2::ZERO,
            style,
        }
    }
}

impl Widget for BorderedLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let BorderedLabel {
            text,
            min_size,
            style,
        }: BorderedLabel = self;

        let style = style.unwrap_or_default();

        let total_extra = style.padding.sum() + style.margin.sum();

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = text.size() + total_extra;
        desired_size = desired_size.at_least(min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, Sense::hover());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, text.text()));

        // Focus the button automatically when it is hovered and the mouse is moving
        if response.hovered() && ui.ctx().input().pointer.velocity().length_sq() > 0.0 {
            response.request_focus();
        }

        if ui.is_rect_visible(rect) {
            let mut text_rect = rect;
            text_rect.min += style.padding.left_top() + style.margin.left_top();
            text_rect.max -= style.padding.right_bottom() + style.margin.right_bottom();
            text_rect.max.x = text_rect.max.x.max(text_rect.min.x);
            text_rect.max.y = text_rect.max.y.max(text_rect.min.y);

            let label_pos = ui
                .layout()
                .align_size_within_rect(text.size(), text_rect)
                .min;

            let controlstate = style.normal;

            let mut border_rect = rect;
            border_rect.min += style.margin.left_top();
            border_rect.max -= style.margin.right_bottom();
            border_rect.max.x = border_rect.max.x.max(border_rect.min.x);
            border_rect.max.y = border_rect.max.y.max(border_rect.min.y);

            ui.painter().rect(
                border_rect,
                0.0,
                controlstate.bg,
                Stroke::new(controlstate.stroke_width, controlstate.stroke),
            );

            text.paint_with_fallback_color(ui.painter(), label_pos, controlstate.fg);
        }

        response
    }
}