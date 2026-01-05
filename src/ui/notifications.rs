use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct NotificationsModel {}

#[relm4::component(pub)]
impl SimpleComponent for NotificationsModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle{
                    set_title: "Notifications"
                }
            },



        adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Do Not Disturb",
                    set_activatable: true,

                        add_suffix = &gtk::Switch {
                            set_active: true,
                            set_valign: gtk::Align::Center,
                        }
                    }
                },

                adw::PreferencesGroup {
                    set_title: "App Notifications",

                    adw::ActionRow {
                        set_title: "HuaweiNetwork",
                        set_subtitle: "Connected",
                        set_activatable: true,

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
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
