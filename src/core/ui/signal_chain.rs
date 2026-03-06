use eframe::egui;

pub fn draw_signal_chain(ui: &mut egui::Ui, panel_open: &mut bool) {
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 60.0),
        egui::Sense::hover(),
    );
    
    let painter = ui.painter_at(rect);
    
    // Background to clearly see the signal chain section
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(25, 25, 30));
    
    let line_y = rect.center().y;
    let start_x = rect.left() + 20.0;
    let end_x = rect.right() - 20.0;
    
    // Base line (signal chain path)
    painter.hline(
        start_x..=end_x,
        line_y,
        egui::Stroke::new(3.0, egui::Color32::from_rgb(80, 80, 90)),
    );
    
    // Node/Box
    let node_width = 60.0;
    let node_height = 40.0;
    let node_x_center = end_x - 30.0; // Place it near the end
    let node_rect = egui::Rect::from_center_size(
        egui::pos2(node_x_center, line_y),
        egui::vec2(node_width, node_height)
    );
    
    // Add interaction to it
    let node_response = ui.interact(node_rect, egui::Id::new("cabinet_node"), egui::Sense::click_and_drag());
    
    if node_response.clicked() {
        *panel_open = !*panel_open;
    }
    
    // Render Node
    let fill_color = if node_response.hovered() {
        egui::Color32::from_rgb(60, 60, 70)
    } else {
        egui::Color32::from_rgb(45, 45, 55)
    };
    
    let stroke_color = if *panel_open {
        egui::Color32::from_rgb(0, 210, 255)
    } else {
        egui::Color32::from_rgb(120, 120, 130)
    };
    
    painter.rect_filled(node_rect, 4.0, fill_color);
    painter.rect_stroke(node_rect, 4.0, egui::Stroke::new(2.0, stroke_color), egui::StrokeKind::Inside);
    
    // Add text or icon inside the node
    painter.text(
        node_rect.center(),
        egui::Align2::CENTER_CENTER,
        "CABINET",
        egui::FontId::proportional(12.0),
        egui::Color32::from_rgb(200, 200, 200),
    );
}
