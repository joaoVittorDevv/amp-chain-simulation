use crate::core::state::plugin_params::{
    BaseIOParams, ClipType, MlcBright, MlcFeedback, MlcGatePos,
};
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

        // Absolute runtime value the DSP used on the last audio callback.
        let actual = param.smoothed.previous_value();
        ui.label(
            egui::RichText::new(format!("{:.2}", actual))
                .small()
                .monospace(),
        );
    });
}

/// Knob with a custom absolute-value readout (decimals + unit), for params
/// whose natural unit isn't a bare float (e.g. dB, ms).
fn param_knob_unit(
    ui: &mut egui::Ui,
    setter: &ParamSetter,
    label: &str,
    param: &FloatParam,
    decimals: usize,
    unit: &str,
) {
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

        // Absolute runtime value the DSP used on the last audio callback.
        let actual = param.smoothed.previous_value();
        ui.label(
            egui::RichText::new(format!("{:.*}{}", decimals, actual, unit))
                .small()
                .monospace(),
        );
    });
}

/// Horizontal slider bound to a `FloatParam`, for continuous 0..1-style controls
/// that read better as a bar than a knob (e.g. the even/odd harmonics balance).
fn param_slider(ui: &mut egui::Ui, setter: &ParamSetter, label: &str, param: &FloatParam) {
    let mut value = param.value();
    let (min, max) = match param.range() {
        nih_plug::prelude::FloatRange::Linear { min, max } => (min, max),
        nih_plug::prelude::FloatRange::Skewed { min, max, .. } => (min, max),
        _ => (0.0, 1.0),
    };
    let response = ui.add(
        egui::Slider::new(&mut value, min..=max)
            .text(label)
            .show_value(false),
    );
    if response.drag_started() {
        setter.begin_set_parameter(param);
    }
    if response.changed() {
        setter.set_parameter(param, value);
    }
    if response.drag_stopped() {
        setter.end_set_parameter(param);
    }
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

fn clip_type_combo(
    ui: &mut egui::Ui,
    setter: &ParamSetter,
    id: &str,
    param: &EnumParam<ClipType>,
) {
    let mut value = param.value();
    egui::ComboBox::from_id_salt(id)
        .width(130.0)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for clip in ClipType::ALL {
                ui.selectable_value(&mut value, clip, clip.label())
                    .on_hover_text(clip.description());
            }
        });
    if value != param.value() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, value);
        setter.end_set_parameter(param);
    }
}

/// Combo for the oversampling `IntParam` (0 = 1x, 1 = 2x, 2 = 4x).
fn ovs_combo(ui: &mut egui::Ui, setter: &ParamSetter, param: &nih_plug::prelude::IntParam) {
    use crate::core::state::plugin_params::ovs_factor_label;
    let mut value = param.value();
    egui::ComboBox::from_id_salt("mlc_ovs_combo")
        .width(70.0)
        .selected_text(ovs_factor_label(value))
        .show_ui(ui, |ui| {
            for v in 0..=2 {
                ui.selectable_value(&mut value, v, ovs_factor_label(v));
            }
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
            ui.label(egui::RichText::new("Gain + Clip").strong());
            ui.horizontal(|ui| {
                param_knob(ui, setter, "Gain", &params.mlc_gain);
                param_knob(ui, setter, "Master", &params.mlc_master);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Clip per stage")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                    ui.horizontal(|ui| {
                        ui.label("1");
                        clip_type_combo(ui, setter, "mlc_clip1", &params.mlc_clip_type1);
                    });
                    ui.horizontal(|ui| {
                        ui.label("2");
                        clip_type_combo(ui, setter, "mlc_clip2", &params.mlc_clip_type2);
                    });
                    ui.horizontal(|ui| {
                        ui.label("3");
                        clip_type_combo(ui, setter, "mlc_clip3", &params.mlc_clip_type3);
                    });
                });
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Oversampling").strong());
            ui.horizontal(|ui| {
                ui.label("Rate");
                ovs_combo(ui, setter, &params.mlc_ovs_factor);
            });
            ui.label(
                egui::RichText::new("Lanczos3 anti-alias")
                    .small()
                    .color(egui::Color32::GRAY),
            );
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Clean Blend").strong());
            ui.vertical(|ui| {
                param_slider(ui, setter, "Dry", &params.mlc_clean_blend);
                param_slider(ui, setter, "Sag", &params.mlc_sag);
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Chebyshev").strong());
            ui.vertical(|ui| {
                param_slider(ui, setter, "H2", &params.mlc_h2);
                param_slider(ui, setter, "H3", &params.mlc_h3);
                param_slider(ui, setter, "H4", &params.mlc_h4);
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Tight").strong());
            ui.horizontal(|ui| {
                bool_switch(ui, setter, "Enable", &params.mlc_tight);
                ui.label(
                    egui::RichText::new("HPF 80Hz entre estágios")
                        .small()
                        .color(egui::Color32::GRAY),
                );
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Harmonics").strong());
            ui.vertical(|ui| {
                bool_switch(ui, setter, "Enable", &params.mlc_asymmetry_enable);
                ui.horizontal(|ui| {
                    ui.label("odd");
                    param_slider(ui, setter, "Asymmetry", &params.mlc_asymmetry);
                    ui.label("even");
                });
            });
        });
        ui.group(|ui| {
            ui.label(egui::RichText::new("Pre-Shape").strong());
            ui.vertical(|ui| {
                bool_switch(ui, setter, "Enable", &params.mlc_preshape);
                ui.horizontal(|ui| {
                    param_knob(ui, setter, "Tight", &params.mlc_preshape_tight);
                    param_knob(ui, setter, "Bite", &params.mlc_preshape_bite);
                });
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
        ui.group(|ui| {
            ui.label(egui::RichText::new("LIMITER").strong());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add_space(14.0);
                    bool_switch(ui, setter, "Enable", &params.limiter_enable);
                });
                param_knob_unit(ui, setter, "Ceiling", &params.limiter_ceiling, 1, " dB");
                param_knob_unit(ui, setter, "Release", &params.limiter_release, 0, " ms");
            });
        });
    });
}
