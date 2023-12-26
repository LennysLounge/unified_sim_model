use std::{
    collections::HashMap,
    ops::ControlFlow,
    sync::{
        mpsc::{self, TryRecvError},
        Arc, RwLock,
    },
    time::Duration,
};

use rand::Rng;
use tracing::error;

use crate::{
    model::{
        Camera, Car, CarCategory, Day, Driver, DriverId, Entry, EntryGameData, EntryId, Event, Lap,
        Model, Nationality, Session, SessionGameData, SessionId, SessionPhase, SessionType, Value,
    },
    time::Time,
    AdapterCommand, AdapterError, Distance, GameAdapter, GameAdapterCommand, Temperature,
    UpdateEvent,
};

/// Commands for the dummy adapter.
pub enum DummyCommands {
    /// Set the focus to a specific entry.
    SetFocus(EntryId),
}

#[derive(Default)]
pub struct DummyAdapter {}

impl GameAdapter for DummyAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        command_rx: mpsc::Receiver<crate::AdapterCommand>,
        update_event: UpdateEvent,
    ) -> Result<(), AdapterError> {
        setup_model(&model);

        loop {
            match command_rx.try_recv() {
                Ok(AdapterCommand::Close) => break,
                Ok(action) => {
                    if self.handle_command(&model, action).is_break() {
                        break;
                    }
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => {
                    // This should only happen if all adapters have been dropped.
                    // In which case it is impossible to interact with this adapter any more.
                    // To avoid leaking memory we quit.
                    error!("All adapter handle have been dropped it is impossible to communicate with this game adapter.");
                    break;
                }
            };

            update_event.trigger();
            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}
impl DummyAdapter {
    fn handle_command(
        &mut self,
        model: &Arc<RwLock<Model>>,
        command: AdapterCommand,
    ) -> ControlFlow<()> {
        let mut model = model.write().expect("Should be able to lock for writing");
        match command {
            AdapterCommand::Close => return ControlFlow::Break(()),
            AdapterCommand::Game(GameAdapterCommand::Dummy(command)) => match command {
                DummyCommands::SetFocus(entry_id) => {
                    model.focused_entry = Some(entry_id);
                    if let Some(session) = model.current_session_mut() {
                        session
                            .entries
                            .values_mut()
                            .for_each(|entry| entry.focused = entry.id == entry_id);
                    }
                }
            },
            _ => (),
        }
        ControlFlow::Continue(())
    }
}

fn setup_model(model: &Arc<RwLock<Model>>) {
    let mut model = model.write().expect("Should be able to lock for writing");

    model.event_name.set("Dummy event".to_string());
    model.active_camera.set(Camera::Hellicopter);
    model.available_cameras.insert(Camera::Hellicopter);
    model.available_cameras.insert(Camera::Chase);
    model.available_cameras.insert(Camera::FirstPerson);
    model.focused_entry = None;

    // model.track_name = "Dummy track".to_string();
    // model.track_length = 1234;

    let id = model.add_session(Session {
        id: SessionId(0),
        entries: HashMap::new(),
        session_type: Value::new(SessionType::Race),
        session_time: Value::new(Time::from(1_200_123)),
        time_remaining: Value::new(Time::from(754_123)),
        laps: Value::new(20),
        laps_remaining: Value::new(12),
        phase: Value::new(SessionPhase::Active),
        time_of_day: Value::new(Time::from(50_846_123)),
        day: Value::new(Day::Sunday),
        ambient_temp: Value::new(Temperature::from_celcius(24.0)),
        track_temp: Value::new(Temperature::from_celcius(26.0)),
        best_lap: Value::new(Some(Lap {
            time: Value::new(Time::from(81_1234)),
            splits: Value::new(vec![
                Time::from(12_345),
                Time::from(67_891),
                Time::from(111_213),
            ]),
            driver_id: Some(DriverId::default()),
            entry_id: Some(EntryId::default()),
            invalid: Value::new(false),
        })),
        track_name: Value::new("Dummy track".to_string()),
        track_length: Value::new(Distance::from_meter(1234.0)),
        game_data: SessionGameData::None,
    });
    model.current_session = Some(id);
    model.events.push(Event::SessionChanged(SessionId(0)));

    let mut rand = rand::thread_rng();
    for i in 0..10 {
        let session = model.current_session_mut().unwrap();
        let entry_id = EntryId(i);
        session.entries.insert(
            entry_id,
            Entry {
                id: entry_id,
                drivers: {
                    let mut drivers = HashMap::new();
                    for j in 0..3 {
                        let driver_id = DriverId(j);
                        drivers.insert(driver_id, random_driver(driver_id));
                    }
                    drivers
                },
                current_driver: DriverId(0),
                team_name: Value::new(format!("Team nr.{}", i)),
                car: Value::new(random_car()),
                car_number: Value::new(rand.gen::<i32>().abs() % 100),
                nationality: Value::new(Nationality::NONE),
                world_pos: Value::new([0.0, 0.0, 0.0]),
                orientation: Value::new([0.0, 0.0, 0.0]),
                position: Value::new(i + 1),
                spline_pos: Value::new(0.1234),
                lap_count: Value::new(0),
                laps: Vec::new(),
                current_lap: Value::new(Lap {
                    time: Value::new(Time::from(12_345)),
                    splits: Value::new(Vec::new()),
                    driver_id: Some(DriverId(0)),
                    entry_id: Some(EntryId(i)),
                    invalid: Value::new(i % 2 == 0),
                }),
                best_lap: Value::new(None),
                performance_delta: Value::new(Time::from(-1_234)),
                time_behind_leader: Value::new(Time::from(12_345)),
                in_pits: Value::new(i % 3 == 0),
                gear: Value::new(4),
                speed: Value::new(128.0),
                connected: Value::new(true),
                stint_time: Value::new(Time::from(56_789)),
                distance_driven: Value::new(i as f32 * 0.345),
                focused: i == 0,
                game_data: EntryGameData::None,
                is_finished: Value::new(false),
            },
        );
    }
}

fn random_driver(id: DriverId) -> Driver {
    const FIRST_NAMES: [&str; 20] = [
        "Liam",
        "Noah",
        "Oliver",
        "James",
        "Elijah",
        "William",
        "Henry",
        "Lucas",
        "Benjamin",
        "Theodore",
        "Mateo",
        "Levi",
        "Sebastian",
        "Daniel",
        "Jack",
        "Michael",
        "Alexander",
        "Owen",
        "Asher",
        "Samuel",
    ];

    const LAST_NAMES: [&str; 20] = [
        "Smith",
        "Johnson",
        "Williams",
        "Brown",
        "Jones",
        "Garcia",
        "Miller",
        "Davis",
        "Rodriguez",
        "Martinez",
        "Hernandez",
        "Lopez",
        "Gonzalez",
        "Wilson",
        "Anderson",
        "Thomas",
        "Taylor",
        "Moore",
        "Jackson",
        "Martin",
    ];
    let mut rand = rand::thread_rng();
    let first_name = FIRST_NAMES[rand.gen::<usize>() % FIRST_NAMES.len()];
    let last_name = LAST_NAMES[rand.gen::<usize>() % LAST_NAMES.len()];

    Driver {
        id,
        first_name: Value::new(first_name.to_string()),
        last_name: Value::new(last_name.to_string()),
        short_name: Value::new(format!("{}{}", &first_name[0..1], &last_name[0..1])),
        nationality: Value::new(Nationality::NONE),
        driving_time: Value::new(Time::from(0)),
        best_lap: Value::new(None),
    }
}
fn random_car() -> Car {
    const GT3: CarCategory = CarCategory::new("GT3");
    const GT4: CarCategory = CarCategory::new("GT4");
    const ST: CarCategory = CarCategory::new("ST");
    const ST22: CarCategory = CarCategory::new("ST");
    const CUP: CarCategory = CarCategory::new("CUP");
    const CUP21: CarCategory = CarCategory::new("CUP");
    const CHL: CarCategory = CarCategory::new("CHL");
    const TCX: CarCategory = CarCategory::new("TCX");
    const CARS: [Car; 46] = [
        Car::new_static("Porsche 991 GT3 R", "Porsche", GT3),
        Car::new_static("Mercedes-AMG GT3", "Mercedes-AMG", GT3),
        Car::new_static("Ferrari 488 GT3", "Ferrari", GT3),
        Car::new_static("Audi R8 LMS", "Audi", GT3),
        Car::new_static("Lamborghini Huracan GT3", "Lamborghini", GT3),
        Car::new_static("McLaren 650S GT3", "McLaren", GT3),
        Car::new_static("Nissan GT-R Nismo GT3 2018", "Nissan", GT3),
        Car::new_static("BMW M6 GT3", "BMW", GT3),
        Car::new_static("Bentley Continental GT3 2018", "Bentley", GT3),
        Car::new_static("Porsche 991 II GT3 Cup", "Porsche", CUP),
        Car::new_static("Nissan GT-R Nismo GT3", "Nissan", GT3),
        Car::new_static("Bentley Continental GT3", "Bentley", GT3),
        Car::new_static("AMR V12 Vantage GT3", "Aston-Martin", GT3),
        Car::new_static("Reiter Engineering R-EX GT3", "Reiter-Engineering", GT3),
        Car::new_static("Emil Frey Jaguar G3", "Jaguar", GT3),
        Car::new_static("Lexus RC F GT3", "Lexus", GT3),
        Car::new_static("Lamborghini Huracan GT3 Evo", "Lamborghini", GT3),
        Car::new_static("Honda NSX GT3", "Honda", GT3),
        Car::new_static("Lamborghini Huracan ST", "Lamborghini", ST),
        Car::new_static("Audi R8 LMS Evo", "Audi", GT3),
        Car::new_static("AMR V8 Vantage", "Aston-Martin", GT3),
        Car::new_static("Honda NSX GT3 Evo", "Honda", GT3),
        Car::new_static("McLaren 720S GT3", "McLaren", GT3),
        Car::new_static("Porsche 911 II GT3 R", "Porsche", GT3),
        Car::new_static("Ferrari 488 GT3 Evo", "Ferrari", GT3),
        Car::new_static("Mercedes-AMG GT3 2020", "Mercedes-AMG", GT3),
        Car::new_static("Ferrari 488 Challenge Evo", "Ferrari", CHL),
        Car::new_static("BMW M2 CS Racing", "BMW", TCX),
        Car::new_static("Porsche 992 GT3 CUP", "Porsche", CUP21),
        Car::new_static("Lamborghini Huracan ST EVO2", "Lamborghini", ST22),
        Car::new_static("BMW M4 GT3", "BMW", GT3),
        Car::new_static("Audi R8 LMS Evo 2", "Audi", GT3),
        Car::new_static("FERRARI 296 GT3", "Ferrari", GT3),
        Car::new_static("Lamborghini Huracan EVO2", "Lamborghini", GT3),
        Car::new_static("Porsche 992 GT3 R", "Porsche", GT3),
        Car::new_static("Alpine A110 GT4", "Alpine", GT4),
        Car::new_static("Aston Martin Vantage GT4", "Aston-Martin", GT4),
        Car::new_static("Audi R8 LMS GT4", "Audi", GT4),
        Car::new_static("BMW M4 GT4", "BMW", GT4),
        Car::new_static("Chevrolet Camaro GT4", "Chevrolet", GT4),
        Car::new_static("Ginetta G55 GT4", "Ginetta", GT4),
        Car::new_static("KTM X-Bow GT4", "KTM", GT4),
        Car::new_static("Maserati MC GT4", "Maserati", GT4),
        Car::new_static("McLaren 570S GT4", "McLaren", GT4),
        Car::new_static("Mercedes AMG GT4", "Mercedes-AMG", GT4),
        Car::new_static("Porsche 718 Cayman GT4 Clubsport", "Porsche", GT4),
    ];
    let mut rand = rand::thread_rng();
    CARS[rand.gen::<usize>() % CARS.len()].clone()
}
