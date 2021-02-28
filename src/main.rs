use rust_gpiozero::Servo;

fn main() {
    let mut leg_servo_bottom = Servo::new(8);
    let mut leg_servo_top = Servo::new(10);

    leg_servo_bottom.mid();
}
