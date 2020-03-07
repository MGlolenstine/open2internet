use crate::utils::*;
#[allow(dead_code)]
use glib::types::StaticType;
use gtk::prelude::GtkListStoreExtManual;
use gtk::{
    Align, ButtonExt, CellLayoutExt, EditableExt, GtkWindowExt, Inhibit, Justification, LabelExt,
    OrientableExt, Orientation::*, SpinButtonExt, TreeViewExt, WidgetExt, TreeSelectionExt, TreeModelExt
};
use relm::Widget;
use relm_derive::widget;
use relm_derive::Msg;
use std::net::IpAddr;

#[derive(Clone)]
pub struct O2IInfo {
    local_address: IpAddr,
    public_address: IpAddr,
    ports: Vec<u16>,
}

pub fn open_main_window(local_address: IpAddr, public_address: IpAddr, ports: Vec<u16>) {
    let data = O2IInfo {
        local_address,
        public_address,
        ports,
    };
    Win::run(Some(data)).unwrap();
}

// pub fn open_main_window_2() {
//     Win::run(None).unwrap();
// }

#[derive(Msg)]
pub enum Msg {
    ApplyPort,
    // ChangeExternalPort,
    // ChangePortStatus,
    // RefreshMinecraftClientServers,
    Refresh,
    Quit,
}

// #[derive(Clone)]
// pub struct Model {
//     ports: Vec<u16>,
// }

#[widget]
impl Widget for Win {
    fn model(data: Option<O2IInfo>) -> O2IInfo {
        if data.is_none() {
            O2IInfo {
                public_address: get_public_address().unwrap(),
                local_address: IpAddr::V4(get_local_ip().unwrap()),
                ports: vec![25565],
            }
        } else {
            data.unwrap()
        }
    }

fn update(&mut self, event: Msg) {
    match event {
        Msg::ApplyPort => {
            // println!("Selected port is {}", self.port_selection.get_value());
            let mut selected_port = "".to_string();
            if let Some((list_model, iter)) = self.tree_view.get_selection().get_selected() {
                selected_port = list_model
                    .get_value(&iter, 0)
                    .get::<String>()
                    .ok()
                    .and_then(|value| value)
                    .expect("get_value.get<String> failed");
            }
            let clicked_port = selected_port.split("hosted at ").last().unwrap().parse::<u16>().unwrap();
            let lease = self.lease_time.get_value() as u32;
            redirect_minecraft_to_a_port(clicked_port, self.port_selection.get_value() as u16, lease);
        }
        Msg::Quit => gtk::main_quit(),
        Msg::Refresh => {
            self.model.ports = scan_ports();
            self.model.public_address = get_public_address().unwrap();
            self.model.local_address = IpAddr::V4(get_local_ip().unwrap());
            self.update_port_list();
        }
        // _ => {}
    }
}

    fn init_view(&mut self) {
        // Set up the window size
        // self.window.resize(480, 360);

        // Set ip addresses
        self.public_ip
            .set_text(&format!("Public IP: {}", self.model.public_address));
        self.local_ip
            .set_text(&format!("Local IP: {}", self.model.local_address));

        // Set up the port selection spin button
        self.port_selection.set_range(10000.0, 65535.0);
        self.port_selection.set_increments(1.0, 100.0);
        self.port_selection.set_value(25565.0);
        
        // Set up the lease time selection
        self.lease_time.set_range(0.0, 31_556_926.0); // From 0(indefinite) to about a year
        self.lease_time.set_increments(1.0, 100.0);
        self.lease_time.set_value(3600.0);  // Let it default to an hour

        // Set up minecraft port display
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        self.tree_view.append_column(&column);
        self.update_port_list();
    }

    fn update_port_list(&mut self) {
        let store_model = self
            .create_and_fill_model(self.model.clone())
            .expect("create_and_fill_model failed");
        self.tree_view.set_model(Some(&store_model));
    }

    fn create_and_fill_model(&self, data: O2IInfo) -> std::io::Result<gtk::ListStore> {
        // Single row model
        let model = gtk::ListStore::new(&[String::static_type()]);

        // Add the parent directory
        // model.insert_with_values(None, &[0 as u32], &[&"Minecraft, hosted at {}"]);
        let mut counter = 0;
        for p in data.ports {
            model.insert_with_values(Some(counter), &[0 as u32], &[&format!("Minecraft, hosted at {}", p)]);
            counter+=1;
        }
        Ok(model)
    }

    view! {
        #[name="window"]
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                hexpand: true,
                vexpand: true,
                gtk::Box{
                    orientation: Horizontal,
                    #[name="port_selection_label"]
                    gtk::Label{
                        justify: Justification::Left,
                        halign: Align::Start,
                        property_width_request: 100,
                        text: "External port: "
                    },
                    #[name="port_selection"]
                    gtk::SpinButton{
                        digits: 0,
                        property_width_request: 100,
                        editable: true,
                    },
                    #[name="port_selection_desc"]
                    gtk::Label{
                        text:" Port, on which the server will be visible"
                    }
                },
                gtk::Box{
                    orientation: Horizontal,
                    #[name="lease_time_label"]
                    gtk::Label{
                        justify: Justification::Left,
                        halign: Align::Start,
                        property_width_request: 100,
                        text: "Lease time: "
                    },
                    #[name="lease_time"]
                    gtk::SpinButton{
                        digits: 0,
                        property_width_request: 100,
                        editable: true,
                    },
                    #[name="lease_time_desc"]
                    gtk::Label{
                        text:" Lease time in seconds, 0 for indefinite"
                    }
                },
                #[name="public_ip"]
                gtk::Label{
                    justify: Justification::Left,
                    halign: Align::Start,
                },
                #[name="local_ip"]
                gtk::Label{
                    justify: Justification::Left,
                    halign: Align::Start,
                },
                #[name="refresh_button"]
                gtk::Button{
                    label: "Refresh",
                    clicked => Msg::Refresh,
                    halign: Align::Start,
                },
                #[name="tree_view"]
                gtk::TreeView{
                    vexpand: true,
                },
                gtk::Button{
                    clicked => Msg::ApplyPort,
                    label: "Apply"
                },
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
