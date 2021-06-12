table! {
	artists (id) {
		id -> Integer,
		name -> Text,
	}
}

table! {
	composers (id) {
		id -> Integer,
		name -> Text,
	}
}

table! {
	genres (id) {
		id -> Integer,
		name -> Text,
	}
}

table! {
	ddns_config (id) {
		id -> Integer,
		host -> Text,
		username -> Text,
		password -> Text,
	}
}

table! {
	directories (id) {
		id -> Integer,
		path -> Text,
		parent -> Nullable<Text>,
		year -> Nullable<Integer>,
		album -> Nullable<Text>,
		artwork -> Nullable<Text>,
		date_added -> Integer,
	}
}

table! {
	directory_artists (id) {
		id -> Integer,
		directory -> Integer,
		artist -> Integer,
	}
}

table! {
	lyricists (id) {
		id -> Integer,
		name -> Text,
	}
}

table! {
	misc_settings (id) {
		id -> Integer,
		auth_secret -> Binary,
		index_sleep_duration_seconds -> Integer,
		index_album_art_pattern -> Text,
	}
}

table! {
	mount_points (id) {
		id -> Integer,
		source -> Text,
		name -> Text,
	}
}

table! {
	playlist_songs (id) {
		id -> Integer,
		playlist -> Integer,
		path -> Text,
		ordering -> Integer,
	}
}

table! {
	playlists (id) {
		id -> Integer,
		owner -> Integer,
		name -> Text,
	}
}

table! {
	songs (id) {
		id -> Integer,
		path -> Text,
		parent -> Text,
		track_number -> Nullable<Integer>,
		disc_number -> Nullable<Integer>,
		title -> Nullable<Text>,
		year -> Nullable<Integer>,
		album -> Nullable<Text>,
		artwork -> Nullable<Text>,
		duration -> Nullable<Integer>,
		label -> Nullable<Text>,
	}
}

table! {
	song_album_artists (id) {
		id -> Integer,
		song -> Integer,
		artist -> Integer,
	}
}

table! {
	song_artists (id) {
		id -> Integer,
		song -> Integer,
		artist -> Integer,
	}
}

table! {
	song_composers (id) {
		id -> Integer,
		song -> Integer,
		composer -> Integer,
	}
}

table! {
	song_genres (id) {
		id -> Integer,
		song -> Integer,
		genre -> Integer,
	}
}

table! {
	song_lyricists (id) {
		id -> Integer,
		song -> Integer,
		lyricist -> Integer,
	}
}

table! {
	users (id) {
		id -> Integer,
		name -> Text,
		password_hash -> Text,
		admin -> Integer,
		lastfm_username -> Nullable<Text>,
		lastfm_session_key -> Nullable<Text>,
		web_theme_base -> Nullable<Text>,
		web_theme_accent -> Nullable<Text>,
	}
}

joinable!(song_artists -> songs (song));
joinable!(song_artists -> artists (artist));

joinable!(song_album_artists -> songs (song));
joinable!(song_album_artists -> artists (artist));

joinable!(song_lyricists -> songs (song));
joinable!(song_lyricists -> lyricists (lyricist));

joinable!(song_composers -> songs (song));
joinable!(song_composers -> composers (composer));

joinable!(song_genres -> songs (song));
joinable!(song_genres -> genres (genre));

joinable!(directory_artists -> directories (directory));
joinable!(directory_artists -> artists (artist));

joinable!(playlist_songs -> playlists (playlist));
joinable!(playlists -> users (owner));

allow_tables_to_appear_in_same_query!(
	artists,
	composers,
	ddns_config,
	directories,
	directory_artists,
	genres,
	lyricists,
	misc_settings,
	mount_points,
	playlist_songs,
	playlists,
	song_album_artists,
	song_artists,
	song_composers,
	song_genres,
	song_lyricists,
	songs,
	users,
);
