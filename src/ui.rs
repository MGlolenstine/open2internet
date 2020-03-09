use druid::widget::{Flex, Label, List, Scroll, WidgetExt, Button, TextBox};
use druid::{Widget, Data, Lens, Color};
use std::sync::Arc;
use std::net::IpAddr;
use crate::utils::scan_ports;

#[derive(Clone, Data, Lens)]
pub struct O2IInfo {
    #[druid(same_fn = "PartialEq::eq")]
    pub local_address: IpAddr,
    #[druid(same_fn = "PartialEq::eq")]
    pub public_address: IpAddr,
    #[druid(same_fn = "PartialEq::eq")]
    pub ports: Arc<Vec<u16>>,
}

pub fn ui_builder() -> impl Widget<O2IInfo> {
    let public_address_label = Label::new(|data: &O2IInfo, _env: &_| {
        format!("Public IP  : {}", data.public_address)
    });
    let local_address_label = Label::new(|data: &O2IInfo, _env: &_| {
        format!("Local IP   : {}", data.local_address)
    });
    // let port_list = Scroll::new(List::new(||{Label::new(|item: &u16, _env: &_| format!("List item #{}", item))}).lens(O2IInfo::ports));
    let port_list = Scroll::new(List::new(|| {
        Label::new(|item: &u16, _env: &_| format!("Minecraft on port #{}", item))
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5))
    }))
    .vertical()
    .lens(O2IInfo::ports);
    let refresh_button = Button::new("Refresh", |_, data: &mut O2IInfo, _| {
        data.ports = Arc::new(scan_ports());
    });
    let mut lease_time = Flex::<O2IInfo>::row();
    let mut lease_time_label = Flex::column();
    lease_time_label.add_child(Label::new(|data: &O2IInfo, _env: &_| {
        format!("Lease time: ")
    }), 0.0);
    // let lease_time_input = Flex::column();
    // lease_time_input.add_child(TextBox::new(), 0.0);

    lease_time.add_child(lease_time_label, 0.0);
    // lease_time.add_child(lease_time_input, 0.0);
    let mut root = Flex::column();
    root.add_child(public_address_label, 0.0);
    root.add_child(local_address_label, 0.0);
    root.add_child(port_list, 0.0);
    root.add_child(refresh_button, 0.0);
    root
}