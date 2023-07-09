            X:      Fully done
            -:      Value created correclty but not filled correctly.
                            
                            ACC			iRacing
struct Model {
	connected:              X			X
	sessions:               X			X
	current_session:        X			X
	events:                 X			
	event_name:             X			
	active_camera:			X			
	available_cameras:		X			
	focused_entry:			X			
}

struct Session {
	id:                     X			X
	entries:                X			X
	session_type:           X			X
	phase:                  X			X
	session_time:           X			X
	time_remaining:         X			X
	laps:                   X			X
	laps_remaining:         X			X
	time_of_day:            X			X
	day:                    X			X	
	ambient_temp:           X			X
	track_temp:             X			X
	best_lap:               X			-
	track_name:				X			X
	track_length:			X			X
    game_data:              X			X
}

struct Entry {
	id:						X			X
	drivers:				X			X
	current_driver:			X			-
	team_name:				X			X
	car:					X			X
	car_number:				X			X
	nationality:			X			X
	world_pos:				X			X
	orientation:			X			X
	position:				X			-
	spline_pos:				X			-
	lap_count:				X			-
	laps:					X			-
	current_lap:			X			-
	best_lap:				X			-
	performance_delta:		X			-
	time_behind_leader:		X			-
	in_pits:				X			-
	gear:					X			-
	speed:					X			-
	connected:				X			X
	stint_time:				X			X
	distance_driven:		X			-
	focused:				X			-
	game_data:				X			X
}

struct Driver {
	id:						X			X
	first_name:				X			X
	last_name:				X			X
	short_name:				X			X
	nationality:			X			X
	driving_time:			X			X
	best_lap:				X			-
}

struct Lap {
	time:					X
	splits:					X
	invalid:				X
	driver_id:				X
	entry_id:				X
}
