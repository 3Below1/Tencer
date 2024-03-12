#![allow(non_snake_case)]

use rocket_contrib::json::Json;
use rocket::response::content;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct MatchmakingParams {
    PreferFullRoomsFrequency: f32,
    PreferEmptyRoomsFrequency: f32,
}

#[derive(Serialize, Deserialize)]
struct LevelProgressionMap {
    Level: i32,
    RequiredXp: i32,
}
#[derive(Serialize, Deserialize)]
struct DailyObjective {
    // type should be one of the constants below
    r#type: i32,
    score: i32,
}

#[allow(dead_code, non_upper_case_globals)]
impl DailyObjective {
    const DEFAULT: i32 = -1;
    const FIRST_SESSION_OF_DAY: i32 = 1;
    const DAILY_OBJECTIVE_1: i32 = 10;
    const DAILY_OBJECTIVE_2: i32 = 11;
    const DAILY_OBJECTIVE_3: i32 = 12;
    const OOBE_GoToLockerRoom: i32 = 20;
    const OOBE_GoToActivity: i32 = 21;
    const OOBE_FinishActivity: i32 = 22;
    const CharadesGames: i32 = 100;
    const CharadesWinsPerformer: i32 = 101;
    const CharadesWinsGuesser: i32 = 102;
    const DiscGolfWins: i32 = 200;
    const DiscGolfGames: i32 = 201;
    const DiscGolfHolesUnderPar: i32 = 202;
    const DodgeballWins: i32 = 300;
    const DodgeballGames: i32 = 303;
    const DodgeballHits: i32 = 304;
    const PaddleballGames: i32 = 400;
    const PaddleballWins: i32 = 401;
    const PaddleballScores: i32 = 402;
    const PaintballAnyModeGames: i32 = 500;
    const PaintballAnyModeWins: i32 = 501;
    const PaintballAnyModeHits: i32 = 502;
    const PaintballCTFWins: i32 = 600;
    const PaintballCTFGames: i32 = 601;
    const PaintballCTFHits: i32 = 602;
    const PaintballFlagCaptures: i32 = 603;
    const PaintballTeamBattleWins: i32 = 700;
    const PaintballTeamBattleGames: i32 = 701;
    const PaintballTeamBattleHits: i32 = 702;
    const SoccerWins: i32 = 800;
    const SoccerGames: i32 = 801;
    const SoccerGoals: i32 = 802;
    const QuestGames: i32 = 1000;
    const QuestWins: i32 = 1001;
    const QuestPlayerRevives: i32 = 1002;
    const QuestEnemyKills: i32 = 1003;
    const PaintballFreeForAllWins: i32 = 1100;
    const PaintballFreeForAllGames: i32 = 1101;
    const PaintballFreeForAllHits: i32 = 1102;
}
#[derive(Serialize, Deserialize)]
struct KeyValuePair {
    Key: String,
    Value: String,
}
#[derive(Serialize, Deserialize)]
struct PhotonConfig {
    CrcCheckEnabled: bool,
    EnableServerTracingAfterDisconnect: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigV2 {
    MessageOfTheDay: String,
    CdnBaseUri: String,
    MatchmakingParams: MatchmakingParams, 
    LevelProgressionMaps: Vec<LevelProgressionMap>,
    DailyObjectives: Vec<Vec<DailyObjective>>,
    ConfigTable: Vec<KeyValuePair>,
    PhotonConfig: PhotonConfig,
}

#[get("/config/v2")]
pub fn v2() -> Json<ConfigV2> {
    Json(ConfigV2 {
        MessageOfTheDay: "Tencer custom server (by 567)".to_owned(),
        CdnBaseUri: "cdn".to_owned(),
        MatchmakingParams: MatchmakingParams { PreferFullRoomsFrequency: 0.5, PreferEmptyRoomsFrequency: 0.5 },
        LevelProgressionMaps: vec![LevelProgressionMap { Level: 2, RequiredXp: 999 }],
        DailyObjectives: vec![vec![DailyObjective {r#type: DailyObjective::PaintballFreeForAllWins, score: 100}]],
        ConfigTable: vec![],
        PhotonConfig: PhotonConfig {CrcCheckEnabled: true, EnableServerTracingAfterDisconnect: false},
    })
}

#[get("/config/v1/amplitude")]
pub fn v1_amplitude() -> content::Json<&'static str> {
    content::Json("{\"AmplitudeKey\": \"0\"}")
}