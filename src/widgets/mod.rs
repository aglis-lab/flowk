use crate::{FlowConfig, Node, NodeWidget, Transform};

pub struct ImageWidget {
    texture: egui::TextureHandle,
    size: egui::Vec2,
}

impl ImageWidget {
    pub fn new(texture: egui::TextureHandle, size: egui::Vec2) -> Self {
        Self { texture, size }
    }
}

impl NodeWidget<String> for ImageWidget {
    fn size(&self, node: &Node<String>, config: &FlowConfig) -> egui::Vec2 {
        egui::vec2(
            node.width.unwrap_or(config.default_node_width),
            node.height.unwrap_or(config.default_node_height),
        )
    }

    fn show(
        &self,
        painter: &egui::Painter,
        node: &Node<String>,
        screen_rect: egui::Rect,
        config: &FlowConfig,
        is_hover: bool,
        transform: &Transform,
    ) {
        // Border and Rounded Corners for the Node
        let corner_radius = config.node_corner_radius;
        let border_color = if node.selected {
            config.node_selected_border_color
        } else if is_hover {
            config.node_hover_border_color
        } else {
            config.node_border_color
        };

        if node.selected {
            painter.rect_stroke(
                screen_rect,
                corner_radius,
                egui::Stroke::new(config.node_border_width * 2.0, border_color),
                egui::StrokeKind::Middle,
            );
        } else if is_hover {
            painter.rect_stroke(
                screen_rect,
                config.node_corner_radius,
                egui::Stroke::new(config.node_border_width, border_color),
                egui::StrokeKind::Middle,
            );
        } else {
            painter.rect_stroke(
                screen_rect,
                corner_radius,
                egui::Stroke::new(config.node_border_width, border_color),
                egui::StrokeKind::Middle,
            );
        }

        // Selected Node
        let scale = transform.scale;
        if node.selected {
            painter.rect_filled(
                screen_rect.expand(5.0 * scale),
                corner_radius + 2.0,
                egui::Color32::from_rgba_unmultiplied(59, 130, 246, 65),
            );
        }

        // TODO: Draw a shadow when the node not being selected

        // Background Color
        painter.rect_filled(screen_rect, corner_radius, egui::Color32::WHITE);

        // Calculate Header and Image Size
        let inner = screen_rect.clone();
        let header_height = (27.0 * scale).min(inner.height() * 0.25);
        let header = egui::Rect::from_min_max(
            inner.min,
            egui::pos2(inner.max.x, inner.min.y + header_height),
        );
        let image_area = egui::Rect::from_min_max(
            egui::pos2(inner.min.x, header.max.y + 4.0 * scale),
            inner.max,
        );

        // Fit the phone screenshot without distorting its 1200×1600 aspect ratio.
        let image_aspect = self.size.x / self.size.y;
        let area_aspect = image_area.width() / image_area.height();
        let fitted_width = if area_aspect > image_aspect {
            image_area.height() * image_aspect
        } else {
            image_area.width()
        };

        // Center horizontally, but stretch vertically (no top/bottom padding)
        let margin = 4.0 * scale;
        let image_rect = egui::Rect::from_min_max(
            egui::pos2(
                image_area.center().x - fitted_width / 2.0 + margin,
                image_area.min.y + margin,
            ),
            egui::pos2(
                image_area.center().x + fitted_width / 2.0 - margin,
                image_area.max.y - margin,
            ),
        );

        painter.image(
            self.texture.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );

        // Draw the node header background
        let header_radius = epaint::CornerRadius {
            nw: corner_radius.round() as u8,
            ne: corner_radius.round() as u8,
            sw: 0,
            se: 0,
        };
        painter.rect_filled(header, header_radius, egui::Color32::from_rgb(0, 172, 193));

        // Draw the node header text
        painter.text(
            header.center(),
            egui::Align2::CENTER_CENTER,
            &node.data,
            egui::FontId::proportional(13.0 * scale),
            egui::Color32::WHITE,
        );
        // painter.rect_stroke(
        //     screen_rect,
        //     corner_radius,
        //     egui::Stroke::new(
        //         config.node_border_width * if node.selected { 2.0 } else { 1.0 },
        //         border_color,
        //     ),
        //     egui::StrokeKind::Middle,
        // );
    }
}
