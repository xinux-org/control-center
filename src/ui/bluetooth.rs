use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BluetoothModel {}

#[relm4::component(pub)]
impl SimpleComponent for BluetoothModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Bluetooth",
                },

                pack_end = &gtk::Switch {
                    set_active: true,
                    set_valign: gtk::Align::Center,
                },
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "Devices",
                    adw::ActionRow {
                        set_title: "Wi-Fi",
                        set_activatable: true,

                        add_suffix = &gtk::Label {
                            set_label: "Disconected",
                        }
                    }
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
