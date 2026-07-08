mod data;

fn main() -> anyhow::Result<()> {
    player_tui::ui::run(data::config())
}
