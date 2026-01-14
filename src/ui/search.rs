use crate::app::AppMsg;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub enum SearchAppMsg {
    OpenSearchLocations,
}

#[derive(Debug)]
pub struct SearchModal {
    navigation: adw::NavigationView,
    search_locations: Controller<SearchLocationsPage>,
}

#[derive(Debug)]
pub struct SearchLocationsPage;

#[derive(Debug)]
pub enum SearchLocationsMsg {}

#[relm4::component(pub)]
impl SimpleComponent for SearchLocationsPage {
    type Init = ();
    type Input = SearchLocationsMsg;
    type Output = ();

    view! {
    adw::NavigationPage {
        set_title: "Search Locations",

        adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Search Locations"
                    }
                },

                adw::PreferencesPage {
                    adw::PreferencesGroup{
                        gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "Filesystem locations which are selected by system apps, such as Files",
                            add_css_class: "grey_color",
                        },
                    },

                
                    // set_halign: gtk::Align::Fill,

                    adw::PreferencesGroup {
                        set_title: "Default Locations",
                        set_margin_top: 10,

                        adw::ActionRow {
                            set_title: "Home",
                            // set_hexpand: true,
                            set_subtitle: "Subfolders must be manually added for this locaiton",
                            // set_activatable: true,

                            add_suffix = &gtk::Switch {
                                set_active: true,
                                set_valign: gtk::Align::Center,
                            },
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Custom Locations",
                        // set_halign: gtk::Align::Center,

                        adw::ActionRow {
                            set_title: "Desktop",
                            set_subtitle: "Location not found",
                            // set_activatable: true,
                            // set_hexpand: true,

                            add_suffix = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,
                                // set_hexpand: true,

                                gtk::Button {
                                    set_icon_name: "document-open",
                                    add_css_class: "flat",
                                    // set_valign: gtk::Align::Center,
                                },

                                gtk::Button {
                                    set_icon_name: "edit-delete",
                                    add_css_class: "flat",
                                    set_valign: gtk::Align::Center,
                                }
                            }
                        },

                        adw::ActionRow {
                            set_title: "Add Location",
                            set_activatable: true,

                            add_prefix = &gtk::Image {
                                set_icon_name: Some("list-add-symbolic"),
                                set_valign: gtk::Align::Center,
                            }
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchLocationsPage;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for SearchModal {
    type Init = ();
    type Input = SearchAppMsg;
    type Output = AppMsg;

    view! {
        #[name = "navigation"]
        adw::NavigationView {
            add = &adw::NavigationPage {
                set_title: "Search",

                adw::ToolbarView {
                    set_top_bar_style: adw::ToolbarStyle::Flat,

                    add_top_bar = &adw::HeaderBar {
                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: "Search"
                        }
                    },

                    adw::PreferencesPage {
                        adw::PreferencesGroup {
                            adw::ActionRow {
                                set_title: "App Search",
                                set_subtitle: "Include app-provided search results",
                                set_activatable: false,

                                add_suffix = &gtk::Switch {
                                    set_active: true,
                                    set_valign: gtk::Align::Center,
                                }
                            },

                            adw::ActionRow {
                                set_title: "Search Locations",
                                set_subtitle: "Filesystem locations which are searched",
                                set_activatable: true,

                                connect_activated => SearchAppMsg::OpenSearchLocations,

                                add_suffix = &gtk::Image {
                                    set_icon_name: Some("go-next"),
                                    add_css_class: "flat",
                                    set_valign: gtk::Align::Center,
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let search_locations = SearchLocationsPage::builder().launch(()).detach();

        let model = SearchModal {
            navigation: widgets.navigation.clone(),
            search_locations,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            SearchAppMsg::OpenSearchLocations => {
                let page = self.search_locations.widget();
                self.navigation.push(page);
            }
        }
    }
}
