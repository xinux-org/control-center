use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SearchModal {}

#[relm4::component(pub)]
impl SimpleComponent for SearchModal {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::PreferencesGroup {
            adw::HeaderBar {},

            adw::PreferencesPage {

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Do Not Disturb",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                },
                adw::ActionRow {
                    set_title: "Lock Screen Notifications",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                }
            },

            adw::PreferencesGroup {
                set_title: "App Notifications",

                gtk::SearchEntry {
                    set_placeholder_text: Some("Search"),
                },


                adw::ActionRow {
                    set_title: "Web",
                    set_activatable: true,

                    add_suffix = &gtk::Box {
                        set_spacing: 6,

                        gtk::Label {
                            set_label: "Off",
                            add_css_class: "dim-label",
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
