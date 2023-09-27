use std::{
    collections::HashMap,
    sync::{mpsc, Arc, RwLock},
};

use rand::Rng;

use crate::{
    model::{
        Camera, Car, CarCategory, Day, Driver, DriverId, Entry, EntryGameData, EntryId, Event, Lap,
        Model, Nationality, Session, SessionGameData, SessionId, SessionPhase, SessionType, Value,
    },
    time::Time,
    AdapterError, Distance, GameAdapter, Temperature, UpdateEvent,
};

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

#[derive(Default)]
pub struct DummyAdapter {}

impl GameAdapter for DummyAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        _command_rx: mpsc::Receiver<crate::AdapterCommand>,
        _update_event: UpdateEvent,
    ) -> Result<(), AdapterError> {
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
                    car: Value::new(Car::new_static(
                        "Car model",
                        "Manufacturer",
                        CarCategory { name: "Car Cat" },
                    )),
                    car_number: Value::new(rand.gen::<i32>() % 1000),
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
                    in_pits: Value::new(true),
                    gear: Value::new(4),
                    speed: Value::new(128.0),
                    connected: Value::new(true),
                    stint_time: Value::new(Time::from(56_789)),
                    distance_driven: Value::new(i as f32 * 0.345),
                    focused: i % 3 == 0,
                    game_data: EntryGameData::None,
                    is_finished: Value::new(false),
                },
            );
        }

        Ok(())
    }
}

fn random_driver(id: DriverId) -> Driver {
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
