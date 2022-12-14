use autopilot::alert::Response;
use autopilot::alert::Response::Default;

fn main() {
    let response = autopilot::alert::alert(
        "Hello, world!",
        Some("AutoPilot Alert"),
        Some("OK"),
        Some("Cancel"),
    );

    match response {
        autopilot::alert::Response::Default => println!("Accepted"),
        autopilot::alert::Response::Cancel => println!("Canceled")
    }
}
