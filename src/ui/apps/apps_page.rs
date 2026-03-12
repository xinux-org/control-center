use crate::ui::window::AppMsg;
use crate::ui::apps::default_apps::DefaultAppsPage;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::gtk::gio;
use relm4::prelude::*;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub description: Option<String>,
    pub executable: Option<String>,
    pub icon: Option<gio::Icon>,
}

#[derive(Debug)]
pub struct AppModal {
    navigation: adw::NavigationView,
    apps_list: gtk::ListBox,
    default_apps: Controller<DefaultAppsPage>,
    apps: Vec<AppEntry>,
    filtered_apps: Vec<AppEntry>,
}

#[derive(Debug)]
pub enum AppsMsg {
    OpenDefaultApps,
    SearchChanged(String),
}

#[relm4::component(pub)]
impl SimpleComponent for AppModal {
    type Init = ();
    type Input = AppsMsg;
    type Output = AppMsg;

    view! {
        #[name = "navigation"]
        adw::NavigationView {
            add = &adw::NavigationPage {
                set_title: "Apps",

                adw::ToolbarView {
                    set_top_bar_style: adw::ToolbarStyle::Flat,

                    add_top_bar = &adw::HeaderBar {
                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: "Apps"
                        }
                    },

                    #[wrap(Some)]
                    set_content = &gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_hexpand: true,

                        #[wrap(Some)]
                        set_child = &adw::Clamp {
                            set_maximum_size: 900,
                            set_tightening_threshold: 600,

                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 12,
                                set_margin_top: 12,
                                set_margin_bottom: 12,
                                set_margin_start: 12,
                                set_margin_end: 12,

                                gtk::SearchEntry {
                                    set_placeholder_text: Some("Search apps"),
                                    connect_search_changed[sender] => move |entry| {
                                        sender.input(AppsMsg::SearchChanged(entry.text().to_string()));
                                    }
                                },

                                adw::PreferencesGroup {
                                    adw::ActionRow {
                                        set_title: "Default Apps",
                                        set_subtitle: "Set which apps open links, files, and media",
                                        set_activatable: true,
                                        connect_activated => AppsMsg::OpenDefaultApps,

                                        add_suffix = &gtk::Image {
                                            set_icon_name: Some("go-next-symbolic"),
                                            set_valign: gtk::Align::Center,
                                        }
                                    }
                                },

                                adw::PreferencesGroup {
                                    set_title: "Installed Apps",

                                    #[name = "apps_list"]
                                    gtk::ListBox {
                                        add_css_class: "boxed-list",
                                        set_selection_mode: gtk::SelectionMode::None,
                                    }
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let default_apps = DefaultAppsPage::builder().launch(()).detach();

        let apps = collect_apps();
        let filtered_apps = apps.clone();

        let model = Self {
            navigation: widgets.navigation.clone(),
            apps_list: widgets.apps_list.clone(),
            default_apps,
            apps,
            filtered_apps,
        };

        rebuild_apps_list(&model.apps_list, &model.filtered_apps);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppsMsg::OpenDefaultApps => {
                let page = self.default_apps.widget();
                self.navigation.push(page);
            }
            AppsMsg::SearchChanged(query) => {
                let query = query.trim().to_lowercase();

                if query.is_empty() {
                    self.filtered_apps = self.apps.clone();
                } else {
                    self.filtered_apps = self
                        .apps
                        .iter()
                        .filter(|app| {
                            app.name.to_lowercase().contains(&query)
                                || app
                                    .description
                                    .as_deref()
                                    .unwrap_or("")
                                    .to_lowercase()
                                    .contains(&query)
                                || app
                                    .executable
                                    .as_deref()
                                    .unwrap_or("")
                                    .to_lowercase()
                                    .contains(&query)
                        })
                        .cloned()
                        .collect();
                }
                rebuild_apps_list(&self.apps_list, &self.filtered_apps);
            }
        }
    }
}

fn collect_apps() -> Vec<AppEntry> {
    let mut apps: Vec<AppEntry> = gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .map(|app| AppEntry {
            name: app.display_name().to_string(),
            description: app.description().map(|s| s.to_string()),
            executable: Some(app.executable().to_string_lossy().into_owned()),
            icon: app.icon(),
        })
        .collect();

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    apps.dedup_by(|a, b| {
        a.name == b.name
            && a.executable.as_deref().unwrap_or("") == b.executable.as_deref().unwrap_or("")
    });

    apps
}

fn rebuild_apps_list(list: &gtk::ListBox, apps: &[AppEntry]) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }

    for app in apps {
        let row = adw::ActionRow::new();
        row.set_title(&app.name);

        if let Some(subtitle) = app
            .description
            .as_deref()
            .filter(|s| !s.is_empty())
            .or(app.executable.as_deref())
        {
            row.set_subtitle(subtitle);
        }

        let image = if let Some(icon) = &app.icon {
            gtk::Image::from_gicon(icon)
        } else {
            gtk::Image::from_icon_name("application-x-executable-symbolic")
        };

        image.set_pixel_size(24);
        image.set_valign(gtk::Align::Center);
        row.add_prefix(&image);

        let arrow = gtk::Image::from_icon_name("go-next-symbolic");
        arrow.set_valign(gtk::Align::Center);
        row.add_suffix(&arrow);

        list.append(&row);
    }
}
