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
                    set_title: "System",
                }
            },

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "System",
                    
                    adw::ActionRow {
                        set_title: "Screen Lock",
                        set_subtitle: "Automatic screen lock",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("channel-secure-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Location",
                        set_subtitle: "Control access to your location",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-location-access-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "File History and Trash",
                        set_subtitle: "Remove saved data and files",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-users-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },
                },

                adw::PreferencesGroup {
                    set_title: "Devices",

                    adw::ActionRow {
                        set_title: "Cameras",
                        set_subtitle: "Control camera acess",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-secure-shell-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Device Security",
                        set_subtitle: "Hardware security status and information",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("org.gnome.Settings-device-security-symbolic"),
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
