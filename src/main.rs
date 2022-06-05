use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, Leaflet, ActionRow};
use gtk::{Box, Orientation, ListBox, SelectionMode, Image};
use sysinfo::{System, SystemExt};
use which::which;
use os_release::OsRelease;

const APP_ID: &str = "com.yavko.gnomefetch";
const APP_NAME: &str = "GnomeFetch";

fn main() {
    println!("Hello, world!");
    
    let app = Application::builder()
        .application_id(APP_ID) 
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn generate_package_string() -> std::string::String {
    let mut full_string = String::new();
    
    if which("apt").is_ok()  {
       full_string.push_str(format!("{} (apt) ", nixinfo::packages("apt").unwrap().as_str()).as_str());
    }
    if which("dnf").is_ok()  {
        full_string.push_str(format!("{} (dnf) ", nixinfo::packages("dnf").unwrap().as_str()).as_str());
    }
    if which("pacman").is_ok()  {
        full_string.push_str(format!("{} (pacman) ", nixinfo::packages("pacman").unwrap().as_str()).as_str());
    }
    if which("apk").is_ok()  {
        full_string.push_str(format!("{} (apk) ", nixinfo::packages("apk").unwrap().as_str()).as_str());
    }
    return full_string;
}

fn generate_info_list(list: &ListBox) -> &ListBox {
    let mut sys = System::new_all();
    sys.refresh_all(); 
    
    let os = ActionRow::builder()
        .subtitle("Operating System")
        .title(sys.name().unwrap().as_str())
        .build();
    list.append(&os);
   
    let host = ActionRow::builder()
        .subtitle("Hostname")
        .title(sys.host_name().unwrap().as_str())
        .build();
    list.append(&host);

    let kernel = ActionRow::builder()
        .subtitle("Kernel")
        .title(sys.kernel_version().unwrap().as_str())
        .build();
    list.append(&kernel);

    let uptime = ActionRow::builder()
        .subtitle("Uptime")
        .title(sys.uptime().to_string().as_str())
        .build();
    list.append(&uptime);
    
    // packages should be here
    let packages = ActionRow::builder()
        .subtitle("Packages")
        .title(&generate_package_string().as_str())
        .build();
    list.append(&packages);

    let shell = ActionRow::builder()
        .subtitle("Shell")
        .title(std::env::var("SHELL").unwrap().as_str())
        .build();
    list.append(&shell);

    let de = ActionRow::builder()
        .subtitle("Desktop Environment")
        .title(std::env::var("XDG_CURRENT_DESKTOP").unwrap().as_str())
        .build();
    list.append(&de);
    
    let cpu = ActionRow::builder()
        .subtitle("CPU")
        .title(nixinfo::cpu().unwrap().as_str())
        .build();
    list.append(&cpu);

    let gpu = ActionRow::builder()
        .subtitle("GPU")
        .title(nixinfo::gpu().unwrap().as_str())
        .build();
    list.append(&gpu);

    let mem = ActionRow::builder()
        .subtitle("Memory")
        .title(nixinfo::memory().unwrap().as_str())
        .build();
    list.append(&mem);

    return list;
}

fn build_ui(app: &Application) {
    let leaf = Leaflet::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    
    let release = OsRelease::new(); 


    let icon = Image::from_icon_name(&release.ok().unwrap().extra.get("LOGO").unwrap()); 
    
    leaf.append(&icon);
    
    let list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(vec!["boxed-list".to_string()])
        .build();
    
    generate_info_list(&list);
    leaf.append(&list); 

    let content = Box::new(Orientation::Vertical, 0);

    let header = HeaderBar::builder()
        .title_widget(&WindowTitle::new(&APP_NAME, ""))
        .build();

    content.append(&header);
    content.append(&leaf);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&APP_NAME)
        .default_height(200)
        .default_width(200)
        .content(&content)
        .build();

    window.show();
}
