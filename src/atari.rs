use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::env;
use tempdir;

pub use crate::bindings::root::{
	act,
	ale::{ALEInterface, ALEState},
	cloneState, cloneSystemState, decodeState, deleteState, encodeState, encodeStateLen, game_over,
	getAvailableDifficulties, getAvailableDifficultiesSize, getAvailableModes, getAvailableModesSize, getBool,
	getEpisodeFrameNumber, getFloat, getFrameNumber, getInt, getLegalActionSet, getLegalActionSize,
	getMinimalActionSet, getMinimalActionSize, getRAM, getRAMSize, getScreen, getScreenGrayscale, getScreenHeight,
	getScreenRGB, getScreenWidth, getString, lives, loadROM, loadState, reset_game, restoreState, restoreSystemState,
	saveScreenPNG, saveState, setBool, setDifficulty, setFloat, setInt, setLoggerMode, setMode, setString, ALE_del,
	ALE_new,
};

pub struct Atari {
	ale: *mut ALEInterface,
	action_set: Vec<i32>,
	screen_data: Vec<u8>,
	screen_size: (usize, usize),
	score: i32,
	lives: i32,
    frame_count: i32,
    max_frames: i32,
    gray_scale: bool,
}

unsafe impl Send for Atari {}


impl Atari {
    pub fn new(rom: BundledRom, max_frames: u32, gray_scale: bool, seed: Option<i32>) -> Atari {
        // save ROM to temp dir
		let dir = tempdir::TempDir::new("ale-rs").expect("Create temp dir failed");
		let des_path = dir.path().join(rom.filename());
        let src_path = std::env::current_dir()
            .expect("Cannot find project path")
            .join("roms").join(rom.filename());
        std::fs::copy(&src_path, &des_path).expect("Copy ROM to tempdir failed");


        let (ale, action_set, screen_size) = unsafe {
            setLoggerMode(LoggerMode::Error as c_int);

            // create ALEInterface
            let ale = ALE_new();
            assert!(ale != null_mut(), "Create ALEInterface failed");

			// set no action repeat
			let key = CString::new("repeat_action_probability").expect("Create Cstring key failed");
			setFloat(ale, key.as_ptr(), 0.0);

			// seed the game
			let key = CString::new("random_seed").expect("Create Cstring key failed");
			match seed {
				Some(v) => setInt(ale, key.as_ptr(), v),
				None => (),
			};

            // load ROM
            let rom_path_c_str = CString::new(des_path.to_str().unwrap()).expect("Invalid path");
            loadROM(ale, rom_path_c_str.as_ptr());
            
            // get minimal action set
            let action_dim = getMinimalActionSize(ale);
            let mut action_set = vec![0i32; action_dim as usize];
            getMinimalActionSet(ale, action_set.as_mut_ptr());
            let screen_size = (
                getScreenHeight(ale) as usize,
                getScreenWidth(ale) as usize
            );



            (ale, action_set, screen_size)
        };
        let score = 0;
        let lives = 0;
        let frame_count = 0;
        let screen_data = match gray_scale {
            true => vec![0u8; screen_size.0 * screen_size.1],
            false => vec![0u8; screen_size.0 * screen_size.1 * 3]
        };
        let max_frames = max_frames as i32;
		
		Atari{
            ale, 
            action_set, 
            screen_data, 
            screen_size, 
            score, 
            lives, 
            frame_count, 
            max_frames, 
            gray_scale
        }
    }


    pub fn reset(&mut self) {
        unsafe {
            reset_game(self.ale);
            self.lives = lives(self.ale);
        }
        self.score = 0;
        self.frame_count = 0;
    }

	// return step information: (reward, terminal, truncation, life loss)
    pub fn step(&mut self, action: usize) -> (i32, bool, bool, bool) {
        let action = *self.action_set.iter().nth(action).expect("Action out of range");
        let (reward, terminal, cur_lives) = unsafe {(
            act(self.ale, action),
            game_over(self.ale), 
            lives(self.ale))
        };
        self.frame_count += 1;
        let life_loss = cur_lives < self.lives;
        let truncation = self.frame_count >= self.max_frames;
        self.lives = cur_lives;
        self.score += reward;
        (reward, terminal, truncation, life_loss)
    }

    pub fn obs(&mut self) -> Vec<u8> {
        unsafe {
            match self.gray_scale {
                true => getScreenGrayscale(self.ale, self.screen_data.as_mut_ptr()),
                false => getScreenRGB(self.ale, self.screen_data.as_mut_ptr())
            }
        }
        self.screen_data.clone()
    }

    pub fn action_dim(&mut self) -> usize {
        self.action_set.len()
    }

	// return (height, width) of screen
    pub fn screen_dim(&mut self) -> (usize, usize) {
        self.screen_size
    }

	pub fn get_action_set(&mut self) -> Vec<i32> {
		self.action_set.clone()
	}

	pub fn close(&mut self) {
		unsafe {ALE_del(self.ale);}
	}
}


pub enum LoggerMode {
	Info = 0,
	Warning = 1,
	Error = 2,
}



pub enum BundledRom {
	Adventure,
	AirRaid,
	Alien,
	Amidar,
	Assault,
	Asterix,
	Asteroids,
	Atlantis,
	Atlantis2,
	Backgammon,
	BankHeist,
	BasicMath,
	BattleZone,
	BeamRider,
	Berzerk,
	Blackjack,
	Bowling,
	Boxing,
	Breakout,
	Carnival,
	Casino,
	Centipede,
	ChopperCommand,
	Combat,
	CrazyClimber,
	Crossbow,
	Darkchambers,
	Defender,
	DemonAttack,
	DonkeyKong,
	DoubleDunk,
	Earthworld,
	ElevatorAction,
	Enduro,
	Entombed,
	Et,
	FishingDerby,
	FlagCapture,
	Freeway,
	Frogger,
	Frostbite,
	Galaxian,
	Gopher,
	Gravitar,
	Hangman,
	HauntedHouse,
	Hero,
	HumanCannonball,
	IceHockey,
	Jamesbond,
	JourneyEscape,
	Joust,
	Kaboom,
	Kangaroo,
	KeystoneKapers,
	KingKong,
	Klax,
	Koolaid,
	Krull,
	KungFuMaster,
	LaserGates,
	LostLuggage,
	MarioBros,
	MazeCraze,
	MiniatureGolf,
	MontezumaRevenge,
	MrDo,
	MsPacman,
	NameThisGame,
	Othello,
	Pacman,
	Phoenix,
	Pitfall,
	Pitfall2,
	Pong,
	Pooyan,
	PrivateEye,
	Qbert,
	Riverraid,
	RoadRunner,
	Robotank,
	Seaquest,
	SirLancelot,
	Skiing,
	Solaris,
	SpaceInvaders,
	SpaceWar,
	StarGunner,
	Superman,
	Surround,
	Tennis,
	Tetris,
	TicTacToe3d,
	TimePilot,
	Trondead,
	Turmoil,
	Tutankham,
	UpNDown,
	Venture,
	VideoCheckers,
	VideoChess,
	VideoCube,
	VideoPinball,
	Warlords,
	WizardOfWor,
	WordZapper,
	YarsRevenge,
	Zaxxon,
}

impl BundledRom {
	/// Returns the filename that the ROM should be named, in order for the ALE to pick up on it and
	/// use the correct settings.
	pub fn filename(&self) -> &'static str {
		use BundledRom::*;
		match self {
			Adventure => "adventure.bin",
			AirRaid => "air_raid.bin",
			Alien => "alien.bin",
			Amidar => "amidar.bin",
			Assault => "assault.bin",
			Asterix => "asterix.bin",
			Asteroids => "asteroids.bin",
			Atlantis => "atlantis.bin",
			Atlantis2 => "atlantis2.bin",
			Backgammon => "backgammon.bin",
			BankHeist => "bank_heist.bin",
			BasicMath => "basic_math.bin",
			BattleZone => "battle_zone.bin",
			BeamRider => "beam_rider.bin",
			Berzerk => "berzerk.bin",
			Blackjack => "blackjack.bin",
			Bowling => "bowling.bin",
			Boxing => "boxing.bin",
			Breakout => "breakout.bin",
			Carnival => "carnival.bin",
			Casino => "casino.bin",
			Centipede => "centipede.bin",
			ChopperCommand => "chopper_command.bin",
			Combat => "combat.bin",
			CrazyClimber => "crazy_climber.bin",
			Crossbow => "crossbow.bin",
			Darkchambers => "darkchambers.bin",
			Defender => "defender.bin",
			DemonAttack => "demon_attack.bin",
			DonkeyKong => "donkey_kong.bin",
			DoubleDunk => "double_dunk.bin",
			Earthworld => "earthworld.bin",
			ElevatorAction => "elevator_action.bin",
			Enduro => "enduro.bin",
			Entombed => "entombed.bin",
			Et => "et.bin",
			FishingDerby => "fishing_derby.bin",
			FlagCapture => "flag_capture.bin",
			Freeway => "freeway.bin",
			Frogger => "frogger.bin",
			Frostbite => "frostbite.bin",
			Galaxian => "galaxian.bin",
			Gopher => "gopher.bin",
			Gravitar => "gravitar.bin",
			Hangman => "hangman.bin",
			HauntedHouse => "haunted_house.bin",
			Hero => "hero.bin",
			HumanCannonball => "human_cannonball.bin",
			IceHockey => "ice_hockey.bin",
			Jamesbond => "jamesbond.bin",
			JourneyEscape => "journey_escape.bin",
			Joust => "joust.bin",
			Kaboom => "kaboom.bin",
			Kangaroo => "kangaroo.bin",
			KeystoneKapers => "keystone_kapers.bin",
			KingKong => "king_kong.bin",
			Klax => "klax.bin",
			Koolaid => "koolaid.bin",
			Krull => "krull.bin",
			KungFuMaster => "kung_fu_master.bin",
			LaserGates => "laser_gates.bin",
			LostLuggage => "lost_luggage.bin",
			MarioBros => "mario_bros.bin",
			MazeCraze => "maze_craze.bin",
			MiniatureGolf => "miniature_golf.bin",
			MontezumaRevenge => "montezuma_revenge.bin",
			MrDo => "mr_do.bin",
			MsPacman => "ms_pacman.bin",
			NameThisGame => "name_this_game.bin",
			Othello => "othello.bin",
			Pacman => "pacman.bin",
			Phoenix => "phoenix.bin",
			Pitfall => "pitfall.bin",
			Pitfall2 => "pitfall2.bin",
			Pong => "pong.bin",
			Pooyan => "pooyan.bin",
			PrivateEye => "private_eye.bin",
			Qbert => "qbert.bin",
			Riverraid => "riverraid.bin",
			RoadRunner => "road_runner.bin",
			Robotank => "robotank.bin",
			Seaquest => "seaquest.bin",
			SirLancelot => "sir_lancelot.bin",
			Skiing => "skiing.bin",
			Solaris => "solaris.bin",
			SpaceInvaders => "space_invaders.bin",
			SpaceWar => "space_war.bin",
			StarGunner => "star_gunner.bin",
			Superman => "superman.bin",
			Surround => "surround.bin",
			Tennis => "tennis.bin",
			Tetris => "tetris.bin",
			TicTacToe3d => "tic_tac_toe_3d.bin",
			TimePilot => "time_pilot.bin",
			Trondead => "trondead.bin",
			Turmoil => "turmoil.bin",
			Tutankham => "tutankham.bin",
			UpNDown => "up_n_down.bin",
			Venture => "venture.bin",
			VideoCheckers => "video_checkers.bin",
			VideoChess => "video_chess.bin",
			VideoCube => "video_cube.bin",
			VideoPinball => "video_pinball.bin",
			Warlords => "warlords.bin",
			WizardOfWor => "wizard_of_wor.bin",
			WordZapper => "word_zapper.bin",
			YarsRevenge => "yars_revenge.bin",
			Zaxxon => "zaxxon.bin",
		}
	}
}