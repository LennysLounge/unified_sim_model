//! The model combines all the available data into a single unified data model.
//!
//! The starting point is the [`Model`] where as the base object where all other
//! data is stored.
//!
//! ## Availability:
//! Not all games provide the same data and some values are not available in a game
//! at all. Instead of representing this "optionality" of the data with the Option enum
//! the ['Value'] object is used instead.
//! The ['Value'] object uses a default value to return if the value is not naturally available
//! from the game. This is done to keep the code that used the model easier to use.
//!
//! Sometimes it is required to know if a specific value is a available from the game
//! or if a default is used. To do this, the ['Value'] object has some flags to read this information.

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::{Deref, DerefMut},
};

use indexmap::IndexMap;

use crate::{
    games::acc::model::{AccCamera, AccEntry, AccSession},
    time::Time,
    Distance, Temperature,
};

/// A single piece of data in the model that carries extra information about its
/// availability and editability.
///
/// If the value is not available in the connected game then the `available`
/// flag will be set to false and the wrapped value will be a default value.
/// Unless otherwise specified, the default value is the default for the type.
///
/// If the `editable` flag is set then the value can be edited by the user.
/// To edit a value, send the appropriate adapter command. The adapter may decide
/// to overwrite the value set by the user or set the `editable` flag to false at any time.
///
/// The specific behavior of the game adapter is documented in the documentation for the value.
#[derive(Debug, Clone)]
pub struct Value<T> {
    value: T,
    editable: bool,
    available: bool,
}

impl<T: Default> Default for Value<T> {
    /// Create a value with the default for the type.
    /// `editable` is false.
    /// `available` is false.
    fn default() -> Self {
        Self {
            value: Default::default(),
            editable: false,
            available: false,
        }
    }
}

impl<T> AsRef<T> for Value<T> {
    /// Return the inner value as a reference.
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> Value<T> {
    /// Create a value with a given inner value and the `available` flag set to true.
    pub fn new(value: T) -> Self {
        Self {
            value,
            editable: false,
            available: true,
        }
    }

    /// Create a value with a specific default value.
    pub fn with_default(value: T) -> Self {
        Self {
            value,
            editable: false,
            available: false,
        }
    }

    /// Set the editable flag for the value.
    pub fn with_editable(mut self) -> Self {
        self.editable = true;
        self
    }

    /// Set the editable flag for this value.
    pub fn set_editable(&mut self) {
        self.editable = true;
    }

    /// Set the value to be available.
    pub fn set_available(&mut self) {
        self.available = true;
    }

    /// Set the value to be unavailable.
    pub fn set_unavailable(&mut self) {
        self.available = false;
    }

    /// Set the inner value to a value provided by the game.
    /// This sets the `available` flag to true and the `editable` flag to false.
    ///
    /// Generally once a value is available from the game it should not be editable anymore.
    /// For some values it may be desireable to take the game provided value as a suggestion and allow
    /// the user to overwrite it. In that case use the `edit` method to set the value and continue
    /// to allow editing.
    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
        self.available = true;
    }

    /// Set the inner value to a custom value.
    /// This sets the `available` flag since the value is no longer represented by its default.
    ///
    /// This does not change the editability of the value.
    pub fn edit(&mut self, new_value: T) {
        self.value = new_value;
        self.available = true;
    }

    /// Return if this value is available.
    pub fn is_avaliable(&self) -> bool {
        self.available
    }

    /// Return if this value is editable.
    pub fn is_editable(&self) -> bool {
        self.editable
    }
}

impl<T: Copy> Value<T> {
    /// Get the inner value as a copy.
    pub fn as_copy(&self) -> T {
        self.value
    }
}

impl<T: PartialEq> PartialEq<T> for Value<T> {
    fn eq(&self, other: &T) -> bool {
        &self.value == other
    }
}

impl<T: PartialEq> PartialEq for Value<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialOrd> PartialOrd<T> for Value<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<T: PartialOrd> PartialOrd for Value<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Deref for Value<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Value<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Display> Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

/// The unified sim model.
/// Holds all the date availabe from the game.
#[derive(Debug, Default)]
pub struct Model {
    /// Shows if the adapter is currently receiving data from the game.
    pub connected: bool,
    /// List of sessions that have happend during the event.
    /// Sessions are orderd in the order they occur in the event.
    pub sessions: IndexMap<SessionId, Session>,
    /// Id of the current active session.
    /// `None` if there is no active session.
    pub current_session: Option<SessionId>,
    /// List of events that have happened during the liftime of the adapter.
    pub events: Vec<Event>,
    /// Name of the event.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// In Acc there is no event name or server name available. Instead
    /// the default "Assetto Corsa Competizione" is used. This value is editable during
    /// the entire duration of the connection.
    pub event_name: Value<String>,
    /// The currently active camera.
    pub active_camera: Value<Camera>,
    /// The set of availabe cameras.
    pub available_cameras: HashSet<Camera>,
    /// The currently focused car.
    pub focused_entry: Option<EntryId>,
}

impl Model {
    /// Add a session to the model.
    /// Generates a new id for the session and adds it to the model.
    /// Returns the newly created id.
    pub fn add_session(&mut self, mut session: Session) -> SessionId {
        let id = SessionId(self.sessions.len());
        session.id = id;
        self.sessions.insert(id, session);
        id
    }

    /// Convenience method to access the current session.
    /// `None` if there is no current session.
    pub fn current_session(&self) -> Option<&Session> {
        self.sessions.get(&self.current_session?)
    }

    /// Get the current session mutably.
    /// `None` if there is no current session.
    pub fn current_session_mut(&mut self) -> Option<&mut Session> {
        self.sessions.get_mut(&self.current_session?)
    }

    /// Returns if the given camera is available.
    pub fn is_camera_available(&self, camera: &Camera) -> bool {
        self.available_cameras.contains(camera)
    }
}

/// The identifier for a session.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub usize);

/// A session.
#[derive(Debug, Default, Clone)]
pub struct Session {
    /// The session id of this session
    pub id: SessionId,
    /// The collection of entries that are registered to this session.
    pub entries: HashMap<EntryId, Entry>,
    /// The current session type.
    pub session_type: Value<SessionType>,
    /// The current phase of the session.
    pub phase: Value<SessionPhase>,
    /// The time limit for this session.
    ///
    /// ### Availability:
    /// If the session is not a timed session then this will not be available.
    pub session_time: Value<Time>,
    /// The time remaining in this session.
    ///
    /// ### Availability:
    /// If the session is not a timed session then this will not be available.
    pub time_remaining: Value<Time>,
    /// The amount of laps required to finish this session.
    ///
    /// ### Availability:
    /// If the session is not a lapped session then this will not be available.
    pub laps: Value<i32>,
    /// The amount of laps remaining to finish this session.
    ///
    /// ### Availability:
    /// If the session is not a lapped session then this will not be available.
    pub laps_remaining: Value<i32>,
    /// The current time of day in the game.
    pub time_of_day: Value<Time>,
    /// The day of the week in the game.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// The week day of the session is availabe in Acc. It will default to be sunday.
    /// This value is editable for the entire duration of the event.
    /// - **iRacing:**
    /// Not yet implemented.
    pub day: Value<Day>,
    /// The air temperature.
    pub ambient_temp: Value<Temperature>,
    /// The track temperature
    pub track_temp: Value<Temperature>,
    /// The best lap of the session.
    pub best_lap: Value<Option<Lap>>,
    /// Name of the track.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// After the session changes or when the adapter first connects there might be a short delay before
    /// the track name is availabe.
    pub track_name: Value<String>,
    /// Length of the track in meter.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// After the session changes or when the adapter first connects there might be a short delay before
    /// the track length is availabe.
    pub track_length: Value<Distance>,
    /// Contains additional data that is game specific.
    pub game_data: SessionGameData,
}

/// Game specific session data.
#[derive(Debug, Default, Clone)]
pub enum SessionGameData {
    #[default]
    None,
    Acc(AccSession),
}

/// The identifier for an entry.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntryId(pub i32);

/// A team entry in the session.
#[derive(Debug, Default, Clone)]
pub struct Entry {
    /// The id for this entry.
    pub id: EntryId,
    /// The collection of drivers registered for this entry.
    pub drivers: HashMap<DriverId, Driver>,
    /// The currently driving drivier.
    pub current_driver: DriverId,
    /// The name of the team.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// Team names are not available.
    /// This value can be edited for the entire duration of the connection.
    pub team_name: Value<String>,
    /// The car this entry is driving.
    pub car: Value<Car>,
    /// The car number for this entry.
    pub car_number: Value<i32>,
    /// The nationality of the entry as a whole.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// Team nationality is not availabe.
    /// This value can be edited for the entire duration of the connection.
    pub nationality: Value<Nationality>,
    /// The position of this car in an x, y, z coordinate system.
    ///
    /// ### Availability:
    /// - ** Assetto Corsa Competizione:**
    /// The world position is not availabe in ACC.
    /// TODO: It is possible to approximate the world position using the spline position
    /// and the track map.
    pub world_pos: Value<[f32; 3]>,
    /// The orientation of the car in the pitch, yaw, and roll axis.
    pub orientation: Value<[f32; 3]>,
    /// The classification position of this entry.
    pub position: Value<i32>,
    /// The spline position around the track from 0 to 1.
    pub spline_pos: Value<f32>,
    /// The ammount of laps completed by this entry.
    pub lap_count: Value<i32>,
    /// List of all laps completed by this entry.
    pub laps: Vec<Lap>,
    /// The current lap time data for this entry.
    pub current_lap: Value<Lap>,
    /// The best lap this entry has completed.
    pub best_lap: Value<Option<usize>>,
    /// The performance delta compared to the best lap.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// It is a little unclear what the reference lap time for the performance delta is.
    /// The best guess right now is that it references the best lap of the current stint.
    pub performance_delta: Value<Time>,
    /// The time difference from the leader of the session to this entry.
    /// In a timed session, this is the difference in lap time. Otherwise it is the difference
    /// in time between the leader reaching a checkpoint and this entry reaching the same checkpoint.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// This value is currently not implemented for ACC.
    pub time_behind_leader: Value<Time>,
    /// If the entry is currently in the pitlane or not.
    pub in_pits: Value<bool>,
    /// The gear of the entry.
    pub gear: Value<i32>,
    /// The current speed of the entry in m/s.
    pub speed: Value<f32>,
    /// If the entry is currently connected to the session.
    pub connected: Value<bool>,
    /// The current stint time of the entry.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// Stint time is not implemented for ACC yet.
    pub stint_time: Value<Time>,
    /// The distance driven by this entry in laps.
    /// This is simply the lap count + the current lap progress from the spline position.
    pub distance_driven: Value<f32>,
    /// True if this car is the focus of the camera right now.
    pub focused: bool,
    /// Contains additional data that is game specific.
    pub game_data: EntryGameData,
}

/// Game specific entry data.
#[derive(Debug, Default, Clone)]
pub enum EntryGameData {
    #[default]
    None,
    Acc(AccEntry),
}

/// An iddentifier for a driver.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DriverId(pub i32);

/// A Driver in a entry.
#[derive(Debug, Default, Clone)]
pub struct Driver {
    /// The id of the driver.
    pub id: DriverId,
    /// The first name of the driver.
    pub first_name: Value<String>,
    /// The last name of the driver.
    pub last_name: Value<String>,
    /// The short name of the driver.
    pub short_name: Value<String>,
    /// Nationality of the driver.
    pub nationality: Value<Nationality>,
    /// Total driving time this driver has done in the current session.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// Driving time is not yet implemented for ACC.
    pub driving_time: Value<Time>,
    /// The best lap this driver has done.
    /// This indexes the lap list in the entry of this driver.
    pub best_lap: Value<Option<usize>>,
}

/// Data about a single lap.
#[derive(Debug, Default, Clone)]
pub struct Lap {
    /// The lap time of this lap.
    pub time: Value<Time>,
    /// The splits of this lap.
    ///
    /// ### Availability:
    /// - **Assetto Corsa Competizione:**
    /// Split times as not availabe for a lap that hasnt finished yet.
    /// Only completed laps have split times availabe.
    pub splits: Value<Vec<Time>>,
    /// If the lap was invalid.
    pub invalid: Value<bool>,
    /// Id of the driver that drove this lap.
    pub driver_id: DriverId,
    /// Id of the entry that drove this lap.
    pub entry_id: EntryId,
}

#[derive(Debug, Default, Clone)]
pub struct CarCategory {
    pub name: &'static str,
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
    /// results of the session is finalised.
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

/// Describes the category of a car.
impl CarCategory {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

/// A car model.
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

/// Nationality.
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
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum Camera {
    /// No camera is active.
    #[default]
    None,
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
    /// Game specific camera.
    Game(GameCamera),
}

/// Game specific camera options.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum GameCamera {
    #[default]
    None,
    Acc(AccCamera),
}
