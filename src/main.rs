#![windows_subsystem = "windows"]
mod utils;

// TODO: Close ports once the program closes. https://docs.rs/igd/0.12.0/igd/struct.Gateway.html#method.remove_port

use glib::Sender;
use gtk::{prelude::*, Adjustment, StringObject};
use relm4::Widget;
use relm4::{AppUpdate, RelmApp};
use utils::scan_ports;
use utils::*;

struct AppWidgets {
    main: gtk::ApplicationWindow,
    ports: gtk::DropDown,
}

enum AppMsg {
    RefreshPorts,
    OpenPort,
    UpdatedLease(i32),
    UpdatedPort(i32),
    SelectedPort(u32),
}

struct AppModel {
    port: u16,
    open_ports: Vec<u16>,
    lease_time: u32,
    selected_port: Option<u16>,
    public_ip: String,
    local_ip: String,
}

impl Widget<AppMsg, AppModel> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(sender: Sender<AppMsg>, model: &AppModel) -> Self {
        let main = gtk::ApplicationWindowBuilder::new().build();
        let vbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_end(5)
            .margin_top(5)
            .hexpand(false)
            .build();
        let grid = gtk::Grid::new();
        let public = gtk::Label::new(Some("Public IP"));
        let public_entry = gtk::Entry::new();
        public_entry.set_editable(false);
        public_entry.set_text(&model.public_ip);
        grid.attach(&public, 0, 0, 1, 1);
        grid.attach(&public_entry, 1, 0, 1, 1);

        let local = gtk::Label::new(Some("Private IP"));
        let local_entry = gtk::Entry::new();
        local_entry.set_editable(false);
        local_entry.set_text(&model.local_ip);
        grid.attach(&local, 0, 1, 1, 1);
        grid.attach(&local_entry, 1, 1, 1, 1);

        let lease = gtk::Label::new(Some("Lease time"));
        let adj = Adjustment::new(3600.0, 0.0, 10000000.0, 1.0, 10.0, 10.0);
        let lease_entry = gtk::SpinButton::new(Some(&adj), 1.0, 0);
        let sender2 = sender.clone();
        lease_entry.connect_changed(move |a| {
            sender2
                .send(AppMsg::UpdatedLease(a.value_as_int()))
                .unwrap();
        });
        grid.attach(&lease, 0, 2, 1, 1);
        grid.attach(&lease_entry, 1, 2, 1, 1);

        let external_port = gtk::Label::new(Some("External port"));
        let adj = Adjustment::new(25565.0, 0.0, 10000000.0, 1.0, 10.0, 10.0);
        let external_port_entry = gtk::SpinButton::new(Some(&adj), 1.0, 0);
        let sender2 = sender.clone();
        external_port_entry.connect_changed(move |a| {
            sender2.send(AppMsg::UpdatedPort(a.value_as_int())).unwrap();
        });
        grid.attach(&external_port, 0, 3, 1, 1);
        grid.attach(&external_port_entry, 1, 3, 1, 1);

        let internal_port = gtk::Label::new(Some("Internal port"));
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_factory, item| {
            let row = gtk::Label::new(Some(&format!("{}", item)));
            item.set_child(Some(&row));
        });
        factory.connect_bind(move |_factory, list_item| {
            let item_label = list_item
                .item()
                .unwrap()
                .downcast::<StringObject>()
                .unwrap();
            let child = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            child.set_label(&item_label.string().as_str());
        });

        let internal_port_entry = gtk::DropDownBuilder::new().list_factory(&factory).build();
        let mut store = vec![];
        model.open_ports.iter().for_each(|a| {
            store.push(format!("{}", a));
        });
        let mut vec = vec![];
        for s in store.iter() {
            vec.push(s.as_str());
        }
        let store = gtk::StringList::new(&vec[..]);
        internal_port_entry.set_model(Some(&store));
        let sender2 = sender.clone();
        internal_port_entry.connect_selected_item_notify(move |a| {
            sender2.send(AppMsg::SelectedPort(a.selected())).unwrap();
        });
        internal_port_entry.model();
        grid.attach(&internal_port, 0, 4, 1, 1);
        grid.attach(&internal_port_entry, 1, 4, 1, 1);

        vbox.append(&grid);
        let buttons = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .margin_end(5)
            .margin_top(5)
            .build();
        let refresh = gtk::ButtonBuilder::new().label("Refresh ports").build();

        let sender2 = sender.clone();
        refresh.connect_clicked(move |_button| {
            sender2.send(AppMsg::RefreshPorts).unwrap();
        });
        let open = gtk::ButtonBuilder::new().label("Open port").build();

        let sender2 = sender.clone();
        open.connect_clicked(move |_button| {
            sender2.send(AppMsg::OpenPort).unwrap();
        });

        buttons.append(&refresh);
        buttons.append(&open);
        vbox.append(&buttons);

        main.set_child(Some(&vbox));

        AppWidgets {
            main,
            ports: internal_port_entry,
        }
    }

    fn root_widget(&self) -> gtk::ApplicationWindow {
        self.main.clone()
    }
}

impl AppUpdate<AppMsg> for AppModel {
    type Widgets = AppWidgets;

    fn init_model() -> Self {
        let public_ip = futures::executor::block_on(get_public_address());
        AppModel {
            port: 25565,
            open_ports: scan_ports(),
            lease_time: 3600,
            selected_port: None,
            public_ip: if let Some(ip) = public_ip {
                ip.to_string()
            } else {
                "No public IP O_o".to_owned()
            },
            local_ip: if let Some(ip) = get_local_ip() {
                ip.to_string()
            } else {
                "No local IP O_o".to_owned()
            },
        }
    }

    fn update(&mut self, msg: AppMsg, widgets: &Self::Widgets) {
        match msg {
            AppMsg::OpenPort => {
                redirect_minecraft_to_a_port(
                    self.selected_port.unwrap_or(0),
                    self.port,
                    self.lease_time,
                );
            }
            AppMsg::RefreshPorts => {
                self.open_ports = dbg!(scan_ports());
                let mut store = vec![];
                self.open_ports.iter().for_each(|a| {
                    store.push(format!("{}", a));
                });
                let mut vec = vec![];
                for s in store.iter() {
                    vec.push(s.as_str());
                }
                let store = gtk::StringList::new(&vec[..]);
                widgets.ports.set_model(Some(&store));
            }
            AppMsg::UpdatedLease(a) => self.lease_time = a as u32,
            AppMsg::UpdatedPort(a) => self.port = a as u16,
            AppMsg::SelectedPort(a) => {
                if let Some(a) = self.open_ports.get(a as usize) {
                    self.selected_port = Some(*a);
                } else {
                    self.selected_port = None;
                }
            }
        }
    }

    fn view(&self, _widgets: &mut Self::Widgets) {}
}

fn main() {
    gtk::init().unwrap();
    let relm: RelmApp<AppWidgets, AppModel, AppMsg> = RelmApp::create();
    relm.run();
}
