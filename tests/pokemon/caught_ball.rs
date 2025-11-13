use gammaeditor::pkmn::ball::PokeBall;
use gammaeditor::save::pokemon::caught_ball::{CaughtBall, CaughtBallMut};
use gammaeditor::save::pokemon::pokemon_classes::class_at;
use crate::pokemon::common::{get_gvas, get_gvas_mut};

#[test]
fn test_caught_ball_at_valid() {
    let gvas = get_gvas();
    let wrapper = CaughtBall::new_party(&gvas).expect("get wrapper");

    let actual: PokeBall = wrapper.get_caught_ball_at(1).expect("get ball");
    let expected = PokeBall::UltraBall;
    assert_eq!(actual, expected)
}

#[test]
fn test_set_ball_at_works() {
    let mut gvas = get_gvas_mut();
    let mut wrapper = CaughtBallMut::new_party(&mut gvas).expect("get wrapper");
    wrapper.set_ball_at(PokeBall::PokeBall, 1).expect("get ball");

    let wrapper_read = CaughtBall::new_party(&gvas).expect("get wrapper");
    let actual: PokeBall = wrapper_read.get_caught_ball_at(1).expect("get ball");
    let expected = PokeBall::PokeBall;
    assert_eq!(actual, expected)
}

