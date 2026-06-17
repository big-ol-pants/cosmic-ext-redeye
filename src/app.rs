use crate::blue_light::BlueLightFilter;
use cosmic::app::{Core, Task};
use cosmic::iced::core::window;
use cosmic::iced::window::Id;
use cosmic::iced::{Alignment, Length, Rectangle};
use cosmic::prelude::*;
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{self};

const APPLET_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/Redeye.svg");

pub struct App {
    core: Core,
    popup: Option<Id>,
    value: f32,
    filter: BlueLightFilter,
    status: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    PopupClosed(Id),
    Surface(cosmic::surface::Action),
    ValueChanged(f32),
}

impl cosmic::Application for App {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;

    const APP_ID: &str = "io.github.big-ol-pants.CosmicExtRedeye";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = App {
            core,
            popup: None,
            value: 0.0,
            filter: BlueLightFilter::default(),
            status: "Off".to_string(),
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Self::Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Surface(action) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(action),
                ));
            }
            Message::ValueChanged(value) => {
                self.value = value;
                self.status = match self.filter.set_strength(self.value) {
                    Ok(status) => status.to_string(),
                    Err(err) => err.to_string(),
                };
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let have_popup = self.popup;
        let button = self
            .core
            .applet
            .icon_button_from_handle(widget::icon::from_svg_bytes(APPLET_ICON).symbolic(true))
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = have_popup {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<App>(
                        move |state: &mut App| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);
                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                None,
                                None,
                                None,
                            );

                            popup_settings.positioner.anchor_rect = Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };

                            popup_settings
                        },
                        Some(Box::new(|state: &App| {
                            state.popup_view().map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        Element::from(self.core.applet.applet_tooltip::<Message>(
            button,
            "Redeye",
            self.popup.is_some(),
            Message::Surface,
            None,
        ))
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        self.popup_view()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl App {
    fn popup_view(&self) -> Element<'_, Message> {
        let slider = widget::slider(0.0..=100.0, self.value, Message::ValueChanged);
        let space_s = cosmic::theme::spacing().space_s;
        let label = widget::text(format!("Blue filter: {:.0}%", self.value));
        let status = widget::text(&self.status);
        let content = widget::column::with_capacity(3)
            .push(slider)
            .push(label)
            .push(status)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .spacing(space_s)
            .padding(16.0);

        Element::from(self.core.applet.popup_container(content))
    }
}
