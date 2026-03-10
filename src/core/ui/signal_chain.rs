use nih_plug_egui::egui;

/// Which panel is currently open in the bottom panel area.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ActivePanel {
    None,
    NeuralAmp,
    Preamp,
    Cabinet,
}

pub fn draw_signal_chain(
    ui: &mut egui::Ui,
    active_panel: &mut ActivePanel,
    neural_active: bool,
    preamp_active: bool,
    cabinet_active: bool,
    global_bypass: bool,
    mut on_neural_toggle: impl FnMut(),
    mut on_preamp_toggle: impl FnMut(),
    mut on_cabinet_toggle: impl FnMut(),
) {
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
                      is_panel_active: bool,
                      is_module_active: bool| -> (bool, bool) {
        let node_width = label.len() as f32 * 8.5 + 40.0; // wider for power button
        let node_rect = egui::Rect::from_center_size(
            egui::pos2(x_center, line_y),
            egui::vec2(node_width, node_height),
        );

        let response = ui_ref.interact(node_rect, egui::Id::new(id_str), egui::Sense::click());
        
        // Power button rect (top right corner)
        let power_size = 14.0;
        let power_rect = egui::Rect::from_center_size(
            node_rect.right_top() + egui::vec2(-power_size * 0.6, power_size * 0.6),
            egui::vec2(power_size, power_size)
        );
        let power_resp = ui_ref.interact(power_rect, egui::Id::new(format!("{}_power", id_str)), egui::Sense::click());

        let visually_active = is_module_active && !global_bypass;

        let fill = if !visually_active {
            egui::Color32::from_rgb(30, 30, 35) // Dark "OFF" state
        } else if response.hovered() {
            egui::Color32::from_rgb(70, 60, 80)
        } else {
            egui::Color32::from_rgb(45, 45, 55)
        };

        let stroke_color = if is_panel_active {
            egui::Color32::from_rgb(255, 160, 60) // orange accent = focused
        } else if visually_active {
            egui::Color32::from_rgb(120, 120, 130)
        } else {
            egui::Color32::from_rgb(60, 60, 70) // dimmed stroke
        };

        painter.rect_filled(node_rect, 4.0, fill);
        painter.rect_stroke(
            node_rect,
            4.0,
            egui::Stroke::new(2.0, stroke_color),
            egui::StrokeKind::Inside,
        );

        // Power Icon (Circle and vertical line)
        let power_icon_color = if visually_active {
            egui::Color32::from_rgb(0, 255, 120) // Vibrant green "ON"
        } else if is_module_active && global_bypass {
            egui::Color32::from_rgb(0, 100, 50) // Dimmed green when global bypass is ON
        } else {
            egui::Color32::from_rgb(255, 50, 50)  // Red "OFF"
        };
        
        let icon_center = power_rect.center();
        let icon_radius = power_size * 0.35;
        
        painter.circle_stroke(icon_center, icon_radius, egui::Stroke::new(1.5, power_icon_color));
        painter.line_segment(
            [icon_center - egui::vec2(0.0, icon_radius * 0.4), icon_center - egui::vec2(0.0, icon_radius * 1.3)],
            egui::Stroke::new(1.5, power_icon_color),
        );

        painter.text(
            node_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(11.0),
            if visually_active { egui::Color32::from_rgb(210, 210, 210) } else { egui::Color32::from_rgb(100, 100, 110) },
        );

        (response.clicked(), power_resp.clicked())
    };

    // --- Node positions ---
    let total_span = end_x - start_x;
    let neural_x = start_x + total_span * 0.30;
    let preamp_x = start_x + total_span * 0.55;
    let cabinet_x = start_x + total_span * 0.80;

    // Draw arrow connectors
    let arrow_y = line_y;
    
    // Neural -> Preamp
    let arrow_mid1 = (neural_x + preamp_x) * 0.5;
    painter.arrow(
        egui::pos2(arrow_mid1 - 8.0, arrow_y),
        egui::vec2(16.0, 0.0),
        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 110)),
    );

    // Preamp -> Cabinet
    let arrow_mid2 = (preamp_x + cabinet_x) * 0.5;
    painter.arrow(
        egui::pos2(arrow_mid2 - 8.0, arrow_y),
        egui::vec2(16.0, 0.0),
        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 110)),
    );

    // Draw nodes
    let (neural_clicked, neural_power_clicked) = draw_node(
        &painter,
        ui,
        neural_x,
        "NEURAL AMP",
        "neural_node",
        *active_panel == ActivePanel::NeuralAmp,
        neural_active,
    );
    let (preamp_clicked, preamp_power_clicked) = draw_node(
        &painter,
        ui,
        preamp_x,
        "PREAMP",
        "preamp_node",
        *active_panel == ActivePanel::Preamp,
        preamp_active,
    );
    
    let (cabinet_clicked, cabinet_power_clicked) = draw_node(
        &painter,
        ui,
        cabinet_x,
        "CABINET",
        "cabinet_node",
        *active_panel == ActivePanel::Cabinet,
        cabinet_active,
    );

    if neural_power_clicked {
        on_neural_toggle();
    } else if neural_clicked {
        *active_panel = if *active_panel == ActivePanel::NeuralAmp {
            ActivePanel::None
        } else {
            ActivePanel::NeuralAmp
        };
    }

    if preamp_power_clicked {
        on_preamp_toggle();
    } else if preamp_clicked {
        *active_panel = if *active_panel == ActivePanel::Preamp {
            ActivePanel::None
        } else {
            ActivePanel::Preamp
        };
    }
    
    if cabinet_power_clicked {
        on_cabinet_toggle();
    } else if cabinet_clicked {
        *active_panel = if *active_panel == ActivePanel::Cabinet {
            ActivePanel::None
        } else {
            ActivePanel::Cabinet
        };
    }
}

