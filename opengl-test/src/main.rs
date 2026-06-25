use beryllium::{Sdl, init::InitFlags, video};



fn main(){

    let sdl = Sdl::init(InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

      let win_args = video::CreateWinArgs {
        title: "WINDOW_TITLE",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
  };

  let _win = sdl
    .create_gl_window(win_args)
    .expect("couldn't make a window and context");

  println!("Hello world!");

  let x = 5;
  let y: String = String:From("Hello world");


    
}