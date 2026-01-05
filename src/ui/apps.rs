use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AppModal {}

#[relm4::component(pub)]
impl SimpleComponent for AppModal {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::PreferencesGroup {
            adw::HeaderBar {},

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_width_request: 300,

                    gtk::SearchEntry {
                        set_placeholder_text: Some("Search"),
                    },
                },

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Default Apps",
                        set_activatable: true,
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
