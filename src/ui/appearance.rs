use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

// #[derive(Debug, Clone, Copy)]
// pub struct BackgroundPreview {
//     picture: std::path::Path,
// }
// impl BackgroundPreview {
//     pub fn set_picture(&self, path: std::path::Path){
//         self.picture = path;
//     }

// }

// #[relm4::widget_template]
// impl WidgetTemplate for BackgroundPreview {
//     view! {
//         gtk::Box {
//             set_margin_all: 10,
//             inline_css: format!("background-image: {}", &self.picture) ,
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct AppearanceModel {}

#[relm4::component(pub)]
impl SimpleComponent for AppearanceModel {
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
                    set_title: "Appearance",
                }
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "Style",

                    adw::ActionRow {
                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 24,
                            set_homogeneous: true,
                            set_hexpand: true,
                            set_margin_top: 18,
                            set_margin_bottom: 18,
                            set_margin_start: 86,
                            set_margin_end: 86,

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 12,


                                append = &gtk::Frame {
                                    #[wrap(Some)]
                                    set_child = &gtk::Picture::for_filename("/home/shahruz/.config/background"),
                                },

                                append = &gtk::Label {
                                   set_label: "Default",
                                   set_halign: gtk::Align::Center,
                                   set_hexpand: true,
                                },
                            },

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 12,

                                append = &gtk::Frame {
                                    set_hexpand: true,

                                    #[wrap(Some)]
                                    set_child = &gtk::Picture::for_filename("/home/shahruz/.config/background"),
                                },

                                append = &gtk::Label {
                                  set_label: "Dark",
                                  set_halign: gtk::Align::Center,
                                },
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
