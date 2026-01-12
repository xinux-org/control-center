use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SearchModal {}

#[derive(Debug)]
pub enum SearchAppMsg {
    Search,
}

#[relm4::component(pub)]
impl SimpleComponent for SearchModal {
    type Init = ();
    type Input = SearchAppMsg;
    type Output = AppMsg;

    view! {
        adw::NavigationView {
            add = &adw::NavigationPage {
                    adw::ToolbarView {
                    set_top_bar_style: adw::ToolbarStyle::Flat,
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

                                set_subtitle: "Include app-provided search results",

                                add_suffix = &gtk::Switch {
                                    set_active: true,
                                    set_valign: gtk::Align::Center,
                                }
                            },


                            adw::ActionRow {
                                set_title: "Search Locations",
                                set_activatable: true,
                                set_subtitle: "Filesystem locations which are searched by system apps",


                                add_suffix = &gtk::Button {
                                    set_icon_name: "go-next",
                                    add_css_class: "flat",

                                    set_valign: gtk::Align::Center,

                                    connect_clicked => SearchAppMsg::Search
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            SearchAppMsg::Search => {
                println!("IDK");
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

