use eframe::egui;

/// Which panel is currently open in the bottom panel area.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ActivePanel {
    None,
    Preamp,
    Cabinet,
}

pub fn draw_signal_chain(ui: &mut egui::Ui, active_panel: &mut ActivePanel) {
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 60.0),
        egui::Sense::hover(),
    );

    let painter = ui.painter_at(rect);

    // Background
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(25, 25, 30));

    let line_y = rect.center().y;
    let start_x = rect.left() + 20.0;
    let end_x = rect.right() - 20.0;

    // Signal chain base-line
    painter.hline(
        start_x..=end_x,
        line_y,
        egui::Stroke::new(3.0, egui::Color32::from_rgb(80, 80, 90)),
    );

    let node_height = 40.0;

    // --- Helper closure: draw a single node ---
    let draw_node = |painter: &egui::Painter,
                     ui_ref: &mut egui::Ui,
                     x_center: f32,
                     label: &str,
                     id_str: &str,
                     is_active: bool| -> bool {
        let node_width = label.len() as f32 * 8.5 + 18.0; // auto-width
        let node_rect = egui::Rect::from_center_size(
            egui::pos2(x_center, line_y),
            egui::vec2(node_width, node_height),
        );

        let response = ui_ref.interact(node_rect, egui::Id::new(id_str), egui::Sense::click());

        let fill = if response.hovered() {
            egui::Color32::from_rgb(70, 60, 80)
        } else {
            egui::Color32::from_rgb(45, 45, 55)
        };

        let stroke_color = if is_active {
            egui::Color32::from_rgb(255, 160, 60) // orange accent = active
        } else {
            egui::Color32::from_rgb(120, 120, 130)
        };

        painter.rect_filled(node_rect, 4.0, fill);
        painter.rect_stroke(
            node_rect,
            4.0,
            egui::Stroke::new(2.0, stroke_color),
            egui::StrokeKind::Inside,
        );
        painter.text(
            node_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(11.0),
            egui::Color32::from_rgb(210, 210, 210),
        );

        response.clicked()
    };

    // --- Node positions ---
    let total_span = end_x - start_x;
    let preamp_x = start_x + total_span * 0.55;
    let cabinet_x = start_x + total_span * 0.80;

    // Draw arrow connector between preamp and cabinet
    let arrow_y = line_y;
    let arrow_mid = (preamp_x + cabinet_x) * 0.5;
    painter.arrow(
        egui::pos2(arrow_mid - 8.0, arrow_y),
        egui::vec2(16.0, 0.0),
        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 110)),
    );

    // Draw nodes using a mutable Ui scope so interactions register
    let preamp_clicked = draw_node(
        &painter,
        ui,
        preamp_x,
        "PREAMP",
        "preamp_node",
        *active_panel == ActivePanel::Preamp,
    );
    let cabinet_clicked = draw_node(
        &painter,
        ui,
        cabinet_x,
        "CABINET",
        "cabinet_node",
        *active_panel == ActivePanel::Cabinet,
    );

    if preamp_clicked {
        *active_panel = if *active_panel == ActivePanel::Preamp {
            ActivePanel::None
        } else {
            ActivePanel::Preamp
        };
    }
    if cabinet_clicked {
        *active_panel = if *active_panel == ActivePanel::Cabinet {
            ActivePanel::None
        } else {
            ActivePanel::Cabinet
        };
    }
}
