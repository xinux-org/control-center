use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct MouseAndTouchpad {}

#[relm4::component(pub)]
impl SimpleComponent for MouseAndTouchpad {
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
                    set_title: "Mouse and Touchpad",
                }
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "General",

                    adw::ActionRow {
                        set_title: "Scale",

                        add_suffix = &gtk::Box {
                            set_spacing: 8,
                            set_halign: gtk::Align::End,

                            gtk::ToggleButton {
                                set_label: "100 %",
                                // set_active: true,
                                // add_css_class: "flat",
                            },

                            gtk::ToggleButton {
                                set_label: "200 %",
                                // add_css_class: "flat",
                            },
                        }
                    },
                },

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Night Light",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("night-light-symbolic"),
                        },

                        add_suffix = &gtk::Box {
                            set_spacing: 12,
                            set_halign: gtk::Align::End,

                            gtk::Label {
                                set_label: "Off",
                                add_css_class: "dim-label",
                            },

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                            },
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
