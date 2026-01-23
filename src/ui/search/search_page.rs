use crate::app::AppMsg;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::search::search_locations::SearchLocationsPage;

// First page modal
#[derive(Debug)]
pub struct SearchModal {
    navigation: adw::NavigationView,
    search_locations: Controller<SearchLocationsPage>,
}

// First page message
#[derive(Debug)]
pub enum SearchAppMsg {
    OpenSearchLocations,
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
                            adw::SwitchRow {
                                set_title: "App Search",
                                set_subtitle: "Include app-provided search results",
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
