 use image;
use tauri_utils::config::{Config, WindowConfig,BundleConfig};
use wry::application::window::Icon;
use std::path::Path;
#[cfg(target_os = "macos")]
use wry::application::platform::macos::EventLoopWindowTargetExtMacOS;
#[cfg(target_os = "macos")]
use wry::application::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "linux")]
use wry::application::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
#[cfg(windows)]
use wry::application::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
fn main() -> wry::Result<()> {
    use wry::{
        application::{
            accelerator::{Accelerator, SysMods},
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            keyboard::KeyCode,
            menu::{MenuBar as Menu, MenuItem, MenuItemAttributes, MenuType},
            // platform::macos::WindowBuilderExtMacOS,
            window::{Fullscreen, Window, WindowBuilder},
        },
        webview::WebViewBuilder,
    };



    let mut menu_bar_menu = Menu::new();
    let mut first_menu = Menu::new();

    first_menu.add_native_item(MenuItem::Hide);
    first_menu.add_native_item(MenuItem::EnterFullScreen);
    first_menu.add_native_item(MenuItem::Minimize);
    first_menu.add_native_item(MenuItem::Separator);
    first_menu.add_native_item(MenuItem::Copy);
    first_menu.add_native_item(MenuItem::Cut);
    first_menu.add_native_item(MenuItem::Paste);
    first_menu.add_native_item(MenuItem::Undo);
    first_menu.add_native_item(MenuItem::Redo);
    first_menu.add_native_item(MenuItem::SelectAll);
    first_menu.add_native_item(MenuItem::Separator);
    let close_item = first_menu.add_item(
        MenuItemAttributes::new("CloseWindow")
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyW)),
    );
    first_menu.add_native_item(MenuItem::Quit);

    menu_bar_menu.add_submenu("App", true, first_menu);

    let (WindowConfig {
        url,
        width,
        height,
        resizable,
        transparent,
        fullscreen,
        //标题
        title,
       //是否隐藏窗口标题
        decorations,
        ..
    },BundleConfig{

        icon,
        ..

    }) = get_windows_config().unwrap_or((WindowConfig::default(),BundleConfig::default()));
    let event_loop = EventLoop::new();
    #[cfg(target_os = "macos")]
    let window = WindowBuilder::new()
        .with_resizable(resizable)
        .with_titlebar_transparent(transparent)
        .with_fullscreen(if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        })
        .with_fullsize_content_view(true)
        .with_titlebar_buttons_hidden(false)
        .with_title_hidden(true)
        .with_menu(menu_bar_menu)
        .with_inner_size(wry::application::dpi::LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

      let s =   icon.into_iter().find(|x|x.ends_with(".png")).map(|x|  {
      
   
        load_icon(Path::new(&x ))
      } );
    let window = WindowBuilder::new()
    .with_resizable(resizable)
   
    // .with_titlebar_transparent(transparent)
    .with_fullscreen(if fullscreen {
        Some(Fullscreen::Borderless(None))
    } else {
        None
    })
    .with_decorations(decorations)
    
    // .with_fullsize_content_view(true)
    // .with_titlebar_buttons_hidden(false)
    // .with_title_hidden(true)
    // .with_menu(menu_bar_menu)

    .with_title(title)
    .with_window_icon(s)
    .with_inner_size(wry::application::dpi::LogicalSize::new(width, height))
    .build(&event_loop)
    .unwrap();

    

    let handler = move |window: &Window, req: String| {
        if req == "drag_window" {
            let _ = window.drag_window();
        } else if req == "fullscreen" {
            if window.fullscreen().is_some() {
                window.set_fullscreen(None);
            } else {
                window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            }
        }
    };

    let _webview = WebViewBuilder::new(window)?
        .with_url(&url.to_string())?
       //调试模式
        .with_devtools(true)
        .with_initialization_script(include_str!("pake.js"))
        .with_ipc_handler(handler)
    
        .build()?;

    // _webview.open_devtools();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MenuEvent {
                menu_id,
                origin: MenuType::MenuBar,
                ..
            } => {
                if menu_id == close_item.clone().id() {
                    _webview.window().set_minimized(true);
                }
                println!("Clicked on {:?}", menu_id);
            }
            _ => (),
        }
    });
}

fn get_windows_config() -> Option<(WindowConfig,BundleConfig) > {
    let config_file = include_str!("../tauri.conf.json");
    let config: Config = serde_json::from_str(config_file).expect("failed to parse windows config");

    Some((config.tauri.windows.iter().next().cloned().expect("failed to parse windows config"),config.tauri.bundle))
}


fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
 
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}






