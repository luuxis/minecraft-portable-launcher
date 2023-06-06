use std::{sync::mpsc, ops::Range};

mod impl_window 
{
    use nwg::NativeUi;
    use std::{ops::{Range, Deref}, rc::Rc, cell::RefCell};

    use nwg::stretch::{
        geometry::{Size, Rect},
        style::{Dimension as D, FlexDirection}
    };

    #[derive(Default)]
    pub struct GuiApplication {
        pub window: nwg::Window,
        
        app_layout: nwg::FlexboxLayout, 
        loading_text: nwg::Label,
        pub progress_bar: nwg::ProgressBar,
        
        bottom_layout: nwg::FlexboxLayout, 
        pub status_text: nwg::Label,
        cancel_button: nwg::Button,

        icon: Option<nwg::Icon>
    }

    pub struct GuiApplicationUi {
        inner: Rc<GuiApplication>,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }
    impl nwg::NativeUi<GuiApplicationUi> for GuiApplication {
        fn build_ui(mut data: GuiApplication) -> Result<GuiApplicationUi, nwg::NwgError> {
            use nwg::Event;

            let (win_width, win_height) = (380, 125);
            
            let (screen_width, screen_height) = (nwg::Monitor::width(), nwg::Monitor::height());
            let (win_pos_x, win_pos_y) = ((screen_width-win_width)/2, (screen_height-win_height)/2);
            nwg::Window::builder()
                .title("Minecraft")
                .size((win_width, win_height))
                .flags(nwg::WindowFlags::MAIN_WINDOW & !nwg::WindowFlags::RESIZABLE)
                .position((win_pos_x, win_pos_y))
                .build(&mut data.window)?;

            nwg::Label::builder()
                .parent(&data.window)
                .text("Chargement en cours...")
                .build(&mut data.loading_text)?;

            nwg::ProgressBar::builder()
                .parent(&data.window)
                .build(&mut data.progress_bar)?;

            nwg::Label::builder()
                .parent(&data.window)
                .flags(nwg::LabelFlags::ELIPSIS | nwg::LabelFlags::VISIBLE)
                .build(&mut data.status_text)?;

            nwg::Button::builder()
                .parent(&data.window)
                .text("Arrêter")
                .build(&mut data.cancel_button)?;

            let ui = GuiApplicationUi {
                inner:  Rc::new(data),
                default_handler: Default::default(),
            };

            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        Event::OnWindowClose if &handle == &evt_ui.window => GuiApplication::on_stop(&evt_ui),
                        Event::OnButtonClick if &handle == &evt_ui.cancel_button => GuiApplication::on_stop(&evt_ui),
                        Event::OnInit if &handle == &evt_ui.window => GuiApplication::on_init(&evt_ui),
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .child(&ui.status_text)
                    .child_flex_grow(1.)
                    .child_size(Size{width: D::Auto, height: D::Auto})
                .child(&ui.cancel_button)
                    .child_size(Size{width: D::Points(100.), height: D::Auto})
                    .child_margin(Rect { start: D::Points(0.), end: D::Points(0.), top: D::Points(0.), bottom: D::Points(0.) })
                .build_partial(&ui.bottom_layout)?;

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Column)
                .child(&ui.loading_text)
                    .child_size(Size{width: D::Auto, height: D::Points(25.)})
                .child(&ui.progress_bar)
                    .child_flex_grow(1.)
                    .child_size(Size{width: D::Auto, height: D::Auto})
                .child_layout(&ui.bottom_layout)
                    .child_size(Size{width: D::Auto, height: D::Points(30.)})
                .build(&ui.app_layout)?;
            Ok(ui)
        }
    }

    impl Deref for GuiApplicationUi {
        type Target = GuiApplication;

        fn deref(&self) -> &GuiApplication {
            &self.inner
        }
    }
    
    impl GuiApplication {
        fn initial_state() -> Self {
            Self {
                icon: Self::load_icon(),
                ..Self::default()
            }
        }
        pub fn new(range: Range<u32>) -> Result<GuiApplicationUi, nwg::NwgError> {
            let app = Self::initial_state();

            let app: GuiApplicationUi = GuiApplication::build_ui(Self::initial_state()).expect("Failed to build UI");
            app.window.set_icon(app.icon.as_ref());
            app.progress_bar.set_range(range);
            Ok(app)
        }
        fn load_icon() -> Option::<nwg::Icon> {
            nwg::EmbedResource::load(None)
                .and_then(|res| nwg::Icon::from_embed(&res, Some(1), None))
                .ok()
        }
        fn on_init(&self) {
            self.window.set_visible(true);
        }
        fn on_stop(&self) {
            nwg::stop_thread_dispatch();
        }
        pub fn close(&self) {
            self.window.close();
        }
    }
}


pub enum Signal {
    Quit,
    AdvanceDelta(u32),
    Advance,
    UpdateStatus(String),
}
#[must_use]
pub fn make_window(events: mpsc::Receiver<Signal>, range: Range<u32>) -> std::thread::JoinHandle<()>{
    
    std::thread::spawn(move || {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        let app = impl_window::GuiApplication::new(range).expect("Failed to build UI");
        
        nwg::dispatch_thread_events_with_callback(move || {
            let mut status_text = None;
            while let Ok(signal) = events.try_recv() {
                match signal {
                    Signal::Quit => app.close(),
                    Signal::AdvanceDelta(delta) => app.progress_bar.advance_delta(delta),
                    Signal::Advance => app.progress_bar.advance(),
                    Signal::UpdateStatus(status) => status_text = Some(status),
                }
            }
            if let Some(status_text) = status_text {
                app.status_text.set_text(&status_text);
            }
        });
    })
}