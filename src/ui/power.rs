use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub struct PowerModel {
    view_stack: adw::ViewStack,
    general_page: Controller<GeneralPowerPageView>,
    saving_page: Controller<SavingPowerPageView>,
    show_view_stack_bar: bool,
}

#[derive(Debug)]
pub enum PowerMsg {
    SetViewSwitchBar(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for PowerModel {
    type Init = ();
    type Input = PowerMsg;
    type Output = AppMsg;

    view! {
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                // A top bar when desktop mode
                #[wrap(Some)]
                set_title_widget = &adw::ViewSwitcher {
                    set_stack: Some(&view_stack),
                    set_policy: adw::ViewSwitcherPolicy::Wide,
                },
            },

            #[wrap(Some)]
            set_content = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_vexpand: true,

                adw::Clamp {
                    set_maximum_size: 600,
                    set_tightening_threshold: 400,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 12,
                        set_spacing: 24,

                        #[local_ref]
                        view_stack -> adw::ViewStack {
                            add: model.general_page.widget(),
                            add: model.saving_page.widget(),
                        },
                        // A bottom bar when on mobile. See more:
                        // https://github.com/blissd/fotema/blob/74160645a25d2cfe4ceb4f1935c247158ec3ab5f/src/app.rs#L401C52-L401C64
                        #[name(switcher_bar)]
                        adw::ViewSwitcherBar {
                            set_stack: Some(&view_stack),
                            #[track(model.show_view_stack_bar)]
                            // set_reveal: model.show_view_stack_bar,
                            set_reveal: true,
                            }
                        },
                    },
                },

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let view_stack = adw::ViewStack::new();
        let general_page = GeneralPowerPageView::builder().launch(()).detach();
        let saving_page = SavingPowerPageView::builder().launch(()).detach();
        
        let model = Self {
            view_stack: view_stack.clone(),
            general_page,
            saving_page,
            show_view_stack_bar: false,
        };
        let widgets = view_output!();
        
        let view_stack = model.view_stack.clone();
        let general_view_switcher = widgets.view_stack.page(model.general_page.widget());
        let saving_view_switcher = widgets.view_stack.page(model.saving_page.widget());

        general_view_switcher.set_title(Some("General"));
        general_view_switcher.set_name(Some("general")); // do not translate
        general_view_switcher.set_icon_name(Some("preferences-system-symbolic"));

        saving_view_switcher.set_title(Some("Power Saving"));
        saving_view_switcher.set_name(Some("power-saving")); // do not translate
        saving_view_switcher.set_icon_name(Some("battery-symbolic"));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PowerMsg::SetViewSwitchBar(vsbar) => {
                self.show_view_stack_bar = vsbar;
            }
        }
    }
}

// ------------------------------------------------------------------------------
#[derive(Debug)]
struct GeneralPowerPageView {
    power_mode: PowerMode,
    show_battery_percentage: bool,
}

#[derive(Debug)]
enum GeneralPowerPageViewMsg {
    SetPowerMode(PowerMode),
    ToggleBatteryPercentage(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Performance,
    Balanced,
    PowerSaver,
}

#[relm4::component]
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
                        set_spacing: 8,
                        set_margin_all: 16,

                        gtk::ProgressBar {
                            set_fraction: 1.0,
                            set_show_text: false,
                            set_hexpand: true,
                            add_css_class: "battery-bar",
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            gtk::Label {
                                set_label: "Fully charged",
                                set_halign: gtk::Align::Start,
                                set_hexpand: true,
                            },

                            gtk::Label {
                                set_label: "100 %",
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

                        add_prefix = &gtk::CheckButton {
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

                        add_prefix = &gtk::CheckButton {
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

                        add_prefix = &gtk::CheckButton {
                            #[watch]
                            set_active: model.power_mode == PowerMode::PowerSaver,
                            connect_toggled[sender] => move |btn| {
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
        let model = Self {
            power_mode: PowerMode::PowerSaver,
            show_battery_percentage: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            GeneralPowerPageViewMsg::SetPowerMode(mode) => {
                self.power_mode = mode;
            }
            GeneralPowerPageViewMsg::ToggleBatteryPercentage(state) => {
                self.show_battery_percentage = state;
            }
        }
    }
}
// -------------------------------------------------------------
#[derive(Debug)]
struct SavingPowerPageView {}

#[relm4::component]
impl Component for SavingPowerPageView {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Automic Power Saver",
                    set_subtitle: "Turn on power saver made when battery power is low",

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                }
            },

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Automatic Screen Blank",
                    set_subtitle: "Turn the screen off after a period of inactivity",

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                },

                adw::ComboRow {
                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&[
                        "1 minute",
                        "2 minute",
                        "3 minute",
                        "4 minute",
                        "5 minute",
                        "8 minute",
                        "10 minute",
                        "12 minute",
                        "15 minute",
                    ])),
                }
            },

            adw::PreferencesGroup {
                set_title: "Automatic Suspend",

                adw::ActionRow {
                    set_title: "On Battery Power",

                    add_suffix = &gtk::Switch{
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                },

                adw::ComboRow {
                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&[
                        "15 minute",
                        "20 minute",
                        "25 minute",
                        "30 minute",
                        "45 minute",
                        "1 hour",
                        "1 hour 20 minute",
                        "1 hour 30 minute",
                        "1 hour 40 minute",
                        "2 hours",
                    ])),
                }
            },

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "When plugged",

                    add_suffix = &gtk::Switch{
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                },

                adw::ComboRow {
                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&[
                        "15 minute",
                        "20 minute",
                        "25 minute",
                        "30 minute",
                        "45 minute",
                        "1 hour",
                        "1 hour 20 minute",
                        "1 hour 30 minute",
                        "1 hour 40 minute",
                        "2 hours",
                    ])),
                }
            },

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Disabling automatic suspend will result in higher power consumption. It is recomended to keep automatic suspend enabled.",

                    add_prefix = &gtk::Image {
                        set_icon_name: Some("issue-symbolic"),
                        set_pixel_size: 16,
                    }
                }
            }
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
    }
}
