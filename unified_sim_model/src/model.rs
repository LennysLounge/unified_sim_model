use std::{collections::HashMap, sync::Arc};

use indexmap::IndexMap;

use crate::time::Time;

/// The unified sim model.
#[derive(Debug, Default)]
pub struct Model {
    /// List of sessions that have happend during the event.
    /// Sessions are orderd in the order they occur in the event.
    pub sessions: IndexMap<SessionId, Session>,
    /// Index of the current active session.
    pub current_session: SessionId,
    /// Name of the event.
    pub event_name: String,
    /// Name of the track.
    pub track_name: String,
    /// Length of the track in meter.
    pub track_length: i32,
    /// List of events that have happened since the last update.
    pub events: Vec<Event>,
}

impl Model {
    /// Add a session to the model.
    /// Sets the id of the session and returns it.
    pub fn add_session(&mut self, mut session: Session) -> SessionId {
        let id = SessionId(self.sessions.len());
        session.id = id;
        self.sessions.insert(id, session);
        id
    }

    pub fn current_session(&self) -> Option<&Session> {
        self.sessions.get(&self.current_session)
    }

    pub fn current_session_mut(&mut self) -> Option<&mut Session> {
        self.sessions.get_mut(&self.current_session)
    }
}

#[derive(Debug)]
pub enum Event {
    EntryConnected(EntryId),
    EntryReconnected(EntryId),
    EntryDisconnected(EntryId),
    SessionChanged(SessionId),
    SessionPhaseChanged(SessionId, SessionPhase),
    LapCompleted(LapCompleted),
}

#[derive(Debug)]
pub struct LapCompleted {
    pub lap: Lap,
    pub is_session_best: bool,
    pub is_entry_best: bool,
    pub is_driver_best: bool,
}

/// A session id.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub usize);

/// A session.
#[derive(Debug, Default)]
pub struct Session {
    pub id: SessionId,
    pub entries: HashMap<EntryId, Entry>,
    pub session_type: SessionType,
    pub session_time: Time,
    pub time_remaining: Time,
    pub laps: i32,
    pub laps_remaining: i32,
    pub phase: SessionPhase,
    pub time_of_day: Time,
    pub day: Day,
    pub ambient_temp: f32,
    pub track_temp: f32,
    pub best_lap: Lap,
}

/// The type of the session.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SessionType {
    /// A practice session scored by best lap time.
    Practice,
    /// A qualifying session scored by best lap time.
    Qualifying,
    /// A Race session, scored by furthest distance.
    Race,
    /// Session type is unknown or unavailable.
    #[default]
    None,
}

/// The phase of the current session.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SessionPhase {
    /// The session phase is unknown or unavailable
    #[default]
    None,
    /// The session is waiting to start while a different session is active.
    Waiting,
    /// The session is preparing to start.
    /// Drivers and teams are getting ready.
    Preparing,
    /// The session is forming befor the start.
    /// This is mostly in form of a formation lap.
    Formation,
    /// The session is active and running.
    Active,
    /// The session is ending. The end condition for the session has been met
    /// (either lap count reached or timer expired etc.) and the session is waiting
    /// for all drivers to finish the session.
    Ending,
    /// The session is finished. All drivers have finished their session and the
    /// results of the session are finalised.
    Finished,
}

impl SessionPhase {
    /// Returns the next phase in order.
    /// Once session is in the finished state it does not advance further.
    pub fn next(&self) -> Self {
        use SessionPhase::*;
        match self {
            None => Waiting,
            Waiting => Preparing,
            Preparing => Formation,
            Formation => Active,
            Active => Ending,
            Ending => Finished,
            Finished => Finished,
        }
    }
}

/// Describes the day a session takes part in.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Day {
    Monday,
    Thuesday,
    Wednesday,
    Thrusday,
    Friday,
    Saturday,
    #[default]
    Sunday,
}

/// An id for an entry.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntryId(pub i32);

/// A team entry in the session.
#[derive(Debug, Default)]
pub struct Entry {
    pub id: EntryId,
    pub drivers: HashMap<DriverId, Driver>,
    pub current_driver: DriverId,
    pub team_name: String,
    pub car: Car,
    pub car_number: i32,
    pub nationality: Nationality,
    pub world_pos: [f32; 3],
    pub orientation: [f32; 3],
    pub position: i32,
    pub spline_pos: f32,
    pub lap_count: i32,
    pub laps: Vec<Lap>,
    pub current_lap: Lap,
    pub best_lap: Option<usize>,
    pub performance_delta: Time,
    pub time_behind_leader: Time,
    pub in_pits: bool,
    pub gear: i32,
    pub speed: f32,
    pub connected: bool,
    pub stint_time: Time,
    pub distance_driven: f32,
}

/// An id for a driver.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DriverId(pub i32);

/// A Driver in a team.
#[derive(Debug, Default)]
pub struct Driver {
    pub id: DriverId,
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub nationality: Nationality,
    pub driving_time: Time,
    pub best_lap: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Lap {
    pub time: Time,
    pub splits: Vec<Time>,
    pub driver_id: DriverId,
    pub entry_id: EntryId,
    pub invalid: bool,
}

impl Default for Lap {
    fn default() -> Self {
        Self {
            time: Time::from(i32::MAX),
            splits: Default::default(),
            driver_id: Default::default(),
            entry_id: Default::default(),
            invalid: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CarCategory {
    pub name: &'static str,
}

impl CarCategory {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Car {
    pub name: &'static str,
    pub manufacturer: &'static str,
    pub category: CarCategory,
}

impl Car {
    pub const fn new(
        name: &'static str,
        manufacturer: &'static str,
        category: CarCategory,
    ) -> Self {
        Self {
            name,
            manufacturer,
            category,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Nationality {
    pub name: &'static str,
}

impl Nationality {
    const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const NONE: Self = Self::new("None");
    pub const AFGHANISTAN: Self = Self::new("Afghanistan");
    pub const ALBANIA: Self = Self::new("Albania");
    pub const ALGERIA: Self = Self::new("Algeria");
    pub const ANDORRA: Self = Self::new("Andorra");
    pub const ANGOLA: Self = Self::new("Angola");
    pub const ANTIGUAANDBARBUDA: Self = Self::new("Antigua and Barbuda");
    pub const ARGENTINA: Self = Self::new("Argentina");
    pub const ARMENIA: Self = Self::new("Armenia");
    pub const AUSTRALIA: Self = Self::new("Australia");
    pub const AUSTRIA: Self = Self::new("Austria");
    pub const AZERBAIJAN: Self = Self::new("Azerbaijan");
    pub const BAHAMAS: Self = Self::new("Bahamas");
    pub const BAHRAIN: Self = Self::new("Bahrain");
    pub const BANGLADESH: Self = Self::new("Bangladesh");
    pub const BARBADOS: Self = Self::new("Barbados");
    pub const BELARUS: Self = Self::new("Belarus");
    pub const BELGIUM: Self = Self::new("Belgium");
    pub const BELIZE: Self = Self::new("Belize");
    pub const BENIN: Self = Self::new("Benin");
    pub const BHUTAN: Self = Self::new("Bhutan");
    pub const BOLIVIA: Self = Self::new("Bolivia");
    pub const BOSNIAANDHERZEGOVINA: Self = Self::new("Bosnia and Herzegovina");
    pub const BOTSWANA: Self = Self::new("Botswana");
    pub const BRAZIL: Self = Self::new("Brazil");
    pub const BRUNEI: Self = Self::new("Brunei");
    pub const BULGARIA: Self = Self::new("Bulgaria");
    pub const BURKINAFASO: Self = Self::new("Burkina Faso");
    pub const BURUNDI: Self = Self::new("Burundi");
    pub const CÔTEDIVOIRE: Self = Self::new("Côte d'Ivoire");
    pub const CABOVERDE: Self = Self::new("Cabo Verde");
    pub const CAMBODIA: Self = Self::new("Cambodia");
    pub const CAMEROON: Self = Self::new("Cameroon");
    pub const CANADA: Self = Self::new("Canada");
    pub const CENTRALAFRICANREPUBLIC: Self = Self::new("Central African Republic");
    pub const CHAD: Self = Self::new("Chad");
    pub const CHILE: Self = Self::new("Chile");
    pub const CHINA: Self = Self::new("China");
    pub const COLOMBIA: Self = Self::new("Colombia");
    pub const COMOROS: Self = Self::new("Comoros");
    pub const CONGO: Self = Self::new("Congo");
    pub const COSTARICA: Self = Self::new("Costa Rica");
    pub const CROATIA: Self = Self::new("Croatia");
    pub const CUBA: Self = Self::new("Cuba");
    pub const CYPRUS: Self = Self::new("Cyprus");
    pub const CZECHIA: Self = Self::new("Czechia");
    pub const DEMOCRATICREPUBLICOFTHECONGO: Self = Self::new("Democratic Republic of the Congo");
    pub const DENMARK: Self = Self::new("Denmark");
    pub const DJIBOUTI: Self = Self::new("Djibouti");
    pub const DOMINICA: Self = Self::new("Dominica");
    pub const DOMINICANREPUBLIC: Self = Self::new("Dominican Republic");
    pub const ECUADOR: Self = Self::new("Ecuador");
    pub const EGYPT: Self = Self::new("Egypt");
    pub const ELSALVADOR: Self = Self::new("El Salvador");
    pub const EQUATORIALGUINEA: Self = Self::new("Equatorial Guinea");
    pub const ERITREA: Self = Self::new("Eritrea");
    pub const ESTONIA: Self = Self::new("Estonia");
    pub const ESWATINI: Self = Self::new("Eswatini");
    pub const ETHIOPIA: Self = Self::new("Ethiopia");
    pub const FIJI: Self = Self::new("Fiji");
    pub const FINLAND: Self = Self::new("Finland");
    pub const FRANCE: Self = Self::new("France");
    pub const GABON: Self = Self::new("Gabon");
    pub const GAMBIA: Self = Self::new("Gambia");
    pub const GEORGIA: Self = Self::new("Georgia");
    pub const GERMANY: Self = Self::new("Germany");
    pub const GHANA: Self = Self::new("Ghana");
    pub const GREECE: Self = Self::new("Greece");
    pub const GRENADA: Self = Self::new("Grenada");
    pub const GUATEMALA: Self = Self::new("Guatemala");
    pub const GUINEA: Self = Self::new("Guinea");
    pub const GUINEABISSAU: Self = Self::new("Guinea-Bissau");
    pub const GUYANA: Self = Self::new("Guyana");
    pub const HONGKONG: Self = Self::new("HongKong");
    pub const HAITI: Self = Self::new("Haiti");
    pub const HOLYSEE: Self = Self::new("Holy See");
    pub const HONDURAS: Self = Self::new("Honduras");
    pub const HUNGARY: Self = Self::new("Hungary");
    pub const ICELAND: Self = Self::new("Iceland");
    pub const INDIA: Self = Self::new("India");
    pub const INDONESIA: Self = Self::new("Indonesia");
    pub const IRAN: Self = Self::new("Iran");
    pub const IRAQ: Self = Self::new("Iraq");
    pub const IRELAND: Self = Self::new("Ireland");
    pub const ISRAEL: Self = Self::new("Israel");
    pub const ITALY: Self = Self::new("Italy");
    pub const JAMAICA: Self = Self::new("Jamaica");
    pub const JAPAN: Self = Self::new("Japan");
    pub const JORDAN: Self = Self::new("Jordan");
    pub const KAZAKHSTAN: Self = Self::new("Kazakhstan");
    pub const KENYA: Self = Self::new("Kenya");
    pub const KIRIBATI: Self = Self::new("Kiribati");
    pub const KUWAIT: Self = Self::new("Kuwait");
    pub const KYRGYZSTAN: Self = Self::new("Kyrgyzstan");
    pub const LAOS: Self = Self::new("Laos");
    pub const LATVIA: Self = Self::new("Latvia");
    pub const LEBANON: Self = Self::new("Lebanon");
    pub const LESOTHO: Self = Self::new("Lesotho");
    pub const LIBERIA: Self = Self::new("Liberia");
    pub const LIBYA: Self = Self::new("Libya");
    pub const LIECHTENSTEIN: Self = Self::new("Liechtenstein");
    pub const LITHUANIA: Self = Self::new("Lithuania");
    pub const LUXEMBOURG: Self = Self::new("Luxembourg");
    pub const MACAU: Self = Self::new("Macau");
    pub const MADAGASCAR: Self = Self::new("Madagascar");
    pub const MALAWI: Self = Self::new("Malawi");
    pub const MALAYSIA: Self = Self::new("Malaysia");
    pub const MALDIVES: Self = Self::new("Maldives");
    pub const MALI: Self = Self::new("Mali");
    pub const MALTA: Self = Self::new("Malta");
    pub const MARSHALLISLANDS: Self = Self::new("Marshall Islands");
    pub const MAURITANIA: Self = Self::new("Mauritania");
    pub const MAURITIUS: Self = Self::new("Mauritius");
    pub const MEXICO: Self = Self::new("Mexico");
    pub const MICRONESIA: Self = Self::new("Micronesia");
    pub const MOLDOVA: Self = Self::new("Moldova");
    pub const MONACO: Self = Self::new("Monaco");
    pub const MONGOLIA: Self = Self::new("Mongolia");
    pub const MONTENEGRO: Self = Self::new("Montenegro");
    pub const MOROCCO: Self = Self::new("Morocco");
    pub const MOZAMBIQUE: Self = Self::new("Mozambique");
    pub const MYANMAR: Self = Self::new("Myanmar");
    pub const NAMIBIA: Self = Self::new("Namibia");
    pub const NAURU: Self = Self::new("Nauru");
    pub const NEPAL: Self = Self::new("Nepal");
    pub const NETHERLANDS: Self = Self::new("Netherlands");
    pub const NEWCALEDONIA: Self = Self::new("New Caledonia");
    pub const NEWZEALAND: Self = Self::new("New Zealand");
    pub const NICARAGUA: Self = Self::new("Nicaragua");
    pub const NIGER: Self = Self::new("Niger");
    pub const NIGERIA: Self = Self::new("Nigeria");
    pub const NORTHERNIRELAND: Self = Self::new("northern ireland");
    pub const NORTHKOREA: Self = Self::new("North Korea");
    pub const NORTHMACEDONIA: Self = Self::new("North Macedonia");
    pub const NORWAY: Self = Self::new("Norway");
    pub const OMAN: Self = Self::new("Oman");
    pub const PAKISTAN: Self = Self::new("Pakistan");
    pub const PALAU: Self = Self::new("Palau");
    pub const PALESTINESTATE: Self = Self::new("Palestine State");
    pub const PANAMA: Self = Self::new("Panama");
    pub const PAPUANEWGUINEA: Self = Self::new("Papua New Guinea");
    pub const PARAGUAY: Self = Self::new("Paraguay");
    pub const PERU: Self = Self::new("Peru");
    pub const PHILIPPINES: Self = Self::new("Philippines");
    pub const POLAND: Self = Self::new("Poland");
    pub const PORTUGAL: Self = Self::new("Portugal");
    pub const PUERTORICO: Self = Self::new("Puerto Rico");
    pub const QATAR: Self = Self::new("Qatar");
    pub const ROMANIA: Self = Self::new("Romania");
    pub const RUSSIA: Self = Self::new("Russia");
    pub const RWANDA: Self = Self::new("Rwanda");
    pub const SAINTKITTSANDNEVIS: Self = Self::new("Saint Kitts and Nevis");
    pub const SAINTLUCIA: Self = Self::new("Saint Lucia");
    pub const SAINTVINCENTANDTHEGRENADINES: Self = Self::new("Saint Vincent and the Grenadines");
    pub const SAMOA: Self = Self::new("Samoa");
    pub const SANMARINO: Self = Self::new("San Marino");
    pub const SAOTOMEANDPRINCIPE: Self = Self::new("Sao Tome and Principe");
    pub const SAUDIARABIA: Self = Self::new("Saudi Arabia");
    pub const SCOTLAND: Self = Self::new("Scotland");
    pub const SENEGAL: Self = Self::new("Senegal");
    pub const SERBIA: Self = Self::new("Serbia");
    pub const SEYCHELLES: Self = Self::new("Seychelles");
    pub const SIERRALEONE: Self = Self::new("Sierra Leone");
    pub const SINGAPORE: Self = Self::new("Singapore");
    pub const SLOVAKIA: Self = Self::new("Slovakia");
    pub const SLOVENIA: Self = Self::new("Slovenia");
    pub const SOLOMONISLANDS: Self = Self::new("Solomon Islands");
    pub const SOMALIA: Self = Self::new("Somalia");
    pub const SOUTHAFRICA: Self = Self::new("South Africa");
    pub const SOUTHKOREA: Self = Self::new("South Korea");
    pub const SOUTHSUDAN: Self = Self::new("South Sudan");
    pub const SPAIN: Self = Self::new("Spain");
    pub const SRILANKA: Self = Self::new("Sri Lanka");
    pub const SUDAN: Self = Self::new("Sudan");
    pub const SURINAME: Self = Self::new("Suriname");
    pub const SWEDEN: Self = Self::new("Sweden");
    pub const SWITZERLAND: Self = Self::new("Switzerland");
    pub const SYRIA: Self = Self::new("Syria");
    pub const TAIWAN: Self = Self::new("Taiwan");
    pub const TAJIKISTAN: Self = Self::new("Tajikistan");
    pub const TANZANIA: Self = Self::new("Tanzania");
    pub const THAILAND: Self = Self::new("Thailand");
    pub const TIMORLESTE: Self = Self::new("Timor-Leste");
    pub const TOGO: Self = Self::new("Togo");
    pub const TONGA: Self = Self::new("Tonga");
    pub const TRINIDADANDTOBAGO: Self = Self::new("Trinidad and Tobago");
    pub const TUNISIA: Self = Self::new("Tunisia");
    pub const TURKEY: Self = Self::new("Turkey");
    pub const TURKMENISTAN: Self = Self::new("Turkmenistan");
    pub const TUVALU: Self = Self::new("Tuvalu");
    pub const UGANDA: Self = Self::new("Uganda");
    pub const UKRAINE: Self = Self::new("Ukraine");
    pub const UNITEDARABEMIRATES: Self = Self::new("United Arab Emirates");
    pub const UNITEDKINGDOM: Self = Self::new("United Kingdom");
    pub const UNITEDSTATESOFAMERICA: Self = Self::new("United States of America");
    pub const URUGUAY: Self = Self::new("Uruguay");
    pub const UZBEKISTAN: Self = Self::new("Uzbekistan");
    pub const VANUATU: Self = Self::new("Vanuatu");
    pub const VENEZUELA: Self = Self::new("Venezuela");
    pub const VIETNAM: Self = Self::new("Vietnam");
    pub const WALES: Self = Self::new("Wales");
    pub const YEMEN: Self = Self::new("Yemen");
    pub const ZAMBIA: Self = Self::new("Zambia");
    pub const ZIMBABWE: Self = Self::new("Zimbabwe");
}

/// Set of possible camera views.
pub enum Camera {
    /// The first person view of the driver. This is usually the view a
    /// player would use to drive.
    FirstPerson,
    /// A third person chase cam where the camera is elevated behind the car
    /// and is following it.
    Chase,
    /// A camera like you would see in a tv broadcast. This is usually not a single
    /// camera but a collection of cameras placed around the track pointing at the car.
    /// The game would automatically switch between these cameras to keep the player
    /// in view.
    TV,
    /// A helicopter view of the focused car.
    Hellicopter,
    /// An Camera definition for ACC. This camera is special to ACC and does not
    /// fit to a general camera definition.
    Acc {
        camera_set: Arc<str>,
        camera: Arc<str>,
    },
}
