//! Image-backed graph example for egui_xyflow.
//!
//! Every node displays the supplied Quiz Pool screenshot as a portrait card.
//! The image is decoded once, uploaded once as an egui texture, and reused by
//! all nodes in the graph.

use eframe::egui;
use egui_xyflow::prelude::*;

struct ImageNodeWidget {
    texture: egui::TextureHandle,
    image_size: egui::Vec2,
}

fn node_header_color(label: &str) -> egui::Color32 {
    match label {
        "Redeem" => egui::Color32::from_rgb(168, 85, 247),
        "Maths" => egui::Color32::from_rgb(59, 130, 246),
        "G K" => egui::Color32::from_rgb(14, 165, 233),
        "Science" => egui::Color32::from_rgb(239, 68, 68),
        _ => egui::Color32::from_rgb(0, 172, 193),
    }
}

impl NodeWidget<String> for ImageNodeWidget {
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
        _hovered: bool,
        transform: &Transform,
    ) {
        let scale = transform.scale;
        let rounding = config.node_corner_radius;
        let border = if node.selected {
            config.node_selected_border_color
        } else {
            config.node_border_color
        };

        if node.selected {
            painter.rect_filled(
                screen_rect.expand(5.0 * scale),
                rounding + 2.0,
                egui::Color32::from_rgba_unmultiplied(59, 130, 246, 65),
            );
        }
        painter.rect_filled(screen_rect, rounding, egui::Color32::WHITE);

        let inner = screen_rect.shrink(6.0 * scale);
        let header_height = (27.0 * scale).min(inner.height() * 0.25);
        let header = egui::Rect::from_min_max(
            inner.min,
            egui::pos2(inner.max.x, inner.min.y + header_height),
        );
        let image_area = egui::Rect::from_min_max(
            egui::pos2(inner.min.x, header.max.y + 4.0 * scale),
            inner.max,
        );

        painter.rect_filled(
            header,
            (rounding * scale).min(header_height * 0.5),
            node_header_color(&node.data),
        );

        // Fit the phone screenshot without distorting its 1200×1600 aspect ratio.
        let image_aspect = self.image_size.x / self.image_size.y;
        let area_aspect = image_area.width() / image_area.height();
        let fitted_size = if area_aspect > image_aspect {
            egui::vec2(image_area.height() * image_aspect, image_area.height())
        } else {
            egui::vec2(image_area.width(), image_area.width() / image_aspect)
        };
        let image_rect = egui::Rect::from_center_size(image_area.center(), fitted_size);

        painter.image(
            self.texture.id(),
            image_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        painter.text(
            header.left_center() + egui::vec2(8.0 * scale, 0.0),
            egui::Align2::LEFT_CENTER,
            &node.data,
            egui::FontId::proportional(13.0 * scale),
            egui::Color32::WHITE,
        );
        painter.rect_stroke(
            screen_rect,
            rounding,
            egui::Stroke::new(
                config.node_border_width * if node.selected { 2.0 } else { 1.0 },
                border,
            ),
            egui::StrokeKind::Middle,
        );
    }
}

struct ImageGraphApp {
    state: FlowState<String, ()>,
    node_widget: ImageNodeWidget,
    initial_fit: bool,
}

impl ImageGraphApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let image = image::ImageReader::open("images/logoipsum.png")
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to decode image");
        let size = (image.width(), image.height());
        let image_size = egui::vec2(size.0 as f32, size.1 as f32);
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [size.0 as usize, size.1 as usize],
            image.to_rgba8().as_flat_samples().as_slice(),
        );

        let texture = cc.egui_ctx.load_texture(
            "quiz_pool_app_screenshot",
            color_image,
            egui::TextureOptions::LINEAR,
        );

        let config = FlowConfig {
            show_background: true,
            background_variant: BackgroundVariant::Dots,
            show_minimap: true,
            track_edge_hover: false,
            ..FlowConfig::default()
        };
        let mut state = FlowState::new(config);

        state.add_nodes([
            Node::builder("home")
                .position(egui::pos2(40.0, 240.0))
                .data("Quiz Pool".to_owned())
                .handle(NodeHandle::source(Position::Right).with_id("redeem"))
                .handle(NodeHandle::source(Position::Bottom).with_id("maths"))
                .size(190.0, 280.0)
                .build(),
            Node::builder("redeem")
                .position(egui::pos2(320.0, 70.0))
                .data("Redeem".to_owned())
                .handle(NodeHandle::target(Position::Left).with_id("from_home"))
                .size(190.0, 280.0)
                .build(),
            Node::builder("maths")
                .position(egui::pos2(600.0, 240.0))
                .data("Maths".to_owned())
                .handle(NodeHandle::target(Position::Left).with_id("from_home"))
                .handle(NodeHandle::source(Position::Bottom).with_id("gk"))
                .handle(NodeHandle::source(Position::Right).with_id("science"))
                .size(190.0, 280.0)
                .build(),
            Node::builder("gk")
                .position(egui::pos2(320.0, 520.0))
                .data("G K".to_owned())
                .handle(NodeHandle::target(Position::Top).with_id("from_maths"))
                .size(190.0, 280.0)
                .build(),
            Node::builder("science")
                .position(egui::pos2(880.0, 520.0))
                .data("Science".to_owned())
                .handle(NodeHandle::target(Position::Top).with_id("from_maths"))
                .size(190.0, 280.0)
                .build(),
            Node::builder("computer")
                .position(egui::pos2(920.0, 80.0))
                .data("Computer".to_owned())
                .handle(NodeHandle::target(Position::Left).with_id("from_computer"))
                .handle(NodeHandle::target(Position::Top).with_id("top_computer"))
                .size(190.0, 280.0)
                .build(),
        ]);

        state.add_edges([
            connect("home-redeem", "home", "redeem", "redeem", "from_home")
                .edge_type(EdgeType::Bezier)
                .color(egui::Color32::from_rgb(168, 85, 247))
                .label("to Redeem")
                .marker_end_arrow(),
            connect("home-maths", "home", "maths", "maths", "from_home")
                .edge_type(EdgeType::Bezier)
                .animated(true)
                .color(egui::Color32::from_rgb(59, 130, 246))
                .label("to Maths")
                .marker_end_arrow(),
            connect("maths-gk", "maths", "gk", "gk", "from_maths")
                .edge_type(EdgeType::Step)
                .color(egui::Color32::from_rgb(14, 165, 233))
                .label("to G K")
                .marker_end_arrow(),
            connect("maths-science", "maths", "science", "science", "from_maths")
                .edge_type(EdgeType::SmoothStep)
                .color(egui::Color32::from_rgb(239, 68, 68))
                .label("to Science")
                .marker_end_arrow(),
            connect(
                "maths-computer",
                "maths",
                "computer",
                "from_maths",
                "from_computer",
            )
            .edge_type(EdgeType::Elbow)
            .color(egui::Color32::from_rgb(239, 68, 68))
            .label("to Computer")
            .marker_end_arrow(),
        ]);

        Self {
            state,
            node_widget: ImageNodeWidget {
                texture,
                image_size,
            },
            initial_fit: false,
        }
    }
}

/// Create an edge with explicit handle IDs so the wiring is easy to follow.
fn connect(
    id: &str,
    source: &str,
    target: &str,
    source_handle: &str,
    target_handle: &str,
) -> Edge<()> {
    let mut edge = Edge::new(id, source, target);
    edge.source_handle = Some(source_handle.to_owned());
    edge.target_handle = Some(target_handle.to_owned());
    edge
}

impl eframe::App for ImageGraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initial_fit {
            self.state
                .fit_view(ctx.screen_rect(), 60.0, ctx.input(|i| i.time));
            self.initial_fit = true;
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Quiz Pool Image Graph");
                if ui.button("Fit All").clicked() {
                    self.state
                        .fit_view(ctx.screen_rect(), 40.0, ctx.input(|i| i.time));
                }
                ui.separator();
                ui.label(format!(
                    "{} image nodes · {} edges",
                    self.state.nodes.len(),
                    self.state.edges.len()
                ));
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::from_rgb(22, 30, 48)))
            .show(ctx, |ui| {
                let _events = FlowCanvas::new(&mut self.state, &self.node_widget).show(ui);
            });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("egui_xyflow — image graph")
            .with_inner_size([1400.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui_xyflow Image Graph",
        options,
        Box::new(|cc| Ok(Box::new(ImageGraphApp::new(cc)))),
    )
}
