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
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle{
                    set_title: "Search"
                }
            },

            adw::PreferencesPage {

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "App Search",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                },

                adw::ActionRow {
                    set_title: "Search Locations",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    }
                }
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
