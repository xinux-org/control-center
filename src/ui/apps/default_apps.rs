use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::gtk::{gio, glib};
use relm4::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct DefaultAppsPage {
    categories: Vec<CategoryState>,
}

#[derive(Debug, Clone)]
pub enum DefaultAppsMsg {
    CategoryChanged(DefaultCategory, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DefaultCategory {
    Web,
    Mail,
    Calendar,
    Music,
    Video,
    Photos,
}

#[derive(Debug, Clone)]
struct AppChoice {
    name: String,
    app_info: gio::AppInfo,
}

#[derive(Debug)]
struct CategoryState {
    kind: DefaultCategory,
    choices: Vec<AppChoice>,
}

impl DefaultCategory {
    fn title(self) -> &'static str {
        match self {
            Self::Web => "Web",
            Self::Mail => "Mail",
            Self::Calendar => "Calendar",
            Self::Music => "Music",
            Self::Video => "Video",
            Self::Photos => "Photos",
        }
    }

    fn mime_types(self) -> &'static [&'static str] {
        match self {
            Self::Web => &["text/html", "application/xhtml+xml"],
            Self::Mail => &["message/rfc822"],
            Self::Calendar => &["text/calendar"],
            Self::Music => &["audio/mpeg", "audio/flac", "audio/x-wav", "audio/ogg"],
            Self::Video => &[
                "video/mp4",
                "video/x-matroska",
                "video/webm",
                "video/x-msvideo",
            ],
            Self::Photos => &["image/jpeg", "image/png", "image/webp", "image/gif"],
        }
    }

    fn uri_schemes(self) -> &'static [&'static str] {
        match self {
            Self::Web => &["https", "http"],
            Self::Mail => &["mailto"],
            Self::Calendar => &[],
            Self::Music => &[],
            Self::Video => &[],
            Self::Photos => &[],
        }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for DefaultAppsPage {
    type Init = ();
    type Input = DefaultAppsMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: "Default Apps",

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Default Apps"
                    }
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,

                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        set_maximum_size: 720,
                        set_tightening_threshold: 560,

                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 18,
                            set_margin_top: 24,
                            set_margin_bottom: 24,
                            set_margin_start: 24,
                            set_margin_end: 24,

                            #[name = "default_apps_group"]
                            adw::PreferencesGroup {
                                set_title: "Default Apps",

                                #[name = "web_row"]
                                adw::ComboRow {
                                    set_title: "Web",
                                },

                                #[name = "mail_row"]
                                adw::ComboRow {
                                    set_title: "Mail",
                                },

                                #[name = "calendar_row"]
                                adw::ComboRow {
                                    set_title: "Calendar",
                                },

                                #[name = "music_row"]
                                adw::ComboRow {
                                    set_title: "Music",
                                },

                                #[name = "video_row"]
                                adw::ComboRow {
                                    set_title: "Video",
                                },

                                #[name = "photos_row"]
                                adw::ComboRow {
                                    set_title: "Photos",
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

        let mut model = DefaultAppsPage {
            categories: Vec::new(),
        };

        setup_category_row(
            &mut model,
            &widgets.web_row,
            DefaultCategory::Web,
            sender.clone(),
        );
        setup_category_row(
            &mut model,
            &widgets.mail_row,
            DefaultCategory::Mail,
            sender.clone(),
        );
        setup_category_row(
            &mut model,
            &widgets.calendar_row,
            DefaultCategory::Calendar,
            sender.clone(),
        );
        setup_category_row(
            &mut model,
            &widgets.music_row,
            DefaultCategory::Music,
            sender.clone(),
        );
        setup_category_row(
            &mut model,
            &widgets.video_row,
            DefaultCategory::Video,
            sender.clone(),
        );
        setup_category_row(
            &mut model,
            &widgets.photos_row,
            DefaultCategory::Photos,
            sender.clone(),
        );

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DefaultAppsMsg::CategoryChanged(kind, selected) => {
                if let Some(state) = self.categories.iter().find(|s| s.kind == kind) {
                    if let Some(choice) = state.choices.get(selected as usize) {
                        if let Err(err) = set_default_for_category(kind, &choice.app_info) {
                            eprintln!(
                                "Failed to set default app for {} to {}: {err}",
                                kind.title(),
                                choice.name
                            );
                        }
                    }
                }
            }
        }
    }
}

fn setup_category_row(
    model: &mut DefaultAppsPage,
    row: &adw::ComboRow,
    kind: DefaultCategory,
    sender: ComponentSender<DefaultAppsPage>,
) {
    let choices = collect_apps_for_category(kind);

    let selected = current_default_index(kind, &choices).unwrap_or(0) as u32;

    let names: Vec<&str> = if choices.is_empty() {
        vec!["No applications found"]
    } else {
        choices.iter().map(|choice| choice.name.as_str()).collect()
    };

    let string_list = gtk::StringList::new(&names);
    row.set_model(Some(&string_list));
    row.set_selected(selected.min((names.len().saturating_sub(1)) as u32));

    if choices.is_empty() {
        row.set_sensitive(false);
    } else {
        row.set_sensitive(true);

        row.connect_selected_notify(move |combo| {
            sender.input(DefaultAppsMsg::CategoryChanged(kind, combo.selected()));
        });
    }

    model.categories.push(CategoryState { kind, choices });
}

fn collect_apps_for_category(kind: DefaultCategory) -> Vec<AppChoice> {
    let mut map: BTreeMap<String, AppChoice> = BTreeMap::new();

    for mime in kind.mime_types() {
        for app in gio::AppInfo::all_for_type(mime) {
            if !app.should_show() {
                continue;
            }

            let key = app
                .id()
                .map(|s| s.to_string())
                .unwrap_or_else(|| app.display_name().to_string().to_lowercase());

            map.entry(key).or_insert_with(|| AppChoice {
                name: app.display_name().to_string(),
                app_info: app,
            });
        }
    }

    for scheme in kind.uri_schemes() {
        let handler = format!("x-scheme-handler/{scheme}");

        for app in gio::AppInfo::all_for_type(&handler) {
            if !app.should_show() {
                continue;
            }

            let key = app
                .id()
                .map(|s| s.to_string())
                .unwrap_or_else(|| app.display_name().to_string().to_lowercase());

            map.entry(key).or_insert_with(|| AppChoice {
                name: app.display_name().to_string(),
                app_info: app,
            });
        }
    }

    let mut apps: Vec<AppChoice> = map.into_values().collect();
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn current_default_index(kind: DefaultCategory, choices: &[AppChoice]) -> Option<usize> {
    let current = current_default_app(kind)?;

    let current_id = current.id().map(|s| s.to_string());
    let current_name = current.display_name().to_string();

    choices.iter().position(|choice| {
        choice.app_info.id().map(|s| s.to_string()) == current_id
            || choice.app_info.display_name() == current_name
    })
}

fn current_default_app(kind: DefaultCategory) -> Option<gio::AppInfo> {
    for scheme in kind.uri_schemes() {
        if let Some(app) = gio::AppInfo::default_for_uri_scheme(scheme) {
            return Some(app);
        }
    }

    for mime in kind.mime_types() {
        if let Some(app) = gio::AppInfo::default_for_type(mime, false) {
            return Some(app);
        }
    }

    None
}

fn set_default_for_category(kind: DefaultCategory, app: &gio::AppInfo) -> Result<(), glib::Error> {
    for scheme in kind.uri_schemes() {
        let handler = format!("x-scheme-handler/{scheme}");
        app.set_as_default_for_type(&handler)?;
    }

    for mime in kind.mime_types() {
        app.set_as_default_for_type(mime)?;
    }

    Ok(())
}
