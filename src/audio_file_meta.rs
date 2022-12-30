use lofty::TaggedFile;

pub struct AudioFileMeta {
    tagged_file: TaggedFile,
    album_artist_name: Option<String>,
    artist_name: Option<String>,
    album_title: Option<String>,
    year: Option<u32>,
    track_number: Option<u32>,
    track_title: Option<String>,
    genre: Option<String>,
    audio_file_type: Option<AudioFileType>
}

pub enum AudioFileType {
    Flac,
    Mp3
}

impl AudioFileMeta {
    pub fn new(
        tagged_file: TaggedFile,
        album_artist_name: Option<String>,
        artist_name: Option<String>,
        album_title: Option<String>,
        year: Option<u32>,
        track_number: Option<u32>,
        track_title: Option<String>,
        genre: Option<String>,
        audio_file_type: Option<AudioFileType>
    ) -> AudioFileMeta {
        AudioFileMeta {
            tagged_file,
            album_artist_name,
            artist_name,
            album_title,
            year,
            track_number,
            track_title,
            genre,
            audio_file_type
        }
    }

    pub fn tagged_file(&self) -> &TaggedFile {
        &self.tagged_file
    }

    pub fn album_artist_name(&self) -> Option<&str> {
        self.album_artist_name.as_deref()
    }

    pub fn album_title(&self) -> Option<&str> {
        self.album_title.as_deref()
    }

    pub fn year(&self) -> Option<u32> {
        self.year
    }

    pub fn artist_name(&self) -> Option<&str> {
        self.artist_name.as_deref()
    }

    pub fn track_title(&self) -> Option<&str> {
        self.track_title.as_deref()
    }

    pub fn track_number(&self) -> Option<u32> {
        self.track_number
    }

    pub fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }

    pub fn audio_file_type(&self) -> Option<&AudioFileType> {
        self.audio_file_type.as_ref()
    }
}

impl AudioFileType {

    pub fn to_extension(&self) -> &str {
        match *self {
            AudioFileType::Flac => "flac",
            AudioFileType::Mp3 => "mp3"
        }
    }
}

impl std::str::FromStr for AudioFileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "flac" => Ok(AudioFileType::Flac),
            "mp3" => Ok(AudioFileType::Mp3),
            _ => Err(format!("'{}' is not a valid value for AudioFileType", s)),
        }
    }
}
