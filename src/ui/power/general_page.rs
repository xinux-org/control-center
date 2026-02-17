use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use ppd::PpdProxyBlocking;
use zbus::blocking::Connection;

use std::fmt;
use std::fs;
use std::sync::Arc;
use std::thread;

use notify::{Event, RecursiveMode, Result, Watcher};
use regex::Regex;
use std::{path::Path, sync::mpsc};

use crate::ui::power::power_page::PowerMsg;

use glib::ControlFlow;
use glib::source::timeout_add_seconds;

#[derive(Debug)]
#[tracker::track]
pub struct GeneralPowerPageView {
    pub power_mode: PowerMode,
    pub show_battery_percentage: bool,

    pub battery_percentage: String,
    pub battery_status: String,
    pub battery_percentage_float: f64,
    #[tracker::do_not_track]
    pub ppd: Arc<PpdProxyBlocking<'static>>,
}

impl fmt::Display for PowerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PowerMode::Performance => "performance",
            PowerMode::Balanced => "balanced",
            PowerMode::PowerSaver => "power-saver",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub enum GeneralPowerPageViewMsg {
    SetPowerMode(PowerMode),
    ToggleBatteryPercentage(bool),
    ChangeBattery,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Performance, // performance
    Balanced,    // balanced
    PowerSaver,  // power-saver
}

#[relm4::component(pub)]
impl Component for GeneralPowerPageView {
    type Init = ();
    type Input = GeneralPowerPageViewMsg;
    type Output = PowerMsg;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 24,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,

                gtk::Label {
                    set_label: "Battery Level",
                    set_halign: gtk::Align::Start,
                    add_css_class: "heading",
                },

                adw::PreferencesGroup {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::Center,
                        set_spacing: 8,
                        set_margin_all: 16,

                        add_css_class: "action-row",

                        gtk::LevelBar {
                            set_min_value: 1.0,
                            set_max_value: 100.0,

                            add_offset_value: ("low", 20.0),
                            add_offset_value: ("high", 60.0),
                            add_offset_value: ("full", 100.0),

                            #[watch]
                            set_value: model.battery_percentage_float,

                            set_hexpand: true,
                            add_css_class: "battery-bar",
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            gtk::Label {
                                #[watch]
                                set_label: model.battery_status.as_str(),
                                set_halign: gtk::Align::Start,
                                set_hexpand: true,
                            },

                            gtk::Label {
                                #[watch]
                                // #[track(model.changed(GeneralPowerPageView::battery_percentage()))]
                                set_label: model.battery_percentage.as_str(),
                                set_halign: gtk::Align::End,
                            },
                        },
                    },
                },
            },


            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,

                gtk::Label {
                    set_label: "Power Mode",
                    set_halign: gtk::Align::Start,
                    add_css_class: "heading",
                },

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Performance",
                        set_subtitle: "High performance and power usage",
                        set_activatable: true,

                        set_activatable_widget: Some(&activatable_performance),

                        #[name = "activatable_performance"]
                        add_prefix = &gtk::CheckButton {
                            set_group: Some(&activatable_balanced),

                            #[watch]
                            set_active: model.power_mode == PowerMode::Performance,
                            connect_toggled[sender] => move |btn| {
                                if btn.is_active() {
                                    sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::Performance));
                                }
                            },
                        },
                    },

                    adw::ActionRow {
                        set_title: "Balanced",
                        set_subtitle: "Standard performance and power usage",
                        set_activatable: true,

                        set_activatable_widget: Some(&activatable_balanced),

                        #[name = "activatable_balanced"]
                        add_prefix = &gtk::CheckButton {
                            set_group: Some(&activatable_powersaver),

                            #[watch]
                            set_active: model.power_mode == PowerMode::Balanced,
                            connect_toggled[sender] => move |btn| {
                                if btn.is_active() {
                                    sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::Balanced));
                                }
                            },
                        },
                    },

                    adw::ActionRow {
                        set_title: "Power Saver",
                        set_subtitle: "Reduced performance and power usage",
                        set_activatable: true,

                        set_activatable_widget: Some(&activatable_powersaver),

                        #[name = "activatable_powersaver"]
                        add_prefix = &gtk::CheckButton {
                            #[watch]
                            set_active: model.power_mode == PowerMode::PowerSaver,
                            connect_activate[sender] => move |btn| {
                                if btn.is_active() {
                                    sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::PowerSaver));
                                }
                            },
                        },
                    },
                },
            },


            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,

                gtk::Label {
                    set_label: "General",
                    set_halign: gtk::Align::Start,
                    add_css_class: "heading",
                },

                adw::PreferencesGroup {
                    adw::ComboRow {
                        set_title: "Power Button Behavior",
                        set_model: Some(&gtk::StringList::new(&["Suspend", "Shutdown", "Do Nothing"])),
                        set_selected: 0,
                    },

                    adw::ActionRow {
                        set_title: "Show Battery Percentage",
                        set_subtitle: "Show exact charge level in the top bar",

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_active: model.show_battery_percentage,
                            connect_state_set[sender] => move |_, state| {
                                sender.input(GeneralPowerPageViewMsg::ToggleBatteryPercentage(state));
                                gtk::glib::Propagation::Proceed
                            },
                        },
                    },
                },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let connection = Connection::system().unwrap();
        let proxy = PpdProxyBlocking::new(&connection).unwrap();

        let model = Self {
            power_mode: get_current_profile(&proxy),
            show_battery_percentage: false,

            // .unwrap() calls should be fixed ASAP!!
            ppd: Arc::new(proxy),
            battery_percentage: read_file("capacity", String::from("No battery"))
                .first()
                .unwrap_or(&String::from("No battery"))
                .to_string(),
            battery_status: read_file("status", String::from(""))
                .first()
                .unwrap_or(&String::from(""))
                .to_string(),
            battery_percentage_float: *get_battery_percentages_float(read_file(
                "capacity",
                String::from("No battery"),
            ))
            .get(1)
            .unwrap_or(&0.0),

            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            GeneralPowerPageViewMsg::SetPowerMode(mode) => {
                self.power_mode = mode;

                self.ppd
                    .set_active_profile(format!("{}", mode).trim().to_lowercase())
                    .unwrap();
            }
            GeneralPowerPageViewMsg::ToggleBatteryPercentage(state) => {
                self.show_battery_percentage = state;
            }
            GeneralPowerPageViewMsg::ChangeBattery => {
                self.battery_percentage = read_file("capacity", String::from("No battery"))
                    .first()
                    .unwrap()
                    .to_string();
                // It's a quick solution, will be fixed later
                self.battery_percentage_float = *get_battery_percentages_float(read_file(
                    "capacity",
                    String::from("No battery"),
                ))
                .get(1)
                .unwrap();
            }
        }
    }
}

fn get_current_profile(proxy: &PpdProxyBlocking) -> PowerMode {
    match proxy.active_profile().unwrap().trim() {
        "balanced" => PowerMode::Balanced,
        "power-saver" => PowerMode::PowerSaver,
        "performance" => PowerMode::Performance,
        _ => PowerMode::Balanced,
    }
}

fn get_battery_path() -> Vec<fs::DirEntry> {
    let global_path = Path::new("/sys/class/power_supply/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");

    let entries = match global_path.read_dir() {
        Ok(els) => els,
        Err(_) => return Vec::new(),
    };

    entries
        .filter_map(|el| el.ok())
        .filter(|el| re.is_match(el.path().to_str().unwrap()))
        .collect()
}

fn read_file(file_name: &str, no_entry: String) -> Vec<String> {
    let batteries = get_battery_path();

    batteries
        .iter()
        .map(|el| {
            fs::read_to_string(format!("{}/{}", el.path().display(), file_name))
                .unwrap_or(no_entry.clone())
        })
        .collect()
}
fn get_battery_percentages_float(els: Vec<String>) -> Vec<f64> {
    els.iter().map(|el| el.trim().parse().unwrap()).collect()
}
