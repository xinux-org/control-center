use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct WellbeingModel {}

#[relm4::component(pub)]
impl SimpleComponent for WellbeingModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle{
                    set_title: "Wellbeing"
                }
            },

            adw::PreferencesPage {

                adw::PreferencesGroup {
                    set_title: "Screen Time",

                    #[wrap(Some)]
                    set_header_suffix = &gtk::Button {
                        set_icon_name: "view-more-symbolic",
                        add_css_class: "flat",
                        set_valign: gtk::Align::Center,
                    },
                },

                adw::PreferencesGroup {
                    set_margin_top: 10,
                    set_title: "Screen Limits",

                    adw::SwitchRow {
                        set_title: "Screen Time Limit"
                    },

                    adw::ComboRow {
                        set_title: "Daily Limit",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "1 minute",
                            "2 minute",
                        ]),
                        set_selected: 0,
                    },

                    adw::SwitchRow {
                        set_title: "Gray Scala",
                        set_subtitle: "Black and white for screen limits",
                    },
                },

                adw::PreferencesGroup {
                    set_margin_top: 10,
                    set_title: "Break Reminders",

                    adw::SwitchRow {
                        set_title: "Eyesight Reminders",
                        set_subtitle: "Reminders to look away from the screen"
                    },

                    adw::SwitchRow {
                        set_title: "Movement Reminders",
                        set_subtitle: "Reminders to move around"
                    },

                    adw::ComboRow {
                        set_title: "Daily Limit",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "5 minutes / 30 minutes",
                            "60 minutes / 90 minutes",
                        ]),
                        set_selected: 0,
                    },

                    adw::SwitchRow {
                        set_title: "Sounds",
                        set_subtitle: "Play a sound when a break ends",
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
