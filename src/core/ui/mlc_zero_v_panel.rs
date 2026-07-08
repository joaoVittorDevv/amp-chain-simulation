use crate::core::state::plugin_params::{
    BaseIOParams, ClipType, MlcBright, MlcFeedback, MlcGatePos, MlcTab,
};
use nih_plug::prelude::{BoolParam, Enum, EnumParam, FloatParam, ParamSetter};
use nih_plug_egui::egui;
use std::sync::Arc;

/// Generic dropdown for any `EnumParam<T>` whose `T` derives nih_plug's `Enum`
/// (used for the Tone Stack / Tube / ADAA selectors, which have many variants).
fn enum_combo<T>(
    ui: &mut egui::Ui,
    setter: &ParamSetter,
    id: &str,
    width: f32,
    param: &EnumParam<T>,
) where
    T: Enum + PartialEq + Copy + 'static,
{
    let mut value = param.value();
    let names = T::variants();
    egui::ComboBox::from_id_salt(id)
        .width(width)
        .selected_text(names.get(value.to_index()).copied().unwrap_or(""))
        .show_ui(ui, |ui| {
            for (i, name) in names.iter().enumerate() {
                ui.selectable_value(&mut value, T::from_index(i), *name);
            }
        });
    if value != param.value() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, value);
        setter.end_set_parameter(param);
    }
}

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

fn clip_type_combo(ui: &mut egui::Ui, setter: &ParamSetter, id: &str, param: &EnumParam<ClipType>) {
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

pub fn draw_mlc_zero_v_panel(
    ui: &mut egui::Ui,
    setter: &ParamSetter,
    params: &Arc<BaseIOParams>,
    mlc_tab: &mut MlcTab,
) {
    // Tab bar. The controls are split across tabs so each tab's row fits the
    // panel width instead of overflowing horizontally.
    ui.horizontal(|ui| {
        ui.selectable_value(mlc_tab, MlcTab::Tone, "Tone");
        ui.selectable_value(mlc_tab, MlcTab::GainClip, "Gain/Clip");
        ui.selectable_value(mlc_tab, MlcTab::Harmonics, "Harmonics");
        ui.selectable_value(mlc_tab, MlcTab::PowerAmp, "Power Amp");
        ui.selectable_value(mlc_tab, MlcTab::Limiter, "Limiter");
    });
    ui.separator();

    match *mlc_tab {
        MlcTab::Tone => {
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("EQ").strong());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Tone Stack")
                                    .small()
                                    .color(egui::Color32::GRAY),
                            );
                            enum_combo(ui, setter, "mlc_ts_model", 130.0, &params.mlc_ts_model);
                        });
                        param_knob(ui, setter, "Bass", &params.mlc_bass);
                        param_knob(ui, setter, "Middle", &params.mlc_middle);
                        param_knob(ui, setter, "Treble", &params.mlc_treble);
                    });
                });
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Power Amp (EQ)").strong());
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
                    ui.label(egui::RichText::new("Gate").strong());
                    param_knob(ui, setter, "Threshold", &params.mlc_gate);
                });
            });
        }
        MlcTab::GainClip => {
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
                    ui.label(egui::RichText::new("Clean Blend").strong());
                    ui.vertical(|ui| {
                        param_slider(ui, setter, "Dry", &params.mlc_clean_blend);
                        param_slider(ui, setter, "Sag", &params.mlc_sag);
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
                    ui.label(egui::RichText::new("Tube").strong());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Model")
                                    .small()
                                    .color(egui::Color32::GRAY),
                            );
                            enum_combo(ui, setter, "mlc_tube_model", 110.0, &params.mlc_tube_model);
                            bool_switch(ui, setter, "Bypass", &params.mlc_tube_bypass);
                        });
                        param_knob_unit(ui, setter, "Drive", &params.mlc_tube_drive, 1, " dB");
                    });
                });
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Multi-Band Clip").strong());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            bool_switch(ui, setter, "Bypass", &params.mlc_mbc_bypass);
                            param_knob_unit(ui, setter, "XOver Lo", &params.mlc_mbc_cf_lo, 0, " Hz");
                            param_knob_unit(ui, setter, "XOver Hi", &params.mlc_mbc_cf_hi, 0, " Hz");
                        });
                        param_knob(ui, setter, "Drv Lo", &params.mlc_mbc_drive_lo);
                        param_knob(ui, setter, "Drv Mid", &params.mlc_mbc_drive_mid);
                        param_knob(ui, setter, "Drv Hi", &params.mlc_mbc_drive_hi);
                    });
                });
            });
        }
        MlcTab::Harmonics => {
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Chebyshev").strong());
                    ui.vertical(|ui| {
                        param_slider(ui, setter, "H2", &params.mlc_h2);
                        param_slider(ui, setter, "H3", &params.mlc_h3);
                        param_slider(ui, setter, "H4", &params.mlc_h4);
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
                    ui.label(egui::RichText::new("Quality (ADAA)").strong());
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Anti-alias order")
                                .small()
                                .color(egui::Color32::GRAY),
                        );
                        enum_combo(ui, setter, "mlc_adaa_order", 90.0, &params.mlc_adaa_order);
                    });
                });
            });
        }
        MlcTab::PowerAmp => {
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("NFB Loop").strong());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.add_space(14.0);
                            bool_switch(ui, setter, "Bypass", &params.mlc_nfb_bypass);
                        });
                        param_knob(ui, setter, "Presence", &params.mlc_nfb_presence);
                        param_knob(ui, setter, "Resonance", &params.mlc_nfb_resonance);
                        param_knob(ui, setter, "Depth", &params.mlc_nfb_depth);
                    });
                    ui.label(
                        egui::RichText::new(
                            "Real negative-feedback power amp. Presence boosts highs, \
                             Resonance boosts lows, Depth sets feedback amount.",
                        )
                        .small()
                        .color(egui::Color32::GRAY),
                    );
                });
            });
        }
        MlcTab::Limiter => {
            ui.horizontal_wrapped(|ui| {
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
    }
}
