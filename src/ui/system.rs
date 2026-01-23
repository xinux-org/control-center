use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SystemPageModel {}

#[relm4::component(pub)]
impl SimpleComponent for SystemPageModel {
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
                    adw::ActionRow {
                        set_title: "Region and Language",
                        set_subtitle: "System language and localization",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("emoji-flags-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Date and Time",
                        set_subtitle: "Time zone and clock settings",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("preferences-system-time-symbolic"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },

                    adw::ActionRow {
                        set_title: "Users",
                        set_subtitle: "Add and remove accounts, change password",
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

                    adw::ActionRow {
                        set_title: "Secure Shell",
                        set_subtitle: "SSH network access",
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
                        set_title: "About",
                        set_subtitle: "Hardware details and software versions",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("dialog-warning-symbolicc"),
                            set_pixel_size: 16
                        },

                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },
                },
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
