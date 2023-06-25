use std::{
    collections::HashMap,
    sync::{mpsc, Arc, RwLock},
};

use crate::{
    model::{
        Camera, Car, CarCategory, Day, Driver, DriverId, Entry, EntryGameData, EntryId, Event, Lap,
        Model, Nationality, Session, SessionGameData, SessionId, SessionPhase, SessionType, Value,
    },
    time::Time,
    AdapterError, GameAdapter, UpdateEvent,
};

pub struct DummyAdapter {}

impl GameAdapter for DummyAdapter {
    fn run(
        &mut self,
        model: Arc<RwLock<Model>>,
        _command_rx: mpsc::Receiver<crate::AdapterCommand>,
        _update_event: &UpdateEvent,
    ) -> Result<(), AdapterError> {
        let mut model = model.write().expect("Should be able to lock for writing");

        model.event_name.set("Dummy event".to_string());
        model.active_camera.set(Camera::Hellicopter);
        model.available_cameras.insert(Camera::Hellicopter);
        model.available_cameras.insert(Camera::Chase);
        model.available_cameras.insert(Camera::FirstPerson);

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
            ambient_temp: Value::new(24.0),
            track_temp: Value::new(26.0),
            best_lap: Value::new(Some(Lap {
                time: Value::new(Time::from(81_1234)),
                splits: Value::new(vec![
                    Time::from(12_345),
                    Time::from(67_891),
                    Time::from(111_213),
                ]),
                driver_id: DriverId::default(),
                entry_id: EntryId::default(),
                invalid: Value::new(false),
            })),
            track_name: Value::new("Dummy track".to_string()),
            track_length: Value::new(1234),
            game_data: SessionGameData::None,
        });
        model.current_session = Some(id);
        model.events.push(Event::SessionChanged(SessionId(0)));

        for i in 0..10 {
            let session = model.current_session_mut().unwrap();
            let entry_id = EntryId(i);
            session.entries.insert(
                entry_id,
                Entry {
                    id: entry_id,
                    drivers: {
                        let mut drivers = HashMap::new();
                        for i in 0..3 {
                            let driver_id = DriverId(i);
                            drivers.insert(
                                driver_id,
                                Driver {
                                    id: driver_id,
                                    first_name: Value::new(format!("John")),
                                    last_name: Value::new(format!("Wayne {}", i)),
                                    short_name: Value::new(format!("JW{}", i)),
                                    nationality: Value::new(Nationality::NONE),
                                    driving_time: Value::new(Time::from(0)),
                                    best_lap: Value::new(None),
                                },
                            );
                        }
                        drivers
                    },
                    current_driver: DriverId(0),
                    team_name: Value::new(format!("Team nr.{}", i)),
                    car: Value::new(Car {
                        name: "Car model",
                        manufacturer: "Manufacturer",
                        category: CarCategory { name: "Car Cat" },
                    }),
                    car_number: Value::new(i),
                    nationality: Value::new(Nationality::NONE),
                    world_pos: Value::new([0.0, 0.0, 0.0]),
                    orientation: Value::new([0.0, 0.0, 0.0]),
                    position: Value::new(i),
                    spline_pos: Value::new(0.1234),
                    lap_count: Value::new(0),
                    laps: Vec::new(),
                    current_lap: Value::new(Lap {
                        time: Value::new(Time::from(12_345)),
                        splits: Value::new(Vec::new()),
                        driver_id: DriverId(0),
                        entry_id: EntryId(i),
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
                    game_data: EntryGameData::None,
                },
            );
        }

        Ok(())
    }
}

impl DummyAdapter {
    pub fn new() -> Self {
        Self {}
    }
}
