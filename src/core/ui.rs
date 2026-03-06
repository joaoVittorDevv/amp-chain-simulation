use eframe::egui;

pub fn draw_spectrum(ui: &mut egui::Ui, spectrum: &[f32], fft_size: usize) {

    let width = ui.available_width().max(10.0);
    let height = (ui.available_height() - 20.0).max(10.0);
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(width, height),
        egui::Sense::hover(),
    );
    
    let painter = ui.painter_at(rect);
    
    painter.rect_filled(rect, 5.0, egui::Color32::from_rgb(15, 15, 20));
    
    let min_freq = 20.0f32;
    let max_freq = 20000.0f32;
    let log_min = min_freq.log2();
    let log_max = max_freq.log2();
    let log_range = log_max - log_min;
    
    let db_min = -100.0f32;
    let db_max = 20.0f32;
    let db_range = db_max - db_min;

    for db in [0.0, -12.0, -24.0, -48.0, -72.0] {
        let fy = (db - db_min) / db_range;
        let fy = fy.clamp(0.0, 1.0);
        let y = rect.bottom() - fy * rect.height();
        painter.hline(
            rect.left()..=rect.right(),
            y,
            egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 40))
        );
        painter.text(
            egui::pos2(rect.left() + 10.0, y - 5.0),
            egui::Align2::LEFT_BOTTOM,
            format!("{} dB", db),
            egui::FontId::proportional(10.0),
            egui::Color32::from_gray(120),
        );
    }
    
    for &freq in &[50.0f32, 100.0, 500.0, 1000.0, 5000.0, 10000.0] {
        let fraction_x = (freq.log2() - log_min) / log_range;
        let x = rect.left() + fraction_x * rect.width();
        painter.vline(
            x,
            rect.top()..=rect.bottom(),
            egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 30, 35))
        );
        painter.text(
            egui::pos2(x + 5.0, rect.bottom() - 15.0),
            egui::Align2::LEFT_BOTTOM,
            if freq >= 1000.0 { format!("{}k", freq / 1000.0) } else { format!("{}", freq) },
            egui::FontId::proportional(10.0),
            egui::Color32::from_gray(100),
        );
    }
    
    let mut points = Vec::with_capacity(fft_size / 2);
    let width = rect.width();
    let height = rect.height();
    let sample_rate = 48000.0;
    
    for (i, &db) in spectrum.iter().enumerate() {
        if i == 0 { continue; }
        
        let freq = (i as f32 * sample_rate) / fft_size as f32;
        if freq < min_freq || freq > max_freq { continue; }
        
        let fraction_x = (freq.log2() - log_min) / log_range;
        let x = rect.left() + fraction_x * width;
        
        let fraction_y = (db - db_min) / db_range;
        let fraction_y = fraction_y.clamp(0.0, 1.0);
        let y = rect.bottom() - fraction_y * height;
        
        points.push(egui::pos2(x, y));
    }
    
    if points.len() > 1 {
        let stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 210, 255));
        let shape = egui::Shape::line(points, stroke);
        painter.add(shape);
    }
}

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
    let node_width = 40.0;
    let node_height = 40.0;
    let node_x_center = end_x - 30.0; // Place it near the end
    let node_rect = egui::Rect::from_center_size(
        egui::pos2(node_x_center, line_y),
        egui::vec2(node_width, node_height)
    );
    
    // Add interaction to it
    let node_response = ui.interact(node_rect, egui::Id::new("distortion_node"), egui::Sense::click_and_drag());
    
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
        "DIST",
        egui::FontId::proportional(12.0),
        egui::Color32::from_rgb(200, 200, 200),
    );
}
