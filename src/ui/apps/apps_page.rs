use crate::app::AppMsg;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::apps::default_apps::DefaultAppsPage;

// First page modal
#[derive(Debug)]
pub struct AppModal {
    navigation: adw::NavigationView,
    default_apps: Controller<DefaultAppsPage>,
}

// First page message
#[derive(Debug)]
pub enum AppsMsg {
    OpenDefaultApps,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModal {
    type Init = ();
    type Input = AppsMsg;
    type Output = AppMsg;

    view! {
        #[name = "navigation"]
        adw::NavigationView{
            add = &adw::NavigationPage{
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle{
                    set_title: "Apps"
                }
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_width_request: 300,

                    gtk::SearchEntry {
                        set_placeholder_text: Some("Search apps"),
                    },
                },

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Default Apps",
                        set_activatable: true,
                        set_subtitle: "Set which apps open links, files, and media",

                        connect_activated => AppsMsg::OpenDefaultApps,

                        add_suffix = &gtk::Button {
                            set_icon_name: "go-next",
                            add_css_class: "flat",

                            set_valign: gtk::Align::Center,

                        }
                    },
                },
            }
        }}}

    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let default_apps = DefaultAppsPage::builder().launch(()).detach();

        let model = Self {
            navigation: widgets.navigation.clone(),
            default_apps,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppsMsg::OpenDefaultApps => {
                let page = self.default_apps.widget();
                self.navigation.push(page);
            }
        }
    }
}
