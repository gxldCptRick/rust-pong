mod pong;
use amethyst::core::TransformBundle;
use amethyst::utils::application_root_dir;
use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle, types::DefaultBackend};
use amethyst::GameDataBuilder;
use amethyst::Application;

use crate::pong::Pong;

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");
    let bundle = RenderingBundle::<DefaultBackend>::new()
    // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
    .with_plugin(
        RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.0, 0.0, 0.0, 1.0]),
    )
    // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
    .with_plugin(RenderFlat2D::default());
    let game_data = GameDataBuilder::default()
        .with_bundle(
            bundle,
        )?;
    let game_data = game_data.with_bundle(TransformBundle::new())?;
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}

