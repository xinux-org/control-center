use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub struct SoundModel {}

#[relm4::component(pub)]
impl SimpleComponent for SoundModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        #[root]
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Sound",
                },
            },

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "Output",

                    #[wrap(Some)]
                    set_header_suffix = &gtk::Button {
                        set_label: "Test...",
                    },

                    adw::ComboRow {
                        set_title: "Output Device",

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("audio-headphones-symbolic"),
                            set_pixel_size: 16,
                        },

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "Headphones - Family 17h/19h/1ah HD Audio Controller",
                            "Built-in Audio Analog Stereo",
                        ]),
                        set_selected: 0,
                    },

                    adw::ActionRow {
                        set_title: "Output Volume",
                        set_activatable: false,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("audio-volume-high-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Scale {
                            set_hexpand: true,
                            set_valign: gtk::Align::Center,
                            set_width_request: 300,
                            set_draw_value: false,
                            set_range: (0.0, 100.0),
                            set_value: 50.0,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Balance",
                        set_activatable: false,

                        add_suffix = &gtk::Scale {
                            set_hexpand: true,
                            set_valign: gtk::Align::Center,
                            set_width_request: 300,
                            set_draw_value: false,
                            set_range: (-1.0, 1.0),
                            set_value: 0.0,
                            add_css_class: "balance-scale",
                            add_mark: (0.0, gtk::PositionType::Bottom, None),
                        }
                    },
                },

                adw::PreferencesGroup {
                    set_title: "Input",

                    #[wrap(Some)]
                    set_header_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    },

                    adw::ComboRow {
                        set_title: "Input Device",

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("audio-input-microphone-symbolic"),
                            set_pixel_size: 16,
                        },

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "Internal Microphone - Family 17h/19h/1ah HD Audio Con...",
                            "Built-in Audio Analog Stereo",
                            "External Microphone"
                        ]),
                        set_selected: 0,
                    },

                    adw::ActionRow {
                        set_title: "Input Volume",
                        set_activatable: false,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("audio-input-microphone-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Scale {
                            set_hexpand: true,
                            set_valign: gtk::Align::Center,
                            set_width_request: 300,
                            set_draw_value: false,
                            set_range: (0.0, 100.0),
                            set_value: 50.0,
                        }
                    },
                },

                adw::PreferencesGroup {
                    set_title: "Sounds",

                    adw::ActionRow {
                        set_title: "Volume levels",
                        set_activatable: true,

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Alert Sound",
                        set_activatable: true,

                        add_suffix = &gtk::Box {
                            set_spacing: 12,
                            set_halign: gtk::Align::End,

                            gtk::Label {
                                set_label: "Click",
                                add_css_class: "dim-label",
                            },

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            },
                        }
                    },
                }
            }
        }
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
}
