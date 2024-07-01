use signal_lights::LightColor;

mod signal_lights;

fn main() {
    signal_lights::next_signal(LightColor::RED);
}
