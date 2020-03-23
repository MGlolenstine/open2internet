// use iced::{
//     button, scrollable, slider, text_input, Align, Button, Checkbox, Column, Container, Element,
//     Length, ProgressBar, Radio, Row, Sandbox, Scrollable, Slider, Space, Text, TextInput,
//     HorizontalAlignment, VerticalAlignment
// };
// use crate::utils::{redirect_minecraft_to_a_port, scan_ports};

// use style::Theme;

// #[derive(PartialEq, Eq, Clone, Debug)]
// struct O2IInfo{
//     port_number: u32,
//     version: String,
//     more_info: String,
// }

// // impl Copy for O2IInfo{
// //     fn copy(&self) -> O2IInfo{
// //         O2IInfo{
// //             port_number: self.port_number,
// //             version: self.version.clone(),
// //             more_info: self.more_info.clone()
// //         }
// //     }
// // }

// impl Default for O2IInfo{
//     fn default() -> O2IInfo{
//         O2IInfo{
//             port_number: 0,
//             version: "Unknown".to_string(),
//             more_info: "None".to_string()
//         }
//     }
// }

// #[derive(Default)]
// pub struct Styling {
//     theme: style::Theme,
//     lease_time: text_input::State,
//     lease_time_value: String,
//     global_port_input: text_input::State,
//     global_port_input_value: String,
//     scroll: scrollable::State,
//     minecraft_ports: Vec<O2IInfo>,
//     refresh_button: button::State,
//     open_port_button: button::State,
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     RefreshPorts,
//     OpenPort,
//     ItemSelected(O2IInfo),
//     LeaseTimeChanged(String),
//     GlobalPortChanged(String),
// }

// impl Sandbox for Styling {
//     type Message = Message;

//     fn new() -> Self {
//         let mut s = Styling::default();
//         s.theme = Theme::Dark;
//         s
//     }

//     fn title(&self) -> String {
//         String::from("Open 2 Internet - Open Minecraft LAN worlds to the WWW")
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::RefreshPorts => {
//                 // println!("You requested a port refresh and that's what you're getting!")
//                 let mut ports = scan_ports();
//                 ports.sort();
//                 self.minecraft_ports.clear();
//                 println!("Found {} ports!", ports.len());
//                 for port in ports {
//                     self.minecraft_ports.push(O2IInfo{port_number: port.into(), ..Default::default()})
//                     // self.scroll
//                 }
//             },
//             Message::OpenPort => {
//                 // println!("redirect_minecraft_to_a_port({}, {}, {})", 39695, self.global_port_input_value.parse::<u16>().unwrap(), self.lease_time_value.parse::<u32>().unwrap());
//                 redirect_minecraft_to_a_port(39695, self.global_port_input_value.parse::<u16>().unwrap(), self.lease_time_value.parse::<u32>().unwrap())
//             },
//             Message::LeaseTimeChanged(v) => {
//                 if v.trim().parse::<i32>().is_ok() || v.trim() == "" {
//                     self.lease_time_value = v;
//                 }
//             },
//             Message::GlobalPortChanged(v) => {
//                 if v.trim().parse::<u16>().is_ok() || v.trim() == "" {
//                     self.global_port_input_value = v;
//                 }
//             },
//             Message::ItemSelected(v) => {
                
//             },
//             _ => {}
//         }
//     }

//     fn view(&mut self) -> Element<Message> {
//         let input_names = Column::new().push(
//             Text::new("Lease time: ")
//             .horizontal_alignment(HorizontalAlignment::Left)
//             .vertical_alignment(VerticalAlignment::Center)
//             .height(Length::from(40))
//             .size(20)
//         )
//         .push(
//             Text::new("Public port: ")
//             .horizontal_alignment(HorizontalAlignment::Left)
//             .vertical_alignment(VerticalAlignment::Center)
//             .height(Length::from(40))
//             .size(20)
//         );
        
//         let input_text = Column::new().push(
//             TextInput::new(
//                 &mut self.lease_time,
//                 "Lease time in seconds.",
//                 &self.lease_time_value,
//                 Message::LeaseTimeChanged,
//             )
//             .padding(10)
//             .size(20)
//             .style(self.theme),
//         )
//         .push(
//             TextInput::new(
//                 &mut self.global_port_input,
//                 "Global port.",
//                 &self.global_port_input_value,
//                 Message::GlobalPortChanged,
//             )
//             .padding(10)
//             .size(20)
//             .style(self.theme)
//         );
//         let number_inputs = Row::new().push(input_names).push(input_text);

//         // println!("There's {} ports available!", self.minecraft_ports.len());
//         let scrollable: Row<Message> = if self.minecraft_ports.len() == 0 {
//             Row::new().push(Scrollable::new(&mut self.scroll)
//                 .padding(5)
//                 .push(
//                     Text::new("Collecting port data...").width(Length::Fill),
//                 ))
//                 // .push(Space::with_height(Length::Fill))
//                 .height(Length::from(460))
//                 .width(Length::Fill)
//         }else{
//             // let choose_theme = self.minecraft_ports.iter().fold(
//             //     Column::new().spacing(10).push(Text::new("Choose a port to forward:")),
//             //     |column, port| {
//             //         column.push(
//             //             Radio::new(
//             //                 *port,
//             //                 &format!("Minecraft opened at port {}", port.port_number),
//             //                 None,
//             //                 |v|{println!("Selected port {}", v.port_number)}
//             //                 // Message::ItemSelected(*port),
//             //             )
//             //             .style(self.theme),
//             //         )
//             //     },
//             // );
//             // Row::new().push(Scrollable::new(&mut self.scroll)
//             //     .padding(5)
//             //     .push(
//             //         choose_theme
//             //     ))
//             //     // .push(Space::with_height(Length::Fill))
//             //     .height(Length::from(460))
//             //     .width(Length::Fill)
//             let mut content: Column<Message> = Column::new();
//             for port in &self.minecraft_ports {
//                 let text = Text::new(format!("Minecraft opened at port {}", port.port_number));
//                 content = content.push(text);
//             }
//             Row::new().push(Scrollable::new(&mut self.scroll)
//                 .padding(5)
//                 .push(
//                     content
//                 ))
//                 .height(Length::from(460))
//                 // .push(Space::with_height(Length::Fill))
//                 .width(Length::Fill)
//         };

//         let buttons = Row::new().push(Button::new(&mut self.refresh_button, Text::new("Refresh"))
//                 .padding(10)
//                 .on_press(Message::RefreshPorts)
//                 .style(self.theme)
//             )
//             .push(Space::with_width(Length::Fill))
//             .push(
//                 Button::new(&mut self.open_port_button, Text::new("Open port"))
//                 .padding(10)
//                 .on_press(Message::OpenPort)
//                 .style(self.theme)
//         );
//         let content = Column::new().spacing(10).push(number_inputs).push(scrollable).push(buttons);
//         Container::new(content)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y()
//             .style(self.theme)
//             .into()
//     }
// }

// mod style {
//     use iced::{button, checkbox, container, progress_bar, radio, scrollable, slider, text_input};

//     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//     pub enum Theme {
//         Light,
//         Dark,
//     }

//     impl Theme {
//         pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
//     }

//     impl Default for Theme {
//         fn default() -> Theme {
//             Theme::Light
//         }
//     }

//     impl From<Theme> for Box<dyn container::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::Container.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn radio::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::Radio.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn text_input::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::TextInput.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn button::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => light::Button.into(),
//                 Theme::Dark => dark::Button.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn scrollable::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::Scrollable.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn slider::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::Slider.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::ProgressBar.into(),
//             }
//         }
//     }

//     impl From<Theme> for Box<dyn checkbox::StyleSheet> {
//         fn from(theme: Theme) -> Self {
//             match theme {
//                 Theme::Light => Default::default(),
//                 Theme::Dark => dark::Checkbox.into(),
//             }
//         }
//     }

//     mod light {
//         use iced::{button, Background, Color, Vector};

//         pub struct Button;

//         impl button::StyleSheet for Button {
//             fn active(&self) -> button::Style {
//                 button::Style {
//                     background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
//                     border_radius: 12,
//                     shadow_offset: Vector::new(1.0, 1.0),
//                     text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
//                     ..button::Style::default()
//                 }
//             }

//             fn hovered(&self) -> button::Style {
//                 button::Style {
//                     text_color: Color::WHITE,
//                     shadow_offset: Vector::new(1.0, 2.0),
//                     ..self.active()
//                 }
//             }
//         }
//     }

//     mod dark {
//         use iced::{
//             button, checkbox, container, progress_bar, radio, scrollable, slider, text_input,
//             Background, Color,
//         };

//         const SURFACE: Color = Color::from_rgb(
//             0x40 as f32 / 255.0,
//             0x44 as f32 / 255.0,
//             0x4B as f32 / 255.0,
//         );

//         const ACCENT: Color = Color::from_rgb(
//             0x6F as f32 / 255.0,
//             0xFF as f32 / 255.0,
//             0xE9 as f32 / 255.0,
//         );

//         const ACTIVE: Color = Color::from_rgb(
//             0x72 as f32 / 255.0,
//             0x89 as f32 / 255.0,
//             0xDA as f32 / 255.0,
//         );

//         const HOVERED: Color = Color::from_rgb(
//             0x67 as f32 / 255.0,
//             0x7B as f32 / 255.0,
//             0xC4 as f32 / 255.0,
//         );

//         pub struct Container;

//         impl container::StyleSheet for Container {
//             fn style(&self) -> container::Style {
//                 container::Style {
//                     background: Some(Background::Color(Color::from_rgb8(0x36, 0x39, 0x3F))),
//                     text_color: Some(Color::WHITE),
//                     ..container::Style::default()
//                 }
//             }
//         }

//         pub struct Radio;

//         impl radio::StyleSheet for Radio {
//             fn active(&self) -> radio::Style {
//                 radio::Style {
//                     background: Background::Color(SURFACE),
//                     dot_color: ACTIVE,
//                     border_width: 1,
//                     border_color: ACTIVE,
//                 }
//             }

//             fn hovered(&self) -> radio::Style {
//                 radio::Style {
//                     background: Background::Color(Color { a: 0.5, ..SURFACE }),
//                     ..self.active()
//                 }
//             }
//         }

//         pub struct TextInput;

//         impl text_input::StyleSheet for TextInput {
//             fn active(&self) -> text_input::Style {
//                 text_input::Style {
//                     background: Background::Color(SURFACE),
//                     border_radius: 2,
//                     border_width: 0,
//                     border_color: Color::TRANSPARENT,
//                 }
//             }

//             fn focused(&self) -> text_input::Style {
//                 text_input::Style {
//                     border_width: 1,
//                     border_color: ACCENT,
//                     ..self.active()
//                 }
//             }

//             fn hovered(&self) -> text_input::Style {
//                 text_input::Style {
//                     border_width: 1,
//                     border_color: Color { a: 0.3, ..ACCENT },
//                     ..self.focused()
//                 }
//             }

//             fn placeholder_color(&self) -> Color {
//                 Color::from_rgb(0.4, 0.4, 0.4)
//             }

//             fn value_color(&self) -> Color {
//                 Color::WHITE
//             }
//         }

//         pub struct Button;

//         impl button::StyleSheet for Button {
//             fn active(&self) -> button::Style {
//                 button::Style {
//                     background: Some(Background::Color(ACTIVE)),
//                     border_radius: 3,
//                     text_color: Color::WHITE,
//                     ..button::Style::default()
//                 }
//             }

//             fn hovered(&self) -> button::Style {
//                 button::Style {
//                     background: Some(Background::Color(HOVERED)),
//                     text_color: Color::WHITE,
//                     ..self.active()
//                 }
//             }

//             fn pressed(&self) -> button::Style {
//                 button::Style {
//                     border_width: 1,
//                     border_color: Color::WHITE,
//                     ..self.hovered()
//                 }
//             }
//         }

//         pub struct Scrollable;

//         impl scrollable::StyleSheet for Scrollable {
//             fn active(&self) -> scrollable::Scrollbar {
//                 scrollable::Scrollbar {
//                     background: Some(Background::Color(SURFACE)),
//                     border_radius: 2,
//                     border_width: 0,
//                     border_color: Color::TRANSPARENT,
//                     scroller: scrollable::Scroller {
//                         color: ACTIVE,
//                         border_radius: 2,
//                         border_width: 0,
//                         border_color: Color::TRANSPARENT,
//                     },
//                 }
//             }

//             fn hovered(&self) -> scrollable::Scrollbar {
//                 let active = self.active();

//                 scrollable::Scrollbar {
//                     background: Some(Background::Color(Color { a: 0.5, ..SURFACE })),
//                     scroller: scrollable::Scroller {
//                         color: HOVERED,
//                         ..active.scroller
//                     },
//                     ..active
//                 }
//             }

//             fn dragging(&self) -> scrollable::Scrollbar {
//                 let hovered = self.hovered();

//                 scrollable::Scrollbar {
//                     scroller: scrollable::Scroller {
//                         color: Color::from_rgb(0.85, 0.85, 0.85),
//                         ..hovered.scroller
//                     },
//                     ..hovered
//                 }
//             }
//         }

//         pub struct Slider;

//         impl slider::StyleSheet for Slider {
//             fn active(&self) -> slider::Style {
//                 slider::Style {
//                     rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
//                     handle: slider::Handle {
//                         shape: slider::HandleShape::Circle { radius: 9 },
//                         color: ACTIVE,
//                         border_width: 0,
//                         border_color: Color::TRANSPARENT,
//                     },
//                 }
//             }

//             fn hovered(&self) -> slider::Style {
//                 let active = self.active();

//                 slider::Style {
//                     handle: slider::Handle {
//                         color: HOVERED,
//                         ..active.handle
//                     },
//                     ..active
//                 }
//             }

//             fn dragging(&self) -> slider::Style {
//                 let active = self.active();

//                 slider::Style {
//                     handle: slider::Handle {
//                         color: Color::from_rgb(0.85, 0.85, 0.85),
//                         ..active.handle
//                     },
//                     ..active
//                 }
//             }
//         }

//         pub struct ProgressBar;

//         impl progress_bar::StyleSheet for ProgressBar {
//             fn style(&self) -> progress_bar::Style {
//                 progress_bar::Style {
//                     background: Background::Color(SURFACE),
//                     bar: Background::Color(ACTIVE),
//                     border_radius: 10,
//                 }
//             }
//         }

//         pub struct Checkbox;

//         impl checkbox::StyleSheet for Checkbox {
//             fn active(&self, is_checked: bool) -> checkbox::Style {
//                 checkbox::Style {
//                     background: Background::Color(if is_checked { ACTIVE } else { SURFACE }),
//                     checkmark_color: Color::WHITE,
//                     border_radius: 2,
//                     border_width: 1,
//                     border_color: ACTIVE,
//                 }
//             }

//             fn hovered(&self, is_checked: bool) -> checkbox::Style {
//                 checkbox::Style {
//                     background: Background::Color(Color {
//                         a: 0.8,
//                         ..if is_checked { ACTIVE } else { SURFACE }
//                     }),
//                     ..self.active(is_checked)
//                 }
//             }
//         }
//     }
// }
