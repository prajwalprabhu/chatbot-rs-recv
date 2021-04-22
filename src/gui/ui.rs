use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use super::backend_gui::Message;
use std::sync::{ Arc,Mutex, mpsc};
// mod gui;
fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);

        let name = String::from("Test");
        let listener_res = std::net::TcpStream::connect("127.0.0.1:8080");
        // for i in listener{
        if listener_res.is_err() {
            let d = gtk::MessageDialog::new(
                // gtk::NONE_WINDOW,
                Some(&window),
                gtk::DialogFlags::DESTROY_WITH_PARENT,
                gtk::MessageType::Error,
                gtk::ButtonsType::Close,
                &listener_res.err().unwrap().to_string(),
            );
            d.show_all();
            // std::process::exit(1);
            return;
        }
        let stream = listener_res.unwrap();
        window.set_title("First GTK Program");
        window.set_default_size(350, 70);

        let box_main: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let box_h: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        let entry: gtk::Entry = gtk::Entry::with_buffer(&gtk::EntryBuffer::new(None));
        let send_button: gtk::Button = gtk::Button::with_label("Send");

        let sc: gtk::ScrolledWindow =
            gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        sc.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        let ls = gtk::ListBox::new();
        box_h.add(&entry);
        box_h.add(&send_button);
        sc.add(&ls);
        box_main.add(&sc);
        box_main.add(&box_h);
        window.add(&box_main);
        let stream_clone = stream.try_clone().expect("stream cant be cloned");
        let name_clone = format!("{:?}", name);
        let (thread_handle,rx) = super::backend_gui::handle_client(&stream);
        send_button.connect_clicked(move |_| {
            super::backend_gui::send(
                &stream_clone,
                Message {
                    member: name_clone.clone(),
                    chat: entry.get_text().to_string(),
                },
            )
        });
        // let new_ls = Mutex::new(Arc::new(ls);
        std::thread::spawn(move ||{

    });
        // gui::backend_gui::run(ls).expect("backend_gui : ");
        window.show_all();
        thread_handle.join().expect("Failed to join :");
    });

    application.run(&[]);
}

fn test(rx:mpsc::Receiver<Message>,ls:gtk::ListBox){
    loop{
        // let rx = new_rx.lock().expect("Failed to lock");
        let message = rx.recv().expect("Failed recv ");
        // let ls = new_ls.lock().expect("Failed to lock ");
        ls.add(&gtk::Label::new(Some(&format!("{:?} : {:?}",message.member,message.chat))));

    }
}
// }
