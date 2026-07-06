use crate::core::state::plugin_params::{BaseIOParams, MlcBright, MlcFeedback, MlcGatePos};
use nih_plug::prelude::{BoolParam, EnumParam, FloatParam, ParamSetter};
use nih_plug_egui::egui;
use std::sync::Arc;

fn param_knob(ui: &mut egui::Ui, setter: &ParamSetter, label: &str, param: &FloatParam) {
    use egui_knob::{Knob, KnobStyle};

    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(label)
                .small()
                .color(egui::Color32::GRAY),
        );
        let mut value = param.value();
        let (min, max) = match param.range() {
            nih_plug::prelude::FloatRange::Linear { min, max } => (min, max),
            nih_plug::prelude::FloatRange::Skewed { min, max, .. } => (min, max),
            _ => (0.0, 1.0),
        };
        let response = ui.add(Knob::new(&mut value, min, max, KnobStyle::Wiper).with_size(45.0));
        if response.drag_started() {
            setter.begin_set_parameter(param);
        }
        if response.changed() {
            setter.set_parameter(param, value);
        }
        if response.drag_stopped() {
            setter.end_set_parameter(param);
        }
    });
}

fn bool_switch(ui: &mut egui::Ui, setter: &ParamSetter, label: &str, param: &BoolParam) {
    let mut value = param.value();
    if ui.checkbox(&mut value, label).changed() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, value);
        setter.end_set_parameter(param);
    }
}

fn bright_switch(ui: &mut egui::Ui, setter: &ParamSetter, param: &EnumParam<MlcBright>) {
    let mut value = param.value();
    egui::ComboBox::from_id_salt("mlc_bright_combo")
        .selected_text(match value {
            MlcBright::I => "I",
            MlcBright::Ii => "II",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut value, MlcBright::I, "I");
            ui.selectable_value(&mut value, MlcBright::Ii, "II");
        });
    if value != param.value() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, value);
        setter.end_set_parameter(param);
    }
}

fn feedback_switch(ui: &mut egui::Ui, setter: &ParamSetter, param: &EnumParam<MlcFeedback>) {
    let mut value = param.value();
    egui::ComboBox::from_id_salt("mlc_feedback_combo")
        .selected_text(match value {
            MlcFeedback::Lo => "Lo",
            MlcFeedback::Hi => "Hi",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut value, MlcFeedback::Lo, "Lo");
            ui.selectable_value(&mut value, MlcFeedback::Hi, "Hi");
        });
    if value != param.value() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, value);
        setter.end_set_parameter(param);
    }
}

fn gate_pos_switch(ui: &mut egui::Ui, setter: &ParamSetter, param: &EnumParam<MlcGatePos>) {
    let mut value = param.value();
    egui::ComboBox::from_id_salt("mlc_gate_pos_combo")
        .selected_text(match value {
            MlcGatePos::Pre => "Pre",
            MlcGatePos::Post => "Post",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut value, MlcGatePos::Pre, "Pre");
            ui.selectable_value(&mut value, MlcGatePos::Post, "Post");
        });
    if value != param.value() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, value);
        setter.end_set_parameter(param);
    }
}

pub fn draw_mlc_zero_v_panel(ui: &mut egui::Ui, setter: &ParamSetter, params: &Arc<BaseIOParams>) {
    ui.horizontal_wrapped(|ui| {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Gain").strong());
            ui.horizontal(|ui| {
                param_knob(ui, setter, "Gain", &params.mlc_gain);
                param_knob(ui, setter, "Master", &params.mlc_master);
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("EQ").strong());
            ui.horizontal(|ui| {
                param_knob(ui, setter, "Bass", &params.mlc_bass);
                param_knob(ui, setter, "Middle", &params.mlc_middle);
                param_knob(ui, setter, "Treble", &params.mlc_treble);
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Power Amp").strong());
            ui.horizontal(|ui| {
                param_knob(ui, setter, "Presence", &params.mlc_presence);
                param_knob(ui, setter, "Depth", &params.mlc_depth);
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Switches").strong());
            ui.horizontal(|ui| {
                ui.label("Bright");
                bright_switch(ui, setter, &params.mlc_bright);
                bool_switch(ui, setter, "M45", &params.mlc_m45);
                bool_switch(ui, setter, "WARCLAW", &params.mlc_warclaw);
                ui.label("Feedback");
                feedback_switch(ui, setter, &params.mlc_feedback);
                ui.label("Gate");
                gate_pos_switch(ui, setter, &params.mlc_gate_pos);
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Gate").strong());
            param_knob(ui, setter, "Threshold", &params.mlc_gate);
        });
    });
}
