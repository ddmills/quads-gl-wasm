use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "WASM TEST".to_string(),
        window_width: 800,
        window_height: 600,
        // high_dpi: todo!(),
        fullscreen: false,
        // sample_count: todo!(),
        window_resizable: true,
        // icon: todo!(),
        // platform: todo!(),
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, world!");

    // [warning] "Missed function: glCheckFramebufferStatus"
    // [panic] .cargo/registry/src/index.crates.io-1949cf8c6b5b557f/miniquad-0.4.8/src/graphics/gl.rs, line: 1144, column: 21
    render_target(100, 100);

    loop {
        draw_fps();
        next_frame().await;
    }
}
