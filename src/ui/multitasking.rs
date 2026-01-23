use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct MultitaskingModel {}

#[relm4::component(pub)]
impl SimpleComponent for MultitaskingModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Multitasking",
                }
            },

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "General",

                    adw::SwitchRow {
                        set_title: "Hot Corner",
                        set_subtitle: "Touch the top-left corner to open the Activities Overview",
                    },

                    adw::SwitchRow {
                        set_title: "Active Screen Edges",
                        set_subtitle: "Drag windows against the top, left, and right screen edges to resize them",
                    },
                },



                adw::PreferencesGroup {
                    set_title: "Workspaces",

                    adw::ActionRow {
                        set_title: "Dynamic Workspaces",
                        set_subtitle: "Automatically removes empty workspaces",
                        set_activatable: true,

                        add_prefix = &gtk::CheckButton {
                            set_active: false,

                        }
                    },

                    adw::ActionRow {
                        set_title: "Fixed Number of Workspaces",
                        set_subtitle: "Specify a number of permanent workspaces",
                        set_activatable: true,

                        add_prefix = &gtk::CheckButton {
                            set_active: false
                        }
                    },

                    adw::SpinRow {
                        set_title: "Number of Workspaces"
                    },
                },

                adw::PreferencesGroup {
                    set_title: "Multi-Monitor",

                    adw::ActionRow {
                        set_title: "Workspaces on primary display only",
                        set_activatable: true,

                        add_prefix = &gtk::CheckButton {
                            set_active: false
                        }
                    },

                    adw::ActionRow {
                        set_title: "Workspaces on all displays",
                        set_activatable: true,

                        add_prefix = &gtk::CheckButton {
                            set_active: false
                        }
                    },
                },

                adw::PreferencesGroup {
                    set_title: "App Switching",

                    adw::ActionRow {
                        set_title: "Include apps from all workspaces",
                        set_activatable: true,

                        add_prefix = &gtk::CheckButton {
                            set_active: false
                        }
                    },

                    adw::ActionRow {
                        set_title: "Include apps from the current workspace only",
                        set_activatable: true,

                        add_prefix = &gtk::CheckButton {
                            set_active: false
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
