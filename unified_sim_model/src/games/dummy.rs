use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use crate::{
    model::{
        Car, CarCategory, Day, Driver, DriverId, Entry, EntryId, Event, Lap, Model, Nationality,
        Session, SessionId, SessionPhase, SessionType,
    },
    time::Time,
    AdapterError,
};

pub struct DummyAdapter {
    model: Arc<RwLock<Model>>,
}

impl DummyAdapter {
    pub fn spawn(model: Arc<RwLock<Model>>) -> JoinHandle<Result<(), AdapterError>> {
        thread::Builder::new()
            .name("Dummy Adapter".into())
            .spawn(move || {
                let adapter = Self { model };
                adapter.run()
            })
            .expect("should be able to spawn thread")
    }

    fn run(self) -> Result<(), AdapterError> {
        let mut model = self
            .model
            .write()
            .expect("Should be able to lock for writing");

        model.event_name = "Dummy event".to_string();
        model.track_name = "Dummy track".to_string();
        model.track_length = 1234;

        model.add_session(Session {
            id: SessionId(0),
            entries: HashMap::new(),
            session_type: SessionType::Race,
            session_time: Time::from(1_200_123),
            time_remaining: Time::from(754_123),
            laps: 20,
            laps_remaining: 12,
            phase: SessionPhase::Active,
            time_of_day: Time::from(50_846_123),
            day: Day::Sunday,
            ambient_temp: 24.0,
            track_temp: 26.0,
            best_lap: Lap {
                time: Time::from(81_1234),
                splits: vec![Time::from(12_345), Time::from(67_891), Time::from(111_213)],
                driver_id: DriverId::default(),
                entry_id: EntryId::default(),
                invalid: false,
            },
        });
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
                                    first_name: format!("John"),
                                    last_name: format!("Wayne {}", i),
                                    short_name: format!("JW{}", i),
                                    nationality: Nationality::NONE,
                                    driving_time: Time::from(0),
                                    best_lap: None,
                                },
                            );
                        }
                        drivers
                    },
                    current_driver: DriverId(0),
                    team_name: format!("Team nr.{}", i),
                    car: Car {
                        name: "Car model",
                        manufacturer: "Manufacturer",
                        category: CarCategory { name: "Car Cat" },
                    },
                    car_number: i,
                    nationality: Nationality::NONE,
                    world_pos: [0.0, 0.0, 0.0],
                    orientation: [0.0, 0.0, 0.0],
                    position: i,
                    spline_pos: 0.1234,
                    lap_count: 0,
                    laps: Vec::new(),
                    current_lap: Lap {
                        time: Time::from(12_345),
                        splits: Vec::new(),
                        driver_id: DriverId(0),
                        entry_id: EntryId(i),
                        invalid: i % 2 == 0,
                    },
                    best_lap: None,
                    performance_delta: Time::from(-1_234),
                    time_behind_leader: Time::from(12_345),
                    in_pits: true,
                    gear: 4,
                    speed: 128.0,
                    connected: true,
                    stint_time: Time::from(56_789),
                    distance_driven: i as f32 * 0.345,
                },
            );
        }

        Ok(())
    }
}
