use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SharingModel {}

#[relm4::component(pub)]
impl SimpleComponent for SharingModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle{
                    set_title: "Sharing"
                }
            },

            adw::PreferencesPage {

                adw::PreferencesGroup {
                    adw::EntryRow {
                        set_title: "Device Name",
                    },

                },

                adw::PreferencesGroup {
                    set_margin_top: 10,
                    set_title: "Connect an Account",

                    adw::ActionRow {
                        set_title: "File Sharing",
                        set_subtitle: "Share files with other devices on the current netwrok",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("printer-network-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Label {
                                set_label: "Active",
                                add_css_class: "flat",
                                set_valign: gtk::Align::Center,
                            },

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Media Sharing",
                        set_subtitle: "Stream music, photos and videos to device on the current network",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("folder-music-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Label {
                                set_label: "Off",
                                add_css_class: "flat",
                                set_valign: gtk::Align::Center,
                            },

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
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
