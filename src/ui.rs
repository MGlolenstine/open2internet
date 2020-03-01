#[allow(dead_code)]

use glib::types::StaticType;
use gtk::prelude::GtkListStoreExtManual;
use gtk::{
    ButtonExt, CellLayoutExt, EditableExt, GtkWindowExt, Inhibit, LabelExt,
    OrientableExt, Orientation::*, SpinButtonExt, TreeViewExt, WidgetExt,
};
use relm::Widget;
use relm_derive::widget;
use relm_derive::Msg;
use std::net::{IpAddr, Ipv4Addr};
use crate::utils::*;

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
    Win::run(()).unwrap();
}

pub fn open_main_window_2() {
    Win::run(()).unwrap();
}

#[derive(Msg)]
pub enum Msg {
    ApplyPort,
    ChangeExternalPort,
    ChangePortStatus,
    RefreshMinecraftClientServers,
    Quit,
}

// #[derive(Clone)]
// pub struct Model {
//     ports: Vec<u16>,
// }

#[widget]
impl Widget for Win {
    fn model() -> O2IInfo {
        O2IInfo { 
            public_address: get_public_address().unwrap(),
            // public_address: IpAddr::V4(Ipv4Addr::new(84, 41, 35, 2)),
            local_address: IpAddr::V4(get_local_ip().unwrap()),
            // local_address: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 104)),
            // ports: scan_ports(),
            ports: vec!(25565),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            // A call to self.label1.set_text() is automatically inserted by the
            // attribute every time the model.counter attribute is updated.
            Msg::ApplyPort => {
                println!("Selected port is {}", self.port_selection.get_value());
            }
            Msg::Quit => gtk::main_quit(),
            _ => {}
        }
    }

    fn init_view(&mut self) {
        // Set up the window size
        self.window.resize(1280, 720);

        // Set up the port selection spin button
        self.port_selection.set_range(10000.0, 65535.0);
        self.port_selection.set_increments(1.0, 100.0);
        self.port_selection.set_value(25565.0);
        // Set up minecraft port display
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        self.tree_view.append_column(&column);
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

        for p in data.ports {
            model.insert_with_values(
                None,
                &[0 as u32],
                &[&format!("Minecraft, hosted at {}", p)],
            );
        }
        Ok(model)
    }

    view! {
        #[name="window"]
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Box{
                    orientation: Horizontal,
                    gtk::Label{
                        text: "External port: "
                    },
                    #[name="port_selection"]
                    gtk::SpinButton{
                        digits: 0,
                        editable: true,
                    },
                    gtk::Button{
                        clicked => Msg::ApplyPort,
                        label: "Apply"
                    },
                },
                gtk::Label{
                    text: "Public IP: {}"
                },
                gtk::Label{
                    text: "Local IP: {}"
                },
                #[name="tree_view"]
                gtk::TreeView{
                }
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
