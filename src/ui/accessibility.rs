use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AccessibilityModel {}

#[relm4::component(pub)]
impl SimpleComponent for AccessibilityModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Accessibility",
                }
            },

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    // set_title: "System",
                    
                    adw::SwitchRow {
                        set_title: "Always Show Accessibility Menu",
                        set_subtitle: "Display the accessibility menu in the top bar"
                    }
                },

                adw::PreferencesGroup {
                    // set_title: "Devices",

                    adw::ActionRow {
                        set_title: "Seeing",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-accessibility-seeing-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Hearing",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-accessibility-hearing-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Typing",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-accessibility-typing-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Pointing and Clicking",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-accessibility-pointing-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Zoom",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("oorg.gnome.Settings-accessibility-zoom-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
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
