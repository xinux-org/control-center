use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

//Second page modal
#[derive(Debug)]
pub struct DefaultAppsPage;

// Second page message
#[derive(Debug)]
pub enum DefaultAppsMsg {}

#[relm4::component(pub)]
impl SimpleComponent for DefaultAppsPage {
    type Init = ();
    type Input = DefaultAppsMsg;
    type Output = ();

    view! {
    adw::NavigationPage {
        set_title: "Search Locations",

        adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Default Apps"
                    }
                },

                gtk::Box {
                    set_spacing: 12,
                    set_halign: gtk::Align::Center,
                    set_orientation: gtk::Orientation::Vertical,

                    adw::PreferencesGroup {
                        set_title: "Default Apps",

                        adw::ComboRow {
                            set_title: "Web",
                            #[wrap(Some)]
                            set_model = &gtk::StringList::new(&[
                                "Firefox",
                                "Web",
                                ]),
                            set_selected: 0,
                        },

                        adw::ComboRow {
                            set_title: "Mail",
                            #[wrap(Some)]
                            set_model = &gtk::StringList::new(&[
                                "Geary"
                            ]),
                            set_selected: 0,
                        },

                        adw::ComboRow {
                            set_title: "Calendar",
                            #[wrap(Some)]
                            set_model = &gtk::StringList::new(&[
                                "Calendar"
                            ]),
                            set_selected: 0,
                        },

                        adw::ComboRow {
                            set_title: "Music",
                            #[wrap(Some)]
                            set_model = &gtk::StringList::new(&[
                                "Audio Player"
                            ]),
                            set_selected: 0,
                        },

                        adw::ComboRow {
                            set_title: "Video",
                            #[wrap(Some)]
                            set_model = &gtk::StringList::new(&[
                                "Video Player"
                            ]),
                            set_selected: 0,
                        },

                        adw::ComboRow {
                            set_title: "Photos",
                            #[wrap(Some)]
                            set_model = &gtk::StringList::new(&[
                                "Image Viewer",
                                "Gradia",
                                "feh",
                            ]),
                            set_selected: 0,
                        },

                    },

                    adw::PreferencesGroup {
                        set_title: "Removable Media",

                        adw::SwitchRow {
                            set_title: "Media Autostart",
                            set_subtitle: "Start apps or prompt when media is connected",
                        },
                    }
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DefaultAppsPage;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
