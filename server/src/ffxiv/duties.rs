use super::LocalisedText;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DutyInfo {
    pub name: LocalisedText,
    pub high_end: bool,
    pub content_kind: ContentKind,
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(u32)]
pub enum ContentKind {
    DutyRoulette = 1,
    Dungeons = 2,
    Guildhests = 3,
    Trials = 4,
    Raids = 5,
    PvP = 6,
    QuestBattles = 7,
    FATEs = 8,
    TreasureHunt = 9,
    Levequests = 10,
    GrandCompany = 11,
    Companions = 12,
    SocietyQuests = 13,
    OverallCompletion = 14,
    PlayerCommendation = 15,
    DisciplesoftheLand = 16,
    DisciplesoftheHand = 17,
    RetainerVentures = 18,
    GoldSaucer = 19,
    DeepDungeons = 21,
    WondrousTails = 24,
    CustomDeliveries = 25,
    Eureka = 26,
    TheMaskedCarnivale = 27,
    UltimateRaids = 28,
    SavetheQueen = 29,
    VCDungeonFinder = 30,
    OceanFishing = 31,
    TripleTriad = 32,
    TheHunt = 33,
    Fishing = 34,
    GATE = 35,
    IslandSanctuary = 36,
    ChaoticAllianceRaid = 37,
    Other(u32),
}

impl ContentKind {
    #[expect(unused)]
    fn from_u32(kind: u32) -> Self {
        match kind {
            1 => Self::DutyRoulette,
            2 => Self::Dungeons,
            3 => Self::Guildhests,
            4 => Self::Trials,
            5 => Self::Raids,
            6 => Self::PvP,
            7 => Self::QuestBattles,
            8 => Self::FATEs,
            9 => Self::TreasureHunt,
            10 => Self::Levequests,
            11 => Self::GrandCompany,
            12 => Self::Companions,
            13 => Self::SocietyQuests,
            14 => Self::OverallCompletion,
            15 => Self::PlayerCommendation,
            16 => Self::DisciplesoftheLand,
            17 => Self::DisciplesoftheHand,
            18 => Self::RetainerVentures,
            19 => Self::GoldSaucer,
            21 => Self::DeepDungeons,
            24 => Self::WondrousTails,
            25 => Self::CustomDeliveries,
            26 => Self::Eureka,
            27 => Self::TheMaskedCarnivale,
            28 => Self::UltimateRaids,
            29 => Self::SavetheQueen,
            30 => Self::VCDungeonFinder,
            31 => Self::OceanFishing,
            32 => Self::TripleTriad,
            33 => Self::TheHunt,
            34 => Self::Fishing,
            35 => Self::GATE,
            36 => Self::IslandSanctuary,
            37 => Self::ChaoticAllianceRaid,
            x => Self::Other(x),
        }
    }

    pub fn as_u32(self) -> u32 {
        match self {
            Self::DutyRoulette => 1,
            Self::Dungeons => 2,
            Self::Guildhests => 3,
            Self::Trials => 4,
            Self::Raids => 5,
            Self::PvP => 6,
            Self::QuestBattles => 7,
            Self::FATEs => 8,
            Self::TreasureHunt => 9,
            Self::Levequests => 10,
            Self::GrandCompany => 11,
            Self::Companions => 12,
            Self::SocietyQuests => 13,
            Self::OverallCompletion => 14,
            Self::PlayerCommendation => 15,
            Self::DisciplesoftheLand => 16,
            Self::DisciplesoftheHand => 17,
            Self::RetainerVentures => 18,
            Self::GoldSaucer => 19,
            Self::DeepDungeons => 21,
            Self::WondrousTails => 24,
            Self::CustomDeliveries => 25,
            Self::Eureka => 26,
            Self::TheMaskedCarnivale => 27,
            Self::UltimateRaids => 28,
            Self::SavetheQueen => 29,
            Self::VCDungeonFinder => 30,
            Self::OceanFishing => 31,
            Self::TripleTriad => 32,
            Self::TheHunt => 33,
            Self::Fishing => 34,
            Self::GATE => 35,
            Self::IslandSanctuary => 36,
            Self::ChaoticAllianceRaid => 37,
            Self::Other(x) => x,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DUTIES: HashMap<u32, DutyInfo> = maplit::hashmap! {
        1 => DutyInfo {
            name: LocalisedText {
                en: "The Thousand Maws of Toto-Rak",
                ja: "監獄廃墟 トトラクの千獄",
                de: "Tausend Löcher von Toto-Rak",
                fr: "Les Mille Gueules de Toto-Rak",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        2 => DutyInfo {
            name: LocalisedText {
                en: "The Tam-Tara Deepcroft",
                ja: "地下霊殿 タムタラの墓所",
                de: "Totenacker Tam-Tara",
                fr: "L'Hypogée de Tam-Tara",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        3 => DutyInfo {
            name: LocalisedText {
                en: "Copperbell Mines",
                ja: "封鎖坑道 カッパーベル銅山",
                de: "Kupferglocken-Mine",
                fr: "Les Mines de Clochecuivre",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        4 => DutyInfo {
            name: LocalisedText {
                en: "Sastasha",
                ja: "天然要害 サスタシャ浸食洞",
                de: "Sastasha",
                fr: "Sastasha",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        5 => DutyInfo {
            name: LocalisedText {
                en: "The Aurum Vale",
                ja: "霧中行軍 オーラムヴェイル",
                de: "Goldklamm",
                fr: "Le Val d'Aurum",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        6 => DutyInfo {
            name: LocalisedText {
                en: "Haukke Manor",
                ja: "名門屋敷 ハウケタ御用邸",
                de: "Haukke-Herrenhaus",
                fr: "Le Manoir des Haukke",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        7 => DutyInfo {
            name: LocalisedText {
                en: "Halatali",
                ja: "魔獣領域 ハラタリ修練所",
                de: "Halatali",
                fr: "Halatali",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        8 => DutyInfo {
            name: LocalisedText {
                en: "Brayflox's Longstop",
                ja: "奪還支援 ブレイフロクスの野営地",
                de: "Brüllvolx' Langrast",
                fr: "Le Bivouac de Brayflox",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        9 => DutyInfo {
            name: LocalisedText {
                en: "The Sunken Temple of Qarn",
                ja: "遺跡探索 カルン埋没寺院",
                de: "Versunkener Tempel von Qarn",
                fr: "Le Temple enseveli de Qarn",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        10 => DutyInfo {
            name: LocalisedText {
                en: "The Wanderer's Palace",
                ja: "旅神聖域 ワンダラーパレス",
                de: "Palast des Wanderers",
                fr: "Le Palais du Vagabond",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        11 => DutyInfo {
            name: LocalisedText {
                en: "The Stone Vigil",
                ja: "城塞攻略 ストーンヴィジル",
                de: "Steinerne Wacht",
                fr: "Le Vigile de Pierre",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        12 => DutyInfo {
            name: LocalisedText {
                en: "Cutter's Cry",
                ja: "流砂迷宮 カッターズクライ",
                de: "Sägerschrei",
                fr: "Le Gouffre hurlant",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        13 => DutyInfo {
            name: LocalisedText {
                en: "Dzemael Darkhold",
                ja: "掃討作戦 ゼーメル要塞",
                de: "Die Feste Dzemael",
                fr: "La Forteresse de Dzemael",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        14 => DutyInfo {
            name: LocalisedText {
                en: "Amdapor Keep",
                ja: "邪教排撃 古城アムダプール",
                de: "Die Ruinen von Amdapor",
                fr: "Le Château d'Amdapor",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        15 => DutyInfo {
            name: LocalisedText {
                en: "Castrum Meridianum",
                ja: "外郭攻略 カストルム・メリディアヌム",
                de: "Castrum Meridianum - Außenbereich",
                fr: "Castrum Meridianum",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        16 => DutyInfo {
            name: LocalisedText {
                en: "The Praetorium",
                ja: "最終決戦 魔導城プラエトリウム",
                de: "Castrum Meridianum - Praetorium",
                fr: "Le Praetorium",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        17 => DutyInfo {
            name: LocalisedText {
                en: "Pharos Sirius",
                ja: "怪鳥巨塔 シリウス大灯台",
                de: "Pharos Sirius",
                fr: "Le Phare de Sirius",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        18 => DutyInfo {
            name: LocalisedText {
                en: "Copperbell Mines (Hard)",
                ja: "騒乱坑道 カッパーベル銅山 (Hard)",
                de: "Kupferglocken-Mine (schwer)",
                fr: "Les Mines de Clochecuivre (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        19 => DutyInfo {
            name: LocalisedText {
                en: "Haukke Manor (Hard)",
                ja: "妖異屋敷 ハウケタ御用邸 (Hard)",
                de: "Haukke-Herrenhaus (schwer)",
                fr: "Le Manoir des Haukke (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        20 => DutyInfo {
            name: LocalisedText {
                en: "Brayflox's Longstop (Hard)",
                ja: "盟友支援 ブレイフロクスの野営地 (Hard)",
                de: "Brüllvolx' Langrast (schwer)",
                fr: "Le Bivouac de Brayflox (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        21 => DutyInfo {
            name: LocalisedText {
                en: "Halatali (Hard)",
                ja: "剣闘領域 ハラタリ修練所 (Hard)",
                de: "Halatali (schwer)",
                fr: "Halatali (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        22 => DutyInfo {
            name: LocalisedText {
                en: "The Lost City of Amdapor",
                ja: "腐敗遺跡 古アムダプール市街",
                de: "Historisches Amdapor",
                fr: "Les Vestiges de la cité d'Amdapor",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        23 => DutyInfo {
            name: LocalisedText {
                en: "Hullbreaker Isle",
                ja: "財宝伝説 ハルブレーカー・アイル",
                de: "Schiffbrecher-Insel",
                fr: "L'Île de Crèvecarène",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        24 => DutyInfo {
            name: LocalisedText {
                en: "The Tam-Tara Deepcroft (Hard)",
                ja: "惨劇霊殿 タムタラの墓所 (Hard)",
                de: "Totenacker Tam-Tara (schwer)",
                fr: "L'Hypogée de Tam-Tara (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        25 => DutyInfo {
            name: LocalisedText {
                en: "The Stone Vigil (Hard)",
                ja: "城塞奪回 ストーンヴィジル (Hard)",
                de: "Steinerne Wacht (schwer)",
                fr: "Le Vigile de Pierre (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        26 => DutyInfo {
            name: LocalisedText {
                en: "The Sunken Temple of Qarn (Hard)",
                ja: "遺跡救援 カルン埋没寺院 (Hard)",
                de: "Versunkener Tempel von Qarn (schwer)",
                fr: "Le Temple enseveli de Qarn (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        27 => DutyInfo {
            name: LocalisedText {
                en: "Snowcloak",
                ja: "氷結潜窟 スノークローク大氷壁",
                de: "Das Schneekleid",
                fr: "Manteneige",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        28 => DutyInfo {
            name: LocalisedText {
                en: "Sastasha (Hard)",
                ja: "逆襲要害 サスタシャ浸食洞 (Hard)",
                de: "Sastasha (schwer)",
                fr: "Sastasha (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        29 => DutyInfo {
            name: LocalisedText {
                en: "Amdapor Keep (Hard)",
                ja: "邪念排撃 古城アムダプール (Hard)",
                de: "Die Ruinen von Amdapor (schwer)",
                fr: "Le Château d'Amdapor (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        30 => DutyInfo {
            name: LocalisedText {
                en: "The Wanderer's Palace (Hard)",
                ja: "武装聖域 ワンダラーパレス (Hard)",
                de: "Palast des Wanderers (schwer)",
                fr: "Le Palais du Vagabond (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        31 => DutyInfo {
            name: LocalisedText {
                en: "The Great Gubal Library",
                ja: "禁書回収 グブラ幻想図書館",
                de: "Große Gubal-Bibliothek",
                fr: "La Grande bibliothèque de Gubal",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        32 => DutyInfo {
            name: LocalisedText {
                en: "The Keeper of the Lake",
                ja: "幻龍残骸 黙約の塔",
                de: "Hüter des Sees",
                fr: "Le Gardien du lac",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        33 => DutyInfo {
            name: LocalisedText {
                en: "Neverreap",
                ja: "神域浮島 ネバーリープ",
                de: "Nimmerreich",
                fr: "Nalloncques",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        34 => DutyInfo {
            name: LocalisedText {
                en: "The Vault",
                ja: "強硬突入 イシュガルド教皇庁",
                de: "Erzbasilika",
                fr: "La Voûte",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        35 => DutyInfo {
            name: LocalisedText {
                en: "The Fractal Continuum",
                ja: "博物戦艦 フラクタル・コンティニアム",
                de: "Die Fraktal-Kontinuum",
                fr: "Le Continuum fractal",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        36 => DutyInfo {
            name: LocalisedText {
                en: "The Dusk Vigil",
                ja: "廃砦捜索 ダスクヴィジル",
                de: "Abendrot-Wacht",
                fr: "Le Vigile du Crépuscule",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        37 => DutyInfo {
            name: LocalisedText {
                en: "Sohm Al",
                ja: "霊峰踏破 ソーム・アル",
                de: "Sohm Al",
                fr: "Sohm Al",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        38 => DutyInfo {
            name: LocalisedText {
                en: "The Aetherochemical Research Facility",
                ja: "蒼天聖戦 魔科学研究所",
                de: "Ätherochemisches Forschungslabor",
                fr: "Le Laboratoire de magismologie",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        39 => DutyInfo {
            name: LocalisedText {
                en: "The Aery",
                ja: "邪竜血戦 ドラゴンズエアリー",
                de: "Nest des Drachen",
                fr: "L'Aire",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        40 => DutyInfo {
            name: LocalisedText {
                en: "Pharos Sirius (Hard)",
                ja: "制圧巨塔 シリウス大灯台 (Hard)",
                de: "Pharos Sirius (schwer)",
                fr: "Le Phare de Sirius (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        41 => DutyInfo {
            name: LocalisedText {
                en: "Saint Mocianne's Arboretum",
                ja: "草木庭園 聖モシャーヌ植物園",
                de: "Sankt Mocianne-Arboretum",
                fr: "L'Arboretum Sainte-Mocianne",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        42 => DutyInfo {
            name: LocalisedText {
                en: "Basic Training: Enemy Parties",
                ja: "集団戦訓練をくぐり抜けろ！",
                de: "Einer für alle, alle für einen",
                fr: "Entraînement: groupes d'ennemis",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        43 => DutyInfo {
            name: LocalisedText {
                en: "Under the Armor",
                ja: "彷徨う死霊を討て！",
                de: "Bockmanns Gefolge",
                fr: "Chasse au fantôme fantoche",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        44 => DutyInfo {
            name: LocalisedText {
                en: "Basic Training: Enemy Strongholds",
                ja: "全関門を突破し、最深部の敵を討て！",
                de: "Sturmkommando",
                fr: "Entraînement: infiltration en base ennemie",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        45 => DutyInfo {
            name: LocalisedText {
                en: "Hero on the Half Shell",
                ja: "ギルガメを捕獲せよ！",
                de: "Gil oder Leben",
                fr: "Reconquête d'une carapace escamotée",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        46 => DutyInfo {
            name: LocalisedText {
                en: "Pulling Poison Posies",
                ja: "有毒妖花を駆除せよ！",
                de: "Unkraut jäten",
                fr: "Opération fleurs du mal",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        47 => DutyInfo {
            name: LocalisedText {
                en: "Stinging Back",
                ja: "無法者「似我蜂団」を撃滅せよ！",
                de: "Ins Wespennest stechen",
                fr: "Expédition punitive contre les Ventrerouge",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        48 => DutyInfo {
            name: LocalisedText {
                en: "All's Well that Ends in the Well",
                ja: "夢幻のブラキシオを討て！",
                de: "Briaxio ausschalten",
                fr: "Briaxio à bras raccourcis",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        49 => DutyInfo {
            name: LocalisedText {
                en: "Flicking Sticks and Taking Names",
                ja: "爆弾魔ゴブリン軍団を撃滅せよ！",
                de: "Bombige Goblins",
                fr: "Les Gobelins bombardiers",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        50 => DutyInfo {
            name: LocalisedText {
                en: "More than a Feeler",
                ja: "汚染源モルボルを討て！",
                de: "Tödliches Rankenspiel",
                fr: "Sus au morbol pollueur",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        51 => DutyInfo {
            name: LocalisedText {
                en: "Annoy the Void",
                ja: "坑道に現れた妖異ブソを討て！",
                de: "Gefahr aus dem Nichts",
                fr: "Buso l'immolateur",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        52 => DutyInfo {
            name: LocalisedText {
                en: "Shadow and Claw",
                ja: "無敵の眷属を従えし、大型妖異を討て！",
                de: "Kampf gegen Schatten",
                fr: "Ombres et griffes",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        53 => DutyInfo {
            name: LocalisedText {
                en: "Long Live the Queen",
                ja: "ボムを率いる「ボムクイーン」を討て！",
                de: "Miss Bombastic",
                fr: "Longue vie à la Reine",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        54 => DutyInfo {
            name: LocalisedText {
                en: "Ward Up",
                ja: "不気味な陣形を組む妖異をせん滅せよ！",
                de: "Unzertrennlich",
                fr: "Quintettes infernaux",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        55 => DutyInfo {
            name: LocalisedText {
                en: "Solemn Trinity",
                ja: "三つ巴の巨人族を制し、遺物を守れ！",
                de: "Wuchtige Dreifaltigkeit",
                fr: "Trinité sinistre",
            },
            high_end: false,
            content_kind: ContentKind::Guildhests,
        },
        56 => DutyInfo {
            name: LocalisedText {
                en: "The Bowl of Embers",
                ja: "イフリート討伐戦",
                de: "Das Grab der Lohe",
                fr: "Le Cratère des tisons",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        57 => DutyInfo {
            name: LocalisedText {
                en: "The Navel",
                ja: "タイタン討伐戦",
                de: "Der Nabel",
                fr: "Le Nombril",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        58 => DutyInfo {
            name: LocalisedText {
                en: "The Howling Eye",
                ja: "ガルーダ討伐戦",
                de: "Das Tosende Auge",
                fr: "Hurlœil",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        59 => DutyInfo {
            name: LocalisedText {
                en: "The Bowl of Embers (Hard)",
                ja: "真イフリート討滅戦",
                de: "Götterdämmerung - Ifrit",
                fr: "Le Cratère des tisons (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        60 => DutyInfo {
            name: LocalisedText {
                en: "The Navel (Hard)",
                ja: "真タイタン討滅戦",
                de: "Götterdämmerung - Titan",
                fr: "Le Nombril (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        61 => DutyInfo {
            name: LocalisedText {
                en: "The Howling Eye (Hard)",
                ja: "真ガルーダ討滅戦",
                de: "Götterdämmerung - Garuda",
                fr: "Hurlœil (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        63 => DutyInfo {
            name: LocalisedText {
                en: "The Bowl of Embers (Extreme)",
                ja: "極イフリート討滅戦",
                de: "Zenit der Götter - Ifrit",
                fr: "Le Cratère des tisons (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        64 => DutyInfo {
            name: LocalisedText {
                en: "The Navel (Extreme)",
                ja: "極タイタン討滅戦",
                de: "Zenit der Götter - Titan",
                fr: "Le Nombril (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        65 => DutyInfo {
            name: LocalisedText {
                en: "The Howling Eye (Extreme)",
                ja: "極ガルーダ討滅戦",
                de: "Zenit der Götter - Garuda",
                fr: "Hurlœil (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        66 => DutyInfo {
            name: LocalisedText {
                en: "Thornmarch (Hard)",
                ja: "善王モグル・モグXII世討滅戦",
                de: "Königliche Konfrontation (schwer)",
                fr: "La Lisière de ronces (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        67 => DutyInfo {
            name: LocalisedText {
                en: "Thornmarch (Extreme)",
                ja: "極王モグル・モグXII世討滅戦",
                de: "Königliche Konfrontation (extrem)",
                fr: "La Lisière de ronces (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        68 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Ultima's Bane",
                ja: "究極幻想 アルテマウェポン破壊作戦",
                de: "Heldenlied von Ultima",
                fr: "Le fléau d'Ultima",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        69 => DutyInfo {
            name: LocalisedText {
                en: "The Gilded Araya",
                ja: "アスラ討滅戦",
                de: "Prophetie - Asura",
                fr: "Le temple doré d'Araya",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        70 => DutyInfo {
            name: LocalisedText {
                en: "Special Event I",
                ja: "イベント用コンテンツ：1",
                de: "Event-Inhalt 1",
                fr: "Défi spécial I",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        71 => DutyInfo {
            name: LocalisedText {
                en: "Special Event II",
                ja: "イベント用コンテンツ：2",
                de: "Event-Inhalt 2",
                fr: "Défi spécial II",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        72 => DutyInfo {
            name: LocalisedText {
                en: "The Whorleater (Hard)",
                ja: "真リヴァイアサン討滅戦",
                de: "Götterdämmerung - Leviathan",
                fr: "Le Briseur de marées (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        73 => DutyInfo {
            name: LocalisedText {
                en: "The Whorleater (Extreme)",
                ja: "極リヴァイアサン討滅戦",
                de: "Zenit der Götter - Leviathan",
                fr: "Le Briseur de marées (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        74 => DutyInfo {
            name: LocalisedText {
                en: "A Relic Reborn: the Chimera",
                ja: "ドルムキマイラ討伐戦",
                de: "Kampf gegen die Dhorme-Chimära",
                fr: "La chimère dhorme du Coerthas",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        75 => DutyInfo {
            name: LocalisedText {
                en: "A Relic Reborn: the Hydra",
                ja: "ハイドラ討伐戦",
                de: "Kampf gegen die Hydra",
                fr: "L'hydre d'Halatali",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        76 => DutyInfo {
            name: LocalisedText {
                en: "Battle on the Big Bridge",
                ja: "ギルガメッシュ討伐戦",
                de: "Duell auf der großen Brücke",
                fr: "Affrontement sur le grand pont",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        77 => DutyInfo {
            name: LocalisedText {
                en: "The Striking Tree (Hard)",
                ja: "真ラムウ討滅戦",
                de: "Götterdämmerung - Ramuh",
                fr: "L'Arbre du jugement (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        78 => DutyInfo {
            name: LocalisedText {
                en: "The Striking Tree (Extreme)",
                ja: "極ラムウ討滅戦",
                de: "Zenit der Götter - Ramuh",
                fr: "L'Arbre du jugement (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        79 => DutyInfo {
            name: LocalisedText {
                en: "The Akh Afah Amphitheatre (Hard)",
                ja: "真シヴァ討滅戦",
                de: "Götterdämmerung - Shiva",
                fr: "L'Amphithéâtre d'Akh Afah (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        80 => DutyInfo {
            name: LocalisedText {
                en: "The Akh Afah Amphitheatre (Extreme)",
                ja: "極シヴァ討滅戦",
                de: "Zenit der Götter - Shiva",
                fr: "L'Amphithéâtre d'Akh Afah (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        81 => DutyInfo {
            name: LocalisedText {
                en: "The Dragon's Neck",
                ja: "アマジナ杯闘技会決勝戦",
                de: "Das Drachenhals-Kolosseum",
                fr: "Le Col du dragon",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        82 => DutyInfo {
            name: LocalisedText {
                en: "Urth's Fount",
                ja: "闘神オーディン討滅戦",
                de: "Jenseits Urths Quelle",
                fr: "La Fontaine d'Urth",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        84 => DutyInfo {
            name: LocalisedText {
                en: "The Chrysalis",
                ja: "アシエン・ナプリアレス討伐戦",
                de: "Chrysalis",
                fr: "La Chrysalide",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        85 => DutyInfo {
            name: LocalisedText {
                en: "Battle in the Big Keep",
                ja: "真ギルガメッシュ討滅戦",
                de: "Revanche in den Ruinen",
                fr: "Revanche au vieux château",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        86 => DutyInfo {
            name: LocalisedText {
                en: "Thok ast Thok (Hard)",
                ja: "真ラーヴァナ討滅戦",
                de: "Götterdämmerung - Ravana",
                fr: "Thok ast Thok (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        87 => DutyInfo {
            name: LocalisedText {
                en: "Thok ast Thok (Extreme)",
                ja: "極ラーヴァナ討滅戦",
                de: "Zenit der Götter - Ravana",
                fr: "Thok ast Thok (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        88 => DutyInfo {
            name: LocalisedText {
                en: "The Limitless Blue (Hard)",
                ja: "真ビスマルク討滅戦",
                de: "Götterdämmerung - Bismarck",
                fr: "L'Immensité bleue (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        89 => DutyInfo {
            name: LocalisedText {
                en: "The Limitless Blue (Extreme)",
                ja: "極ビスマルク討滅戦",
                de: "Zenit der Götter - Bismarck",
                fr: "L'Immensité bleue (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        90 => DutyInfo {
            name: LocalisedText {
                en: "The Singularity Reactor",
                ja: "ナイツ・オブ・ラウンド討滅戦",
                de: "Singularitäts-Reaktor",
                fr: "Le Réacteur de singularité",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        91 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Thordan's Reign",
                ja: "蒼天幻想 ナイツ・オブ・ラウンド討滅戦",
                de: "Heldenlied von Thordans Fall",
                fr: "Le règne de Thordan",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        92 => DutyInfo {
            name: LocalisedText {
                en: "The Labyrinth of the Ancients",
                ja: "クリスタルタワー：古代の民の迷宮",
                de: "Kristallturm - Das Labyrinth der Alten",
                fr: "La Tour de Cristal - Dédale antique",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        93 => DutyInfo {
            name: LocalisedText {
                en: "The Binding Coil of Bahamut - Turn 1",
                ja: "大迷宮バハムート：邂逅編1",
                de: "Verschlungene Schatten 1",
                fr: "Le Labyrinthe de Bahamut I",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        94 => DutyInfo {
            name: LocalisedText {
                en: "The Binding Coil of Bahamut - Turn 2",
                ja: "大迷宮バハムート：邂逅編2",
                de: "Verschlungene Schatten 2",
                fr: "Le Labyrinthe de Bahamut II",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        95 => DutyInfo {
            name: LocalisedText {
                en: "The Binding Coil of Bahamut - Turn 3",
                ja: "大迷宮バハムート：邂逅編3",
                de: "Verschlungene Schatten 3",
                fr: "Le Labyrinthe de Bahamut III",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        96 => DutyInfo {
            name: LocalisedText {
                en: "The Binding Coil of Bahamut - Turn 4",
                ja: "大迷宮バハムート：邂逅編4",
                de: "Verschlungene Schatten 4",
                fr: "Le Labyrinthe de Bahamut IV",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        97 => DutyInfo {
            name: LocalisedText {
                en: "The Binding Coil of Bahamut - Turn 5",
                ja: "大迷宮バハムート：邂逅編5",
                de: "Verschlungene Schatten 5",
                fr: "Le Labyrinthe de Bahamut V",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        98 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut - Turn 1",
                ja: "大迷宮バハムート：侵攻編1",
                de: "Verschlungene Schatten 2 - 1",
                fr: "Les Méandres de Bahamut I",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        99 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut - Turn 2",
                ja: "大迷宮バハムート：侵攻編2",
                de: "Verschlungene Schatten 2 - 2",
                fr: "Les Méandres de Bahamut II",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        100 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut - Turn 3",
                ja: "大迷宮バハムート：侵攻編3",
                de: "Verschlungene Schatten 2 - 3",
                fr: "Les Méandres de Bahamut III",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        101 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut - Turn 4",
                ja: "大迷宮バハムート：侵攻編4",
                de: "Verschlungene Schatten 2 - 4",
                fr: "Les Méandres de Bahamut IV",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        102 => DutyInfo {
            name: LocalisedText {
                en: "Syrcus Tower",
                ja: "クリスタルタワー：シルクスの塔",
                de: "Kristallturm - Der Syrcus-Turm",
                fr: "La Tour de Cristal - Tour de Syrcus",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        103 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut (Savage) - Turn 1",
                ja: "大迷宮バハムート零式：侵攻編1",
                de: "Verschlungene Schatten 2 - 1 (episch)",
                fr: "Les Méandres de Bahamut I (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        104 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut (Savage) - Turn 2",
                ja: "大迷宮バハムート零式：侵攻編2",
                de: "Verschlungene Schatten 2 - 2 (episch)",
                fr: "Les Méandres de Bahamut II (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        105 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut (Savage) - Turn 3",
                ja: "大迷宮バハムート零式：侵攻編3",
                de: "Verschlungene Schatten 2 - 3 (episch)",
                fr: "Les Méandres de Bahamut III (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        106 => DutyInfo {
            name: LocalisedText {
                en: "The Second Coil of Bahamut (Savage) - Turn 4",
                ja: "大迷宮バハムート零式：侵攻編4",
                de: "Verschlungene Schatten 2 - 4 (episch)",
                fr: "Les Méandres de Bahamut IV (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        107 => DutyInfo {
            name: LocalisedText {
                en: "The Final Coil of Bahamut - Turn 1",
                ja: "大迷宮バハムート：真成編1",
                de: "Verschlungene Schatten 3 - 1",
                fr: "L'Abîme de Bahamut I",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        108 => DutyInfo {
            name: LocalisedText {
                en: "The Final Coil of Bahamut - Turn 2",
                ja: "大迷宮バハムート：真成編2",
                de: "Verschlungene Schatten 3 - 2",
                fr: "L'Abîme de Bahamut II",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        109 => DutyInfo {
            name: LocalisedText {
                en: "The Final Coil of Bahamut - Turn 3",
                ja: "大迷宮バハムート：真成編3",
                de: "Verschlungene Schatten 3 - 3",
                fr: "L'Abîme de Bahamut III",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        110 => DutyInfo {
            name: LocalisedText {
                en: "The Final Coil of Bahamut - Turn 4",
                ja: "大迷宮バハムート：真成編4",
                de: "Verschlungene Schatten 3 - 4",
                fr: "L'Abîme de Bahamut IV",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        111 => DutyInfo {
            name: LocalisedText {
                en: "The World of Darkness",
                ja: "クリスタルタワー：闇の世界",
                de: "Die Welt der Dunkelheit",
                fr: "La Tour de Cristal - Monde des Ténèbres",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        112 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Fist of the Father",
                ja: "機工城アレキサンダー：起動編1",
                de: "Alexander - Faust des Vaters",
                fr: "Alexander - Le Poing du Père",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        113 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Cuff of the Father",
                ja: "機工城アレキサンダー：起動編2",
                de: "Alexander - Elle des Vaters",
                fr: "Alexander - Le Poignet du Père",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        114 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Arm of the Father",
                ja: "機工城アレキサンダー：起動編3",
                de: "Alexander - Arm des Vaters",
                fr: "Alexander - Le Bras du Père",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        115 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Burden of the Father",
                ja: "機工城アレキサンダー：起動編4",
                de: "Alexander - Last des Vaters",
                fr: "Alexander - Le Fardeau du Père",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        116 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Fist of the Father (Savage)",
                ja: "機工城アレキサンダー零式：起動編1",
                de: "Alexander - Faust des Vaters (episch)",
                fr: "Alexander - Le Poing du Père (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        117 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Cuff of the Father (Savage)",
                ja: "機工城アレキサンダー零式：起動編2",
                de: "Alexander - Elle des Vaters (episch)",
                fr: "Alexander - Le Poignet du Père (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        118 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Arm of the Father (Savage)",
                ja: "機工城アレキサンダー零式：起動編3",
                de: "Alexander - Arm des Vaters (episch)",
                fr: "Alexander - Le Bras du Père (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        119 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Burden of the Father (Savage)",
                ja: "機工城アレキサンダー零式：起動編4",
                de: "Alexander - Last des Vaters (episch)",
                fr: "Alexander - Le Fardeau du Père (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        120 => DutyInfo {
            name: LocalisedText {
                en: "The Void Ark",
                ja: "魔航船ヴォイドアーク",
                de: "Die Nichts-Arche",
                fr: "L'Arche du néant",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        127 => DutyInfo {
            name: LocalisedText {
                en: "The Borderland Ruins (Secure)",
                ja: "外縁遺跡群 (制圧戦)",
                de: "Äußere Ruinen (Sicherung)",
                fr: "Les Ruines frontalières (annexion)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        130 => DutyInfo {
            name: LocalisedText {
                en: "Seal Rock (Seize)",
                ja: "シールロック (争奪戦)",
                de: "Robbenholm (Eroberung)",
                fr: "Le Rocher des tréfonds (invasion)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        131 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem (Easy)",
                ja: "雲海探索 ディアデム諸島 (Easy)",
                de: "Das Diadem (leicht)",
                fr: "Le Diadème (facile)",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        132 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem",
                ja: "雲海探索 ディアデム諸島",
                de: "Das Diadem",
                fr: "Le Diadème",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        133 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem (Hard)",
                ja: "雲海探索 ディアデム諸島 (Hard)",
                de: "Das Diadem (schwer)",
                fr: "Le Diadème (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        134 => DutyInfo {
            name: LocalisedText {
                en: "Containment Bay S1T7",
                ja: "魔神セフィロト討滅戦",
                de: "Götterdämmerung - Sephirot",
                fr: "Unité de contention S1P7",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        135 => DutyInfo {
            name: LocalisedText {
                en: "Containment Bay S1T7 (Extreme)",
                ja: "極魔神セフィロト討滅戦",
                de: "Zenit der Götter - Sephirot",
                fr: "Unité de contention S1P7 (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        136 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Fist of the Son",
                ja: "機工城アレキサンダー：律動編1",
                de: "Alexander - Faust des Sohnes",
                fr: "Alexander - Le Poing du Fils",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        137 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Cuff of the Son",
                ja: "機工城アレキサンダー：律動編2",
                de: "Alexander - Elle des Sohnes",
                fr: "Alexander - Le Poignet du Fils",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        138 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Arm of the Son",
                ja: "機工城アレキサンダー：律動編3",
                de: "Alexander - Arm des Sohnes",
                fr: "Alexander - Le Bras du Fils",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        139 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Burden of the Son",
                ja: "機工城アレキサンダー：律動編4",
                de: "Alexander - Last des Sohnes",
                fr: "Alexander - Le Fardeau du Fils",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        140 => DutyInfo {
            name: LocalisedText {
                en: "The Lost City of Amdapor (Hard)",
                ja: "神聖遺跡 古アムダプール市街 (Hard)",
                de: "Historisches Amdapor (schwer)",
                fr: "Les Vestiges de la cité d'Amdapor (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        141 => DutyInfo {
            name: LocalisedText {
                en: "The Antitower",
                ja: "星海観測 逆さの塔",
                de: "Antiturm",
                fr: "L'Antitour",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        147 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Fist of the Son (Savage)",
                ja: "機工城アレキサンダー零式：律動編1",
                de: "Alexander - Faust des Sohnes (episch)",
                fr: "Alexander - Le Poing du Fils (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        148 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Cuff of the Son (Savage)",
                ja: "機工城アレキサンダー零式：律動編2",
                de: "Alexander - Elle des Sohnes (episch)",
                fr: "Alexander - Le Poignet du Fils (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        149 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Arm of the Son (Savage)",
                ja: "機工城アレキサンダー零式：律動編3",
                de: "Alexander - Arm des Sohnes (episch)",
                fr: "Alexander - Le Bras du Fils (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        150 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Burden of the Son (Savage)",
                ja: "機工城アレキサンダー零式：律動編4",
                de: "Alexander - Last des Sohnes (episch)",
                fr: "Alexander - Le Fardeau du Fils (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        151 => DutyInfo {
            name: LocalisedText {
                en: "Avoid Area of Effect Attacks",
                ja: "範囲攻撃を避けよう！",
                de: "Flächenangriffen ausweichen",
                fr: "Éviter les attaques à aire d'effet",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        152 => DutyInfo {
            name: LocalisedText {
                en: "Execute a Combo to Increase Enmity",
                ja: "コンボで敵視を集めよう！",
                de: "Mit Kombos Feindseligkeit auf sich ziehen",
                fr: "Générer de l'inimitié avec un combo",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        153 => DutyInfo {
            name: LocalisedText {
                en: "Execute a Combo in Battle",
                ja: "実戦でコンボに挑戦しよう！",
                de: "Kombos im Kampf einsetzen",
                fr: "Effectuer le combo en combat",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        154 => DutyInfo {
            name: LocalisedText {
                en: "Accrue Enmity from Multiple Targets",
                ja: "複数の敵から敵視を集めよう！",
                de: "Feindseligkeit mehrerer Gegner auf sich ziehen",
                fr: "Attirer l'inimitié de plusieurs ennemis sur soi",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        155 => DutyInfo {
            name: LocalisedText {
                en: "Engage Multiple Targets",
                ja: "実戦で複数の敵と戦ってみよう！",
                de: "Gegen mehrere Gegner auf einmal kämpfen",
                fr: "Affronter plusieurs ennemis",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        156 => DutyInfo {
            name: LocalisedText {
                en: "Execute a Ranged Attack to Increase Enmity",
                ja: "遠距離から敵視を集めよう！",
                de: "Aus der Ferne Feindseligkeit auf sich ziehen",
                fr: "Générer de l'inimitié à distance",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        157 => DutyInfo {
            name: LocalisedText {
                en: "Engage Enemy Reinforcements",
                ja: "敵の増援に対応しよう！",
                de: "Feindliche Verstärkung aufhalten",
                fr: "Faire face à des renforts ennemis",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        158 => DutyInfo {
            name: LocalisedText {
                en: "Assist Allies in Defeating a Target",
                ja: "味方と協力して敵を倒そう！",
                de: "Gegner gemeinsam besiegen",
                fr: "Vaincre un ennemi en assistant des alliés",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        159 => DutyInfo {
            name: LocalisedText {
                en: "Defeat an Occupied Target",
                ja: "味方が引きつけている敵を倒そう！",
                de: "Den Gegner eines Verbündeten besiegen",
                fr: "Vaincre un ennemi occupé par un allié",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        160 => DutyInfo {
            name: LocalisedText {
                en: "Avoid Engaged Targets",
                ja: "敵の攻撃を避けながら戦おう！",
                de: "Angriffen ausweichen",
                fr: "Combattre en évitant les attaques ennemies",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        161 => DutyInfo {
            name: LocalisedText {
                en: "Engage Enemy Reinforcements",
                ja: "敵の増援に対応しよう！",
                de: "Feindliche Verstärkung aufhalten",
                fr: "Éliminer les renforts ennemis",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        162 => DutyInfo {
            name: LocalisedText {
                en: "Interact with the Battlefield",
                ja: "ギミックを活用して戦おう！",
                de: "Mit dem Gelände interagieren",
                fr: "Interagir avec le décor en combat",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        163 => DutyInfo {
            name: LocalisedText {
                en: "Heal an Ally",
                ja: "味方を回復しよう！",
                de: "Verbündete heilen",
                fr: "Soigner un allié",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        164 => DutyInfo {
            name: LocalisedText {
                en: "Heal Multiple Allies",
                ja: "複数の味方を回復しよう！",
                de: "Mehrere Verbündete heilen",
                fr: "Soigner plusieurs alliés",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        165 => DutyInfo {
            name: LocalisedText {
                en: "Avoid Engaged Targets",
                ja: "敵の攻撃を避けながら戦おう！",
                de: "Angriffen ausweichen",
                fr: "Combattre en évitant les attaques ennemies",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        166 => DutyInfo {
            name: LocalisedText {
                en: "Final Exercise",
                ja: "最終訓練！",
                de: "Letzte Übung",
                fr: "Exercice final",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        167 => DutyInfo {
            name: LocalisedText {
                en: "A Spectacle for the Ages",
                ja: "四国合同演習",
                de: "Truppenübung der Eorzäischen Allianz",
                fr: "La grande manœuvre éorzéenne",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        168 => DutyInfo {
            name: LocalisedText {
                en: "The Weeping City of Mhach",
                ja: "禁忌都市マハ",
                de: "Die Stadt der Tränen",
                fr: "La Cité défendue de Mhach",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        169 => DutyInfo {
            name: LocalisedText {
                en: "The Final Steps of Faith",
                ja: "ニーズヘッグ征竜戦",
                de: "Der letzte Schicksalsweg",
                fr: "La Dernière avancée de la Foi",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        170 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Nidhogg's Rage",
                ja: "極ニーズヘッグ征竜戦",
                de: "Das Lied von Nidhoggs letztem Ruf",
                fr: "L'ire de Nidhogg",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        171 => DutyInfo {
            name: LocalisedText {
                en: "Sohr Khai",
                ja: "天竜宮殿 ソール・カイ",
                de: "Sohr Khai",
                fr: "Sohr Khai",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        172 => DutyInfo {
            name: LocalisedText {
                en: "Hullbreaker Isle (Hard)",
                ja: "黒渦伝説 ハルブレーカー・アイル (Hard)",
                de: "Schiffbrecher-Insel (schwer)",
                fr: "L'Île de Crèvecarène (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        173 => DutyInfo {
            name: LocalisedText {
                en: "A Bloody Reunion",
                ja: "レグラ・ヴァン・ヒュドルス追撃戦",
                de: "Blutiges Wiedersehen",
                fr: "Course-poursuite dans le laboratoire",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        174 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 1-10)",
                ja: "死者の宮殿 B1～B10",
                de: "Palast der Toten (Ebenen 1-10)",
                fr: "Le Palais des morts (sous-sols 1-10)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        175 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 11-20)",
                ja: "死者の宮殿 B11～B20",
                de: "Palast der Toten (Ebenen 11-20)",
                fr: "Le Palais des morts (sous-sols 11-20)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        176 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 21-30)",
                ja: "死者の宮殿 B21～B30",
                de: "Palast der Toten (Ebenen 21-30)",
                fr: "Le Palais des morts (sous-sols 21-30)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        177 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 31-40)",
                ja: "死者の宮殿 B31～B40",
                de: "Palast der Toten (Ebenen 31-40)",
                fr: "Le Palais des morts (sous-sols 31-40)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        178 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 41-50)",
                ja: "死者の宮殿 B41～B50",
                de: "Palast der Toten (Ebenen 41-50)",
                fr: "Le Palais des morts (sous-sols 41-50)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        179 => DutyInfo {
            name: LocalisedText {
                en: "The Aquapolis",
                ja: "宝物庫 アクアポリス",
                de: "Aquapolis",
                fr: "L'Aquapole",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        180 => DutyInfo {
            name: LocalisedText {
                en: "The Fields of Glory (Shatter)",
                ja: "フィールド・オブ・グローリー (砕氷戦)",
                de: "Feld der Ehre (Zersplitterung)",
                fr: "Les Champs de la Gloire (brise-glace)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        181 => DutyInfo {
            name: LocalisedText {
                en: "The Haunted Manor",
                ja: "亡霊屋敷 ホーンテッドマナー",
                de: "Das Geisterschloss",
                fr: "Le Manoir hanté",
            },
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        182 => DutyInfo {
            name: LocalisedText {
                en: "Xelphatol",
                ja: "峻厳渓谷 ゼルファトル",
                de: "Xelphatol",
                fr: "Xelphatol",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        183 => DutyInfo {
            name: LocalisedText {
                en: "Containment Bay P1T6",
                ja: "女神ソフィア討滅戦",
                de: "Götterdämmerung - Sophia",
                fr: "Unité de contention P1P6",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        184 => DutyInfo {
            name: LocalisedText {
                en: "Containment Bay P1T6 (Extreme)",
                ja: "極女神ソフィア討滅戦",
                de: "Zenit der Götter - Sophia",
                fr: "Unité de contention P1P6 (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        186 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Eyes of the Creator",
                ja: "機工城アレキサンダー：天動編1",
                de: "Alexander - Augen des Schöpfers",
                fr: "Alexander - Les Yeux du Créateur",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        187 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Breath of the Creator",
                ja: "機工城アレキサンダー：天動編2",
                de: "Alexander - Atem des Schöpfers",
                fr: "Alexander - Le Souffle du Créateur",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        188 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Heart of the Creator",
                ja: "機工城アレキサンダー：天動編3",
                de: "Alexander - Herz des Schöpfers",
                fr: "Alexander - Le Cœur du Créateur",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        189 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Soul of the Creator",
                ja: "機工城アレキサンダー：天動編4",
                de: "Alexander - Seele des Schöpfers",
                fr: "Alexander - L'Âme du Créateur",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        190 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Eyes of the Creator (Savage)",
                ja: "機工城アレキサンダー零式：天動編1",
                de: "Alexander - Augen des Schöpfers (episch)",
                fr: "Alexander - Les Yeux du Créateur (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        191 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Breath of the Creator (Savage)",
                ja: "機工城アレキサンダー零式：天動編2",
                de: "Alexander - Atem des Schöpfers (episch)",
                fr: "Alexander - Le Souffle du Créateur (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        192 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Heart of the Creator (Savage)",
                ja: "機工城アレキサンダー零式：天動編3",
                de: "Alexander - Herz des Schöpfers (episch)",
                fr: "Alexander - Le Cœur du Créateur (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        193 => DutyInfo {
            name: LocalisedText {
                en: "Alexander - The Soul of the Creator (Savage)",
                ja: "機工城アレキサンダー零式：天動編4",
                de: "Alexander - Seele des Schöpfers (episch)",
                fr: "Alexander - L'Âme du Créateur (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        194 => DutyInfo {
            name: LocalisedText {
                en: "One Life for One World",
                ja: "絡み合う宿命",
                de: "Weltenübergreifendes Schicksal",
                fr: "Destins entrecroisés",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        195 => DutyInfo {
            name: LocalisedText {
                en: "The Triple Triad Battlehall",
                ja: "トリプルトライアド：カードバトルルーム",
                de: "Triple Triad: Weltensalon",
                fr: "Arène Triple Triade",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        196 => DutyInfo {
            name: LocalisedText {
                en: "The Great Gubal Library (Hard)",
                ja: "稀書回収 グブラ幻想図書館 (Hard)",
                de: "Große Gubal-Bibliothek (schwer)",
                fr: "La Grande bibliothèque de Gubal (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        197 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Player Battle (RP)",
                ja: "LoVM：プレイヤー対戦 (RP変動あり)",
                de: "Kampf der Trabanten: Gegen Spieler (um RP)",
                fr: "Bataille simple contre un joueur (avec PR)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        198 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Tournament",
                ja: "LoVM：大会対戦 (プレイヤー対戦）",
                de: "Kampf der Trabanten: Turnier (gegen Spieler)",
                fr: "Bataille de tournoi contre des joueurs",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        199 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Player Battle (Non-RP)",
                ja: "LoVM：プレイヤー対戦 (RP変動なし)",
                de: "Kampf der Trabanten: Gegen Spieler (ohne RP)",
                fr: "Bataille simple contre un joueur (sans PR)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        202 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem Hunting Grounds (Easy)",
                ja: "雲海探索 ディアデム諸島：狩猟限定 (Easy)",
                de: "Das Diadem - Jagdgründe (leicht)",
                fr: "Le Diadème: terrains de chasse (facile)",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        203 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem Hunting Grounds",
                ja: "雲海探索 ディアデム諸島：狩猟限定",
                de: "Das Diadem - Jagdgründe",
                fr: "Le Diadème: terrains de chasse",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        204 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 51-60)",
                ja: "死者の宮殿 B51～B60",
                de: "Palast der Toten (Ebenen 51 - 60)",
                fr: "Le Palais des morts (sous-sols 51-60)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        205 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 61-70)",
                ja: "死者の宮殿 B61～B70",
                de: "Palast der Toten (Ebenen 61 - 70)",
                fr: "Le Palais des morts (sous-sols 61-70)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        206 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 71-80)",
                ja: "死者の宮殿 B71～B80",
                de: "Palast der Toten (Ebenen 71 - 80)",
                fr: "Le Palais des morts (sous-sols 71-80)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        207 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 81-90)",
                ja: "死者の宮殿 B81～B90",
                de: "Palast der Toten (Ebenen 81 - 90)",
                fr: "Le Palais des morts (sous-sols 81-90)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        208 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 91-100)",
                ja: "死者の宮殿 B91～B100",
                de: "Palast der Toten (Ebenen 91 - 100)",
                fr: "Le Palais des morts (sous-sols 91-100)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        209 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 101-110)",
                ja: "死者の宮殿 B101～B110",
                de: "Palast der Toten (Ebenen 101 - 110)",
                fr: "Le Palais des morts (sous-sols 101-110)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        210 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 111-120)",
                ja: "死者の宮殿 B111～B120",
                de: "Palast der Toten (Ebenen 111 - 120)",
                fr: "Le Palais des morts (sous-sols 111-120)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        211 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 121-130)",
                ja: "死者の宮殿 B121～B130",
                de: "Palast der Toten (Ebenen 121 - 130)",
                fr: "Le Palais des morts (sous-sols 121-130)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        212 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 131-140)",
                ja: "死者の宮殿 B131～B140",
                de: "Palast der Toten (Ebenen 131 - 140)",
                fr: "Le Palais des morts (sous-sols 131-140)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        213 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 141-150)",
                ja: "死者の宮殿 B141～B150",
                de: "Palast der Toten (Ebenen 141 - 150)",
                fr: "Le Palais des morts (sous-sols 141-150)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        214 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 151-160)",
                ja: "死者の宮殿 B151～B160",
                de: "Palast der Toten (Ebenen 151 - 160)",
                fr: "Le Palais des morts (sous-sols 151-160)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        215 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 161-170)",
                ja: "死者の宮殿 B161～B170",
                de: "Palast der Toten (Ebenen 161 - 170)",
                fr: "Le Palais des morts (sous-sols 161-170)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        216 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 171-180)",
                ja: "死者の宮殿 B171～B180",
                de: "Palast der Toten (Ebenen 171 - 180)",
                fr: "Le Palais des morts (sous-sols 171-180)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        217 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 181-190)",
                ja: "死者の宮殿 B181～B190",
                de: "Palast der Toten (Ebenen 181 - 190)",
                fr: "Le Palais des morts (sous-sols 181-190)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        218 => DutyInfo {
            name: LocalisedText {
                en: "The Palace of the Dead (Floors 191-200)",
                ja: "死者の宮殿 B191～B200",
                de: "Palast der Toten (Ebenen 191 - 200)",
                fr: "Le Palais des morts (sous-sols 191-200)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        219 => DutyInfo {
            name: LocalisedText {
                en: "Baelsar's Wall",
                ja: "巨大防壁 バエサルの長城",
                de: "Baelsar-Wall",
                fr: "La Muraille de Baelsar",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        220 => DutyInfo {
            name: LocalisedText {
                en: "Dun Scaith",
                ja: "影の国ダン・スカー",
                de: "Dun Scaith",
                fr: "Dun Scaith",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        221 => DutyInfo {
            name: LocalisedText {
                en: "Sohm Al (Hard)",
                ja: "霊峰浄化 ソーム・アル (Hard)",
                de: "Sohm Al (schwer)",
                fr: "Sohm Al (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        222 => DutyInfo {
            name: LocalisedText {
                en: "The Carteneau Flats: Heliodrome",
                ja: "カルテノー平原遭遇戦",
                de: "Heliodrom",
                fr: "Rixe à l'Héliodrome",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        223 => DutyInfo {
            name: LocalisedText {
                en: "Containment Bay Z1T9",
                ja: "鬼神ズルワーン討滅戦",
                de: "Götterdämmerung - Zurvan",
                fr: "Unité de contention Z1P9",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        224 => DutyInfo {
            name: LocalisedText {
                en: "Containment Bay Z1T9 (Extreme)",
                ja: "極鬼神ズルワーン討滅戦",
                de: "Zenit der Götter - Zurvan",
                fr: "Unité de contention Z1P9 (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        225 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem - Trials of the Fury",
                ja: "雲海探索 ディアデム諸島 (狩猟)",
                de: "Das Diadem - Halones Prüfung",
                fr: "Le Diadème - Épreuves de Halone",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        234 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem - Trials of the Matron",
                ja: "雲海探索 ディアデム諸島 (採集)",
                de: "Das Diadem - Nophicas Prüfung",
                fr: "Le Diadème - Épreuves de Nophica",
            },
            high_end: false,
            content_kind: ContentKind::Other(23),
        },
        235 => DutyInfo {
            name: LocalisedText {
                en: "Shisui of the Violet Tides",
                ja: "海底宮殿 紫水宮",
                de: "Shisui",
                fr: "Le Palais aux Marées violettes",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        236 => DutyInfo {
            name: LocalisedText {
                en: "The Temple of the Fist",
                ja: "壊神修行 星導山寺院",
                de: "Tempel der Faust",
                fr: "Le Temple du Poing",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        237 => DutyInfo {
            name: LocalisedText {
                en: "It's Probably a Trap",
                ja: "ギョドウ現る！",
                de: "Ein zweifelhaftes Angebot",
                fr: "Un drôle de Namazu",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        238 => DutyInfo {
            name: LocalisedText {
                en: "The Sirensong Sea",
                ja: "漂流海域 セイレーン海",
                de: "Sirenen-See",
                fr: "La Mer du Chant des sirènes",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        239 => DutyInfo {
            name: LocalisedText {
                en: "The Royal Menagerie",
                ja: "神龍討滅戦",
                de: "Königliche Menagerie",
                fr: "La Ménagerie royale",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        240 => DutyInfo {
            name: LocalisedText {
                en: "Bardam's Mettle",
                ja: "伝統試練 バルダム覇道",
                de: "Bardams Probe",
                fr: "La Force de Bardam",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        241 => DutyInfo {
            name: LocalisedText {
                en: "Doma Castle",
                ja: "解放決戦 ドマ城",
                de: "Burg Doma",
                fr: "Le Château de Doma",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        242 => DutyInfo {
            name: LocalisedText {
                en: "Castrum Abania",
                ja: "巨砲要塞 カストルム・アバニア",
                de: "Castrum Abania",
                fr: "Castrum Abania",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        243 => DutyInfo {
            name: LocalisedText {
                en: "The Pool of Tribute",
                ja: "スサノオ討滅戦",
                de: "Götterdämmerung - Susano",
                fr: "La Crique aux tributs",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        244 => DutyInfo {
            name: LocalisedText {
                en: "The Pool of Tribute (Extreme)",
                ja: "極スサノオ討滅戦",
                de: "Zenit der Götter - Susano",
                fr: "La Crique aux tributs (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        245 => DutyInfo {
            name: LocalisedText {
                en: "With Heart and Steel",
                ja: "抗う力",
                de: "Die Kraft des Widerstands",
                fr: "Transmigration démoniaque",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        246 => DutyInfo {
            name: LocalisedText {
                en: "Naadam",
                ja: "終節の合戦",
                de: "Naadam",
                fr: "La grande bataille du Naadam",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        247 => DutyInfo {
            name: LocalisedText {
                en: "Ala Mhigo",
                ja: "紅蓮決戦 アラミゴ",
                de: "Ala Mhigo",
                fr: "Ala Mhigo",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        248 => DutyInfo {
            name: LocalisedText {
                en: "Blood on the Deck",
                ja: "海都を震わす人斬りの宴！",
                de: "Mord ist sein Hobby",
                fr: "La légende de Musosai: l'assassin de Limsa Lominsa",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        249 => DutyInfo {
            name: LocalisedText {
                en: "The Face of True Evil",
                ja: "極悪人コガラシ",
                de: "Der Inbegriff des Bösen",
                fr: "L'abominable Kogarashi",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        250 => DutyInfo {
            name: LocalisedText {
                en: "Matsuba Mayhem",
                ja: "松葉門外の変",
                de: "Vorfall auf dem Matsuba-Platz",
                fr: "Règlement de compte au square Matsuba",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        251 => DutyInfo {
            name: LocalisedText {
                en: "The Battle on Bekko",
                ja: "ベッコウ島の決闘",
                de: "Entscheidungsschlacht auf Bekko",
                fr: "L'affrontement de deux justices",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        252 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V1.0",
                ja: "次元の狭間オメガ：デルタ編1",
                de: "Deltametrie 1.0",
                fr: "Deltastice v1.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        253 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V2.0",
                ja: "次元の狭間オメガ：デルタ編2",
                de: "Deltametrie 2.0",
                fr: "Deltastice v2.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        254 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V3.0",
                ja: "次元の狭間オメガ：デルタ編3",
                de: "Deltametrie 3.0",
                fr: "Deltastice v3.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        255 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V4.0",
                ja: "次元の狭間オメガ：デルタ編4",
                de: "Deltametrie 4.0",
                fr: "Deltastice v4.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        256 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V1.0 (Savage)",
                ja: "次元の狭間オメガ零式：デルタ編1",
                de: "Deltametrie 1.0 (episch)",
                fr: "Deltastice v1.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        257 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V2.0 (Savage)",
                ja: "次元の狭間オメガ零式：デルタ編2",
                de: "Deltametrie 2.0 (episch)",
                fr: "Deltastice v2.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        258 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V3.0 (Savage)",
                ja: "次元の狭間オメガ零式：デルタ編3",
                de: "Deltametrie 3.0 (episch)",
                fr: "Deltastice v3.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        259 => DutyInfo {
            name: LocalisedText {
                en: "Deltascape V4.0 (Savage)",
                ja: "次元の狭間オメガ零式：デルタ編4",
                de: "Deltametrie 4.0 (episch)",
                fr: "Deltastice v4.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        260 => DutyInfo {
            name: LocalisedText {
                en: "Curious Gorge Meets His Match",
                ja: "原初的な彼女",
                de: "Die Urkraft in ihr",
                fr: "L'épreuve de force",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        261 => DutyInfo {
            name: LocalisedText {
                en: "In Thal's Name",
                ja: "ウル王杯闘技会の始まり",
                de: "Thal zu Ehren",
                fr: "Le tournoi commémoratif du sultanat",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        262 => DutyInfo {
            name: LocalisedText {
                en: "Kugane Castle",
                ja: "悪党成敗 クガネ城",
                de: "Schloss Kugane",
                fr: "Le Château de Kugane",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        263 => DutyInfo {
            name: LocalisedText {
                en: "Emanation",
                ja: "ラクシュミ討滅戦",
                de: "Götterdämmerung - Lakshmi",
                fr: "Émanation",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        264 => DutyInfo {
            name: LocalisedText {
                en: "Emanation (Extreme)",
                ja: "極ラクシュミ討滅戦",
                de: "Zenit der Götter - Lakshmi",
                fr: "Émanation (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        265 => DutyInfo {
            name: LocalisedText {
                en: "Our Unsung Heroes",
                ja: "時をかける願い",
                de: "Ein Wunsch aus alten Zeiten",
                fr: "L'espoir en héritage",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        266 => DutyInfo {
            name: LocalisedText {
                en: "The Heart of the Problem",
                ja: "燃えよゴージ！",
                de: "Kriegerische Leidenschaft",
                fr: "Passion guerrière",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        267 => DutyInfo {
            name: LocalisedText {
                en: "Dark as the Night Sky",
                ja: "漆黒の巨竜",
                de: "Der tobende Drache",
                fr: "Aussi sombre que la nuit",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        268 => DutyInfo {
            name: LocalisedText {
                en: "The Lost Canals of Uznair",
                ja: "宝物庫 ウズネアカナル",
                de: "Kanäle von Uznair",
                fr: "Les Canaux perdus d'Uznair",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        269 => DutyInfo {
            name: LocalisedText {
                en: "The Resonant",
                ja: "ウリエンジェの秘策",
                de: "Wege zur Transzendenz",
                fr: "La ruse d'Urianger",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        270 => DutyInfo {
            name: LocalisedText {
                en: "Raising the Sword",
                ja: "さらなる剣術の高みへ",
                de: "Die hohe Kunst des Schwertkampfs",
                fr: "La finale des champions",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        271 => DutyInfo {
            name: LocalisedText {
                en: "The Orphans and the Broken Blade",
                ja: "あと三度、遥かな憧憬",
                de: "Probe des Meisters",
                fr: "L'aspiration refoulée",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        272 => DutyInfo {
            name: LocalisedText {
                en: "Our Compromise",
                ja: "あと一度、君に会えたら",
                de: "Aus der Tiefe des Herzens",
                fr: "La dernière séparation",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        273 => DutyInfo {
            name: LocalisedText {
                en: "Dragon Sound",
                ja: "紅の竜騎士",
                de: "Der Rubin-Drachenreiter",
                fr: "Le Dragon écarlate",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        274 => DutyInfo {
            name: LocalisedText {
                en: "When Clans Collide",
                ja: "影隠忍法帖",
                de: "Aus dem Verborgenen",
                fr: "La bataille des clans",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        275 => DutyInfo {
            name: LocalisedText {
                en: "Interdimensional Rift",
                ja: "次元の狭間：外縁",
                de: "Interdimensionaler Riss",
                fr: "Fissure interdimensionnelle",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        276 => DutyInfo {
            name: LocalisedText {
                en: "The Hidden Canals of Uznair",
                ja: "宝物庫 ウズネアカナル深層",
                de: "Vergessene Kanäle von Uznair",
                fr: "Les Canaux cachés d'Uznair",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        277 => DutyInfo {
            name: LocalisedText {
                en: "Astragalos",
                ja: "アストラガロス (機工戦)",
                de: "Astragalos",
                fr: "Astragalos (machinerie)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        278 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Shinryu's Domain",
                ja: "極神龍討滅戦",
                de: "Heldenlied von Shinryu",
                fr: "Le domaine de Shinryu",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        279 => DutyInfo {
            name: LocalisedText {
                en: "The Drowned City of Skalla",
                ja: "水没遺構 スカラ",
                de: "Die versunkene Stadt Skalla",
                fr: "La Cité engloutie de Skalla",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        280 => DutyInfo {
            name: LocalisedText {
                en: "The Unending Coil of Bahamut (Ultimate)",
                ja: "絶バハムート討滅戦",
                de: "Endlose Schatten von Bahamut (fatal)",
                fr: "L'Abîme infini de Bahamut (fatal)",
            },
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        281 => DutyInfo {
            name: LocalisedText {
                en: "The Royal City of Rabanastre",
                ja: "失われた都 ラバナスタ",
                de: "Rabanastre",
                fr: "La Cité royale de Rabanastre",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        282 => DutyInfo {
            name: LocalisedText {
                en: "Return of the Bull",
                ja: "英雄の帰還",
                de: "Verrat der Qalyana",
                fr: "Retour au bercail",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        283 => DutyInfo {
            name: LocalisedText {
                en: "The Forbidden Land, Eureka Anemos",
                ja: "禁断の地 エウレカ：アネモス編",
                de: "Eureka Anemos",
                fr: "Eurêka Anemos",
            },
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        284 => DutyInfo {
            name: LocalisedText {
                en: "Hells' Lid",
                ja: "紅玉火山 獄之蓋",
                de: "Höllenspund",
                fr: "Le Couvercle des enfers",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        285 => DutyInfo {
            name: LocalisedText {
                en: "The Fractal Continuum (Hard)",
                ja: "暴走戦艦 フラクタル・コンティニアム (Hard)",
                de: "Die Fraktal-Kontinuum (schwer)",
                fr: "Le Continuum fractal (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        286 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V1.0",
                ja: "次元の狭間オメガ：シグマ編1",
                de: "Sigmametrie 1.0",
                fr: "Sigmastice v1.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        287 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V2.0",
                ja: "次元の狭間オメガ：シグマ編2",
                de: "Sigmametrie 2.0",
                fr: "Sigmastice v2.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        288 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V3.0",
                ja: "次元の狭間オメガ：シグマ編3",
                de: "Sigmametrie 3.0",
                fr: "Sigmastice v3.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        289 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V4.0",
                ja: "次元の狭間オメガ：シグマ編4",
                de: "Sigmametrie 4.0",
                fr: "Sigmastice v4.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        290 => DutyInfo {
            name: LocalisedText {
                en: "The Jade Stoa",
                ja: "白虎征魂戦",
                de: "Seelentanz - Byakko",
                fr: "La Clairière de Jade",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        291 => DutyInfo {
            name: LocalisedText {
                en: "The Jade Stoa (Extreme)",
                ja: "極白虎征魂戦",
                de: "Seelensturm - Byakko",
                fr: "La Clairière de Jade (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        292 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V1.0 (Savage)",
                ja: "次元の狭間オメガ零式：シグマ編1",
                de: "Sigmametrie 1.0 (episch)",
                fr: "Sigmastice v1.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        293 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V2.0 (Savage)",
                ja: "次元の狭間オメガ零式：シグマ編2",
                de: "Sigmametrie 2.0 (episch)",
                fr: "Sigmastice v2.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        294 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V3.0 (Savage)",
                ja: "次元の狭間オメガ零式：シグマ編3",
                de: "Sigmametrie 3.0 (episch)",
                fr: "Sigmastice v3.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        295 => DutyInfo {
            name: LocalisedText {
                en: "Sigmascape V4.0 (Savage)",
                ja: "次元の狭間オメガ零式：シグマ編4",
                de: "Sigmametrie 4.0 (episch)",
                fr: "Sigmastice v4.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        473 => DutyInfo {
            name: LocalisedText {
                en: "The Valentione's Ceremony",
                ja: "ヴァレンティオンセレモニー",
                de: "Valentionzeremonie",
                fr: "La Cérémonie de la Valention",
            },
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        474 => DutyInfo {
            name: LocalisedText {
                en: "The Great Hunt",
                ja: "リオレウス狩猟戦",
                de: "Jagd auf Rathalos",
                fr: "Chasse au Rathalos",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        475 => DutyInfo {
            name: LocalisedText {
                en: "The Great Hunt (Extreme)",
                ja: "極リオレウス狩猟戦",
                de: "Jagd auf Rathalos (schwer)",
                fr: "Chasse au Rathalos (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        481 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tutorial",
                ja: "チョコボレース：チュートリアル",
                de: "Chocobo-Rennen: Übungsbahn",
                fr: "Course d'appentissage",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        482 => DutyInfo {
            name: LocalisedText {
                en: "Race 1 - Hugging the Inside",
                ja: "第1節：インコースを狙え",
                de: "1: Immer schön innen",
                fr: "CP1 - Toujours à la corde!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        483 => DutyInfo {
            name: LocalisedText {
                en: "Race 2 - Keep Away",
                ja: "第2節：範囲攻撃を避けろ",
                de: "2: Vorsicht, Flächenangriff",
                fr: "CP2 - Gare aux attaques à aires d'effet!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        484 => DutyInfo {
            name: LocalisedText {
                en: "Race 3 - Inability",
                ja: "第3節：アビリティに頼るな",
                de: "3: Fähigkeiten sind nicht alles",
                fr: "CP3 - En avant même sans aptitudes!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        485 => DutyInfo {
            name: LocalisedText {
                en: "Race 4 - Heavy Hooves",
                ja: "第4節：ヘヴィなレース",
                de: "4: Ein gewichtiges Rennen",
                fr: "CP4 - Une course très très pesante!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        486 => DutyInfo {
            name: LocalisedText {
                en: "Race 5 - Defending the Rush",
                ja: "第5節：とんずらを阻止せよ",
                de: "5: Hiergeblieben!",
                fr: "CP5 - Attention aux départs rapides!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        487 => DutyInfo {
            name: LocalisedText {
                en: "Race 6 - Road Rivals",
                ja: "第6節：ライバルを叩け",
                de: "6: Rivale auf Trab",
                fr: "CP6 - Pas de pitié pour les rivaux!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        488 => DutyInfo {
            name: LocalisedText {
                en: "Race 7 - Field of Dreams",
                ja: "第7節：フィールド・オブ・ドリームズ",
                de: "7: Feld der Träume",
                fr: "CP7 - Prendre du champ avec les champs!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        489 => DutyInfo {
            name: LocalisedText {
                en: "Race 8 - Playing Both Ends",
                ja: "第8節：漁夫の利を狙え",
                de: "8: Der lachende Dritte",
                fr: "CP8 - Laissons les rivaux s'éliminer!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        490 => DutyInfo {
            name: LocalisedText {
                en: "Race 9 - Stamina",
                ja: "第9節：スタミナの戦い",
                de: "9: Der längere Atem",
                fr: "CP9 - Une bataille d'endurance!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        491 => DutyInfo {
            name: LocalisedText {
                en: "Race 10 - Cat and Mouse",
                ja: "第10節：逃げる者と追う者と",
                de: "10: Zwei Temperamente",
                fr: "CP10 - Course poursuite!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        492 => DutyInfo {
            name: LocalisedText {
                en: "Race 11 - Mad Dash",
                ja: "第11節：全速で駆け抜けろ",
                de: "11: Volle Kraft voraus",
                fr: "CP11 - À bride abattue!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        493 => DutyInfo {
            name: LocalisedText {
                en: "Race 12 - Bag of Tricks",
                ja: "第12節：アビリティを駆使せよ",
                de: "12: Fähigkeiten ohne Wenn und Extra",
                fr: "CP12 - Une épreuve d'aptitude!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        494 => DutyInfo {
            name: LocalisedText {
                en: "Race 13 - Tag Team",
                ja: "第13節：連携プレイに勝利せよ",
                de: "13: Ein unschlagbares Gespann",
                fr: "CP13 - Un travail d'équipe!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        495 => DutyInfo {
            name: LocalisedText {
                en: "Race 14 - Heavier Hooves",
                ja: "第14節：続ヘヴィなレース",
                de: "14: Ein äußerst gewichtiges Rennen",
                fr: "CP14 - Une course encore plus pesante!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        496 => DutyInfo {
            name: LocalisedText {
                en: "Race 15 - Ultimatum",
                ja: "第15節：究極のレース",
                de: "15: Rennen der Superlative",
                fr: "CP15 - La course des extrêmes!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        497 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        498 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        499 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        500 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        501 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        502 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        503 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        504 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        505 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        506 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        507 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        508 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        509 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        510 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        511 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        512 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        513 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        514 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        515 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        516 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        517 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        518 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        519 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        520 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        521 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        522 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        523 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        524 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        525 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        526 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        527 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        528 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        529 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        530 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        531 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        532 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        533 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Sagolii Road",
                ja: "チョコボレース：サゴリーロード",
                de: "Chocobo-Rennen: Sagolii-Straße",
                fr: "Course de chocobos: Route de Sagolii",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        534 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Costa del Sol",
                ja: "チョコボレース：コスタ・デル・ソル",
                de: "Chocobo-Rennen: Sonnenküste",
                fr: "Course de chocobos: Costa del Sol",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        535 => DutyInfo {
            name: LocalisedText {
                en: "Chocobo Race: Tranquil Paths",
                ja: "チョコボレース：トランキルパス",
                de: "Chocobo-Rennen: Pfad der Seelenruhe",
                fr: "Course de chocobos: Sentes tranquilles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        536 => DutyInfo {
            name: LocalisedText {
                en: "The Swallow's Compass",
                ja: "風水霊殿 ガンエン廟",
                de: "Kompass der Schwalbe",
                fr: "Le Compas de l'Hirondelle",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        537 => DutyInfo {
            name: LocalisedText {
                en: "Castrum Fluminis",
                ja: "ツクヨミ討滅戦",
                de: "Götterdämmerung - Tsukuyomi",
                fr: "Castrum Fluminis",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        538 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Tsukuyomi's Pain",
                ja: "極ツクヨミ討滅戦",
                de: "Zenit der Götter - Tsukuyomi",
                fr: "Castrum Fluminis (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        539 => DutyInfo {
            name: LocalisedText {
                en: "The Weapon's Refrain (Ultimate)",
                ja: "絶アルテマウェポン破壊作戦",
                de: "Heldenlied von Ultima (fatal)",
                fr: "La Fantasmagorie d'Ultima (fatal)",
            },
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        540 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 1-10)",
                ja: "アメノミハシラ 1～10層",
                de: "Himmelssäule (Ebenen 1-10)",
                fr: "Le Pilier des Cieux (étages 1-10)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        541 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 11-20)",
                ja: "アメノミハシラ 11～20層",
                de: "Himmelssäule (Ebenen 11-20)",
                fr: "Le Pilier des Cieux (étages 11-20)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        542 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 21-30)",
                ja: "アメノミハシラ 21～30層",
                de: "Himmelssäule (Ebenen 21-30)",
                fr: "Le Pilier des Cieux (étages 21-30)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        543 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 31-40)",
                ja: "アメノミハシラ 31～40層",
                de: "Himmelssäule (Ebenen 31-40)",
                fr: "Le Pilier des Cieux (étages 31-40)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        544 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 41-50)",
                ja: "アメノミハシラ 41～50層",
                de: "Himmelssäule (Ebenen 41-50)",
                fr: "Le Pilier des Cieux (étages 41-50)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        545 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 51-60)",
                ja: "アメノミハシラ 51～60層",
                de: "Himmelssäule (Ebenen 51-60)",
                fr: "Le Pilier des Cieux (étages 51-60)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        546 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 61-70)",
                ja: "アメノミハシラ 61～70層",
                de: "Himmelssäule (Ebenen 61-70)",
                fr: "Le Pilier des Cieux (étages 61-70)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        547 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 71-80)",
                ja: "アメノミハシラ 71～80層",
                de: "Himmelssäule (Ebenen 71-80)",
                fr: "Le Pilier des Cieux (étages 71-80)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        548 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 81-90)",
                ja: "アメノミハシラ 81～90層",
                de: "Himmelssäule (Ebenen 81-90)",
                fr: "Le Pilier des Cieux (étages 81-90)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        549 => DutyInfo {
            name: LocalisedText {
                en: "Heaven-on-High  (Floors 91-100)",
                ja: "アメノミハシラ 91～100層",
                de: "Himmelssäule (Ebenen 91-100)",
                fr: "Le Pilier des Cieux (étages 91-100)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        550 => DutyInfo {
            name: LocalisedText {
                en: "The Ridorana Lighthouse",
                ja: "封じられた聖塔 リドルアナ",
                de: "Richtfeuer von Ridorana",
                fr: "Le Phare de Ridorana",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        552 => DutyInfo {
            name: LocalisedText {
                en: "Stage 1: Tutorial",
                ja: "第1節：チュートリアル",
                de: "Stufe 1: Einführung",
                fr: "Bataille 1: Tutoriel",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        553 => DutyInfo {
            name: LocalisedText {
                en: "Stage 2: Hatching a Plan",
                ja: "第2節：チョコチョコチョコチョコボ",
                de: "Stufe 2: Federn lassen",
                fr: "Bataille 2: Ô beaux chocobos",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        554 => DutyInfo {
            name: LocalisedText {
                en: "Stage 3: The First Move",
                ja: "第3節：序盤にありそうな戦い",
                de: "Stufe 3: Angriff der Ungeziefer",
                fr: "Bataille 3: Des ouvertures",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        555 => DutyInfo {
            name: LocalisedText {
                en: "Stage 4: Little Big Beast",
                ja: "第4節：小さな巨獣",
                de: "Stufe 4: Riesenbaby-Goobbue",
                fr: "Bataille 4: Grosses petites bestioles",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        556 => DutyInfo {
            name: LocalisedText {
                en: "Stage 5: Turning Tribes",
                ja: "第5節：蛮族たちの反乱",
                de: "Stufe 5: Aufstand der Wilden",
                fr: "Bataille 5: Hommes-bêtes révoltés",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        557 => DutyInfo {
            name: LocalisedText {
                en: "Stage 6: Off the Deepcroft",
                ja: "第6節：恐怖、タラタラの墓所",
                de: "Stufe 6: Gedankenspiele",
                fr: "Bataille 6: Hypogée de Tam-Tam",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        558 => DutyInfo {
            name: LocalisedText {
                en: "Stage 7: Rivals",
                ja: "第7節：ライバルたちの共闘",
                de: "Stufe 7: Vierbeinige Feinde",
                fr: "Bataille 7: Comme chien et chat",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        559 => DutyInfo {
            name: LocalisedText {
                en: "Stage 8: Always Darkest",
                ja: "第8節：暁は光と闇とを分かつ",
                de: "Stufe 8: Das Bündchen der Morgenröte",
                fr: "Bataille 8: Héritiers miniatures",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        560 => DutyInfo {
            name: LocalisedText {
                en: "Stage 9: Mine Your Minions",
                ja: "第9節：爆破、カパカパベル銅山",
                de: "Stufe 9: Auf den Schleim gegangen",
                fr: "Bataille 9: Déflagration à Clochecuivre",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        561 => DutyInfo {
            name: LocalisedText {
                en: "Stage 10: Children of Mandragora",
                ja: "第10節：マンドラ帝国の逆襲",
                de: "Stufe 10: Mandragora-Salat",
                fr: "Bataille 10: Invasion de mandragores",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        562 => DutyInfo {
            name: LocalisedText {
                en: "Stage 11: The Queen and I",
                ja: "第11節：女王陛下万歳！",
                de: "Stufe 11: Die trabantäische Allianz",
                fr: "Bataille 11: Longue vie aux Sultanes!",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        563 => DutyInfo {
            name: LocalisedText {
                en: "Stage 12: Breakout",
                ja: "第12節：デモンズブロック崩し",
                de: "Stufe 12: Alea iacta est",
                fr: "Bataille 12: Château d'Amdapeur",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        564 => DutyInfo {
            name: LocalisedText {
                en: "Stage 13: My Name Is Cid",
                ja: "第13節：筆頭機工師ガーロンド",
                de: "Stufe 13: Luftadmiral Garlond",
                fr: "Bataille 13: Épreuve des Forges",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        565 => DutyInfo {
            name: LocalisedText {
                en: "Stage 14: Like a Nut",
                ja: "第14節：ナッツの味にはもう飽きた",
                de: "Stufe 14: Eins auf die Nuss",
                fr: "Bataille 14: Casse-noisette",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        566 => DutyInfo {
            name: LocalisedText {
                en: "Stage 15: Urth's Spout",
                ja: "第15節：極小オーディン討滅戦",
                de: "Stufe 15: Der Dunkle Reiter",
                fr: "Bataille 15: Mini-fontaine d'Urth",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        567 => DutyInfo {
            name: LocalisedText {
                en: "Stage 16: Exodus",
                ja: "第16節：チョコボ大行進",
                de: "Stufe 16: Chocobo-Mania",
                fr: "Bataille 16: Parade des chocobos",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        568 => DutyInfo {
            name: LocalisedText {
                en: "Stage 17: Over the Wall",
                ja: "第17節：融解、スノークローク小氷壁",
                de: "Stufe 17: Jenseits des Schneekleids",
                fr: "Bataille 17: Bonhommes de Manteneige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        569 => DutyInfo {
            name: LocalisedText {
                en: "Stage 18: The Hunt",
                ja: "第18節：小さなモブハント",
                de: "Stufe 18: Die Trabanten-Jagd",
                fr: "Bataille 18: Mini-contrat de chasse",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        570 => DutyInfo {
            name: LocalisedText {
                en: "Stage 19: Battle on the Bitty Bridge",
                ja: "第19節：極小ギルガメッシュ討滅戦",
                de: "Stufe 19: Gilgameschs Hühnerstall",
                fr: "Bataille 19: Revanche de Gilgamini",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        571 => DutyInfo {
            name: LocalisedText {
                en: "Stage 20: Guiding Light",
                ja: "第20節：汝にクリスタルの導きあれ",
                de: "Stufe 20: Trabanten des Lichts",
                fr: "Bataille 20: Pouvoir de la Lumière",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        572 => DutyInfo {
            name: LocalisedText {
                en: "Stage 21: Wise Words",
                ja: "第21節：賢人ルイゾワの試練",
                de: "Stufe 21: Die Leveilleurs",
                fr: "Bataille 21: Louisoix et petits fils",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        573 => DutyInfo {
            name: LocalisedText {
                en: "Stage 22: World of Poor Lighting",
                ja: "第22節：薄闇の世界",
                de: "Stufe 22: Kleine Welt der Dunkelheit",
                fr: "Bataille 22: Monde des Quasi-ténèbres",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        574 => DutyInfo {
            name: LocalisedText {
                en: "Stage 23: The Binding Coil",
                ja: "第23節：大迷惑バハムート邂逅編",
                de: "Stufe 23: In die Verschlungenen Schatten",
                fr: "Bataille 23: Labyrinthe de Basta-mut",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        575 => DutyInfo {
            name: LocalisedText {
                en: "Stage 24: The Final Coil",
                ja: "第24節：大迷惑バハムート真成編",
                de: "Stufe 24: Bahamut, der Aufzieh-Primae",
                fr: "Bataille 24: Abîme de Basta-mut",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        576 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Master Battle",
                ja: "LoVM：ミニオンクラス (Easy)",
                de: "Kampf der Trabanten: Trabantenkampf",
                fr: "Bataille simple contre l'ordinateur (facile)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        577 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Master Battle (Hard)",
                ja: "LoVM：真ミニオンクラス (Normal)",
                de: "Kampf der Trabanten: Trabantendämmerung",
                fr: "Bataille simple contre l'ordinateur (normal)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        578 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Master Battle (Extreme)",
                ja: "LoVM：極ミニオンクラス (Hard)",
                de: "Kampf der Trabanten: Zenit der Trabanten",
                fr: "Bataille simple contre l'ordinateur (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        579 => DutyInfo {
            name: LocalisedText {
                en: "LoVM: Master Tournament",
                ja: "LoVM：大会対戦 (CPU対戦)",
                de: "Kampf der Trabanten: Turnier (gegen Arenameister)",
                fr: "Bataille de tournoi contre l'ordinateur",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        581 => DutyInfo {
            name: LocalisedText {
                en: "The Forbidden Land, Eureka Pagos",
                ja: "禁断の地 エウレカ：パゴス編",
                de: "Eureka Pagos",
                fr: "Eurêka Pagos",
            },
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        582 => DutyInfo {
            name: LocalisedText {
                en: "Emissary of the Dawn",
                ja: "「暁」の少年",
                de: "Der Knabe der Morgenröte",
                fr: "Voyage en terre hostile",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        583 => DutyInfo {
            name: LocalisedText {
                en: "The Calamity Retold",
                ja: "新生祭軍事演習",
                de: "Gedenkschlacht der Eorzäischen Allianz",
                fr: "Les grandes manœuvres commémoratives",
            },
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        584 => DutyInfo {
            name: LocalisedText {
                en: "Saint Mocianne's Arboretum (Hard)",
                ja: "草木汚染 聖モシャーヌ植物園 (Hard)",
                de: "Sankt Mocianne-Arboretum (schwer)",
                fr: "L'Arboretum Sainte-Mocianne (brutal)",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        585 => DutyInfo {
            name: LocalisedText {
                en: "The Burn",
                ja: "永久焦土 ザ・バーン",
                de: "Das Kargland",
                fr: "L'Escarre",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        586 => DutyInfo {
            name: LocalisedText {
                en: "The Shifting Altars of Uznair",
                ja: "宝物庫 ウズネアカナル祭殿",
                de: "Glücksaltäre von Uznair",
                fr: "Le Temple sacré d'Uznair",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        587 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V1.0",
                ja: "次元の狭間オメガ：アルファ編1",
                de: "Alphametrie 1.0",
                fr: "Alphastice v1.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        588 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V2.0",
                ja: "次元の狭間オメガ：アルファ編2",
                de: "Alphametrie 2.0",
                fr: "Alphastice v2.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        589 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V3.0",
                ja: "次元の狭間オメガ：アルファ編3",
                de: "Alphametrie 3.0",
                fr: "Alphastice v3.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        590 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V4.0",
                ja: "次元の狭間オメガ：アルファ編4",
                de: "Alphametrie 4.0",
                fr: "Alphastice v4.0",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        591 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V1.0 (Savage)",
                ja: "次元の狭間オメガ零式：アルファ編1",
                de: "Alphametrie 1.0 (episch)",
                fr: "Alphastice v1.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        592 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V2.0 (Savage)",
                ja: "次元の狭間オメガ零式：アルファ編2",
                de: "Alphametrie 2.0 (episch)",
                fr: "Alphastice v2.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        593 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V3.0 (Savage)",
                ja: "次元の狭間オメガ零式：アルファ編3",
                de: "Alphametrie 3.0 (episch)",
                fr: "Alphastice v3.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        594 => DutyInfo {
            name: LocalisedText {
                en: "Alphascape V4.0 (Savage)",
                ja: "次元の狭間オメガ零式：アルファ編4",
                de: "Alphametrie 4.0 (episch)",
                fr: "Alphastice v4.0 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        595 => DutyInfo {
            name: LocalisedText {
                en: "Kugane Ohashi",
                ja: "真ヨウジンボウ討滅戦",
                de: "Duell auf der Kugane-Brücke",
                fr: "Le Pont Ohashi",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        596 => DutyInfo {
            name: LocalisedText {
                en: "Hells' Kier",
                ja: "朱雀征魂戦",
                de: "Seelentanz - Suzaku",
                fr: "Le Nid des Lamentations",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        597 => DutyInfo {
            name: LocalisedText {
                en: "Hells' Kier (Extreme)",
                ja: "極朱雀征魂戦",
                de: "Seelensturm - Suzaku",
                fr: "Le Nid des Lamentations (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        598 => DutyInfo {
            name: LocalisedText {
                en: "The Forbidden Land, Eureka Pyros",
                ja: "禁断の地 エウレカ：ピューロス編",
                de: "Eureka Pyros",
                fr: "Eurêka Pyros",
            },
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        599 => DutyInfo {
            name: LocalisedText {
                en: "Hidden Gorge",
                ja: "ヒドゥンゴージ (機工戦)",
                de: "Verborgene Schlucht",
                fr: "Gorge dérobée (machinerie)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        600 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        601 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        602 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        603 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        604 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        605 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        606 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        607 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        608 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        609 => DutyInfo {
            name: LocalisedText {
                en: "The Will of the Moon",
                ja: "楔石の虚",
                de: "Der Wille der Mondgöttin",
                fr: "Ralliement dans la steppe",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        610 => DutyInfo {
            name: LocalisedText {
                en: "All's Well That Starts Well",
                ja: "デビューマッチ",
                de: "Debüt in der Himmlischen Arena",
                fr: "Début du spectacle",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        611 => DutyInfo {
            name: LocalisedText {
                en: "The Ghimlyt Dark",
                ja: "境界戦線 ギムリトダーク",
                de: "Die Ghimlyt-Finsternis",
                fr: "Les Ténèbres de Ghimlyt",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        612 => DutyInfo {
            name: LocalisedText {
                en: "Much Ado About Pudding",
                ja: "プリン・アラモード",
                de: "Pudding nach Art des Hauses",
                fr: "Puddings à la mode",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        613 => DutyInfo {
            name: LocalisedText {
                en: "Waiting for Golem",
                ja: "最初の岩壁「シパクナー」",
                de: "Zipacna - Aller Anfang ist schwer",
                fr: "Zipacna, le premier obstacle",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        614 => DutyInfo {
            name: LocalisedText {
                en: "Gentlemen Prefer Swords",
                ja: "怪力の鉄巨人「クレイオス」",
                de: "Kreios - Der Mann aus Stahl",
                fr: "Kreios, le destructeur d'acier",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        615 => DutyInfo {
            name: LocalisedText {
                en: "The Threepenny Turtles",
                ja: "ギルガメブラザーズ",
                de: "Die Bruderschaft der Kröten",
                fr: "La fratrie des gilkhélones",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        616 => DutyInfo {
            name: LocalisedText {
                en: "Eye Society",
                ja: "ブラインド・フューリー",
                de: "Den Tod im Auge",
                fr: "Vengeance aveugle",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        617 => DutyInfo {
            name: LocalisedText {
                en: "A Chorus Slime",
                ja: "シアーハートアタック",
                de: "Heißkalte Schauer",
                fr: "Pure attaque cardiaque",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        618 => DutyInfo {
            name: LocalisedText {
                en: "Bomb-edy of Errors",
                ja: "青い牙、赤い牙",
                de: "Explosiv - Vorhut in Blau und Rot",
                fr: "Crocs bleus et crocs rouges",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        619 => DutyInfo {
            name: LocalisedText {
                en: "To Kill a Mockingslime",
                ja: "七色の甘味「ギモーヴ」",
                de: "Guimauve und die sieben Gesichter des Todes",
                fr: "Guimauve, le goût de l'arc-en-ciel",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        620 => DutyInfo {
            name: LocalisedText {
                en: "A Little Knight Music",
                ja: "偶像の王者「クロムドゥーブ」",
                de: "Crom Dubh - König der Götzen",
                fr: "Crom Dubh, roi des idoles",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        621 => DutyInfo {
            name: LocalisedText {
                en: "Some Like It Excruciatingly Hot",
                ja: "爆ボム・ファーストアタック",
                de: "Explosiv - Die zweite Welle",
                fr: "Les courtes mèches",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        622 => DutyInfo {
            name: LocalisedText {
                en: "The Plant-om of the Opera",
                ja: "寄生植物「ヒドノラ」",
                de: "Hydnora - Der Parasit, der Paras sieht",
                fr: "Hydnora, la plante parasite",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        623 => DutyInfo {
            name: LocalisedText {
                en: "Beauty and a Beast",
                ja: "紅の死妖姫「カーミラ」",
                de: "Carmilla - Prinzessin des Todes",
                fr: "La mystérieuse Carmilla",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        624 => DutyInfo {
            name: LocalisedText {
                en: "Blobs in the Woods",
                ja: "死なばもろともーッ！",
                de: "Der Tod kommt süß gekleidet",
                fr: "La mort n'a pas d'ami",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        625 => DutyInfo {
            name: LocalisedText {
                en: "The Me Nobody Nodes",
                ja: "アラグの脅威「闘獣システム」",
                de: "Die Bestie steckt im System",
                fr: "La sphère bestiale, une menace allagoise",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        626 => DutyInfo {
            name: LocalisedText {
                en: "Sunset Bull-evard",
                ja: "豪腕の獣王「ティクバラン」",
                de: "Tikbalang - Der rechte Arm des Verderbens",
                fr: "Tikbalang, bras tout-puissant",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        627 => DutyInfo {
            name: LocalisedText {
                en: "The Sword of Music",
                ja: "剛柔の鉄巨人「クレイオス改」",
                de: "Kreios - Neues vom Mann aus Stahl",
                fr: "Kreios plie, mais ne rompt pas",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        628 => DutyInfo {
            name: LocalisedText {
                en: "Midsummer Night's Explosion",
                ja: "爆破デスマッチ",
                de: "Explosiv - Revanche mit Wumms",
                fr: "Rencontre explosive",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        629 => DutyInfo {
            name: LocalisedText {
                en: "On a Clear Day You Can Smell Forever",
                ja: "魅惑の芳香「リフレクティブ・レベッカ」",
                de: "Rebekkha - Verführerische Gerüche",
                fr: "Miroir, mon beau miroir",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        630 => DutyInfo {
            name: LocalisedText {
                en: "Miss Typhon",
                ja: "名コンビ「オルトロス＆テュポーン」",
                de: "Ultros & Typhon - Zwei wie Rotz und Wasser",
                fr: "Duo de choc: Orthros et maître Typhon",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        631 => DutyInfo {
            name: LocalisedText {
                en: "Chimera on a Hot Tin Roof",
                ja: "憤怒の合成獣「アペデマク」",
                de: "Apademak - Die cholerische Chimära",
                fr: "La chimèrique colère d'Apademak",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        632 => DutyInfo {
            name: LocalisedText {
                en: "Here Comes the Boom",
                ja: "爆発の対消滅「グランパボム」",
                de: "Explosiv - Urgroßbomber lässt es krachen",
                fr: "L'histoire détonante de Papi bombo",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        633 => DutyInfo {
            name: LocalisedText {
                en: "Behemoths and Broomsticks",
                ja: "魔獣の皇太子「クロンプリンツ・ベヒーモス」",
                de: "Kronprinz-Behemoth - Vom Teufel besessen",
                fr: "Kronprinz béhémoth, le prince héritier",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        634 => DutyInfo {
            name: LocalisedText {
                en: "Amazing Technicolor Pit Fiends",
                ja: "異形の人形師「エペロギ」",
                de: "Epilogi - Das Ende kommt immer zuletzt",
                fr: "Epilogi, l'étrange marionnettiste",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        635 => DutyInfo {
            name: LocalisedText {
                en: "Dirty Rotten Azulmagia",
                ja: "悪の青魔道士「アポカリョープス」",
                de: "Azulmagia - Der Blaumagier des Bösen",
                fr: "L'abominable Azulmagia",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        636 => DutyInfo {
            name: LocalisedText {
                en: "The Orbonne Monastery",
                ja: "楽欲の僧院 オーボンヌ",
                de: "Kloster von Orbonne",
                fr: "Le Monastère d'Orbonne",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        637 => DutyInfo {
            name: LocalisedText {
                en: "The Wreath of Snakes",
                ja: "青龍征魂戦",
                de: "Seelentanz - Seiryu",
                fr: "L'Îlot des Amertumes",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        638 => DutyInfo {
            name: LocalisedText {
                en: "The Wreath of Snakes (Extreme)",
                ja: "極青龍征魂戦",
                de: "Seelensturm - Seiryu",
                fr: "L'Îlot des Amertumes (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        639 => DutyInfo {
            name: LocalisedText {
                en: "The Forbidden Land, Eureka Hydatos",
                ja: "禁断の地 エウレカ：ヒュダトス編",
                de: "Eureka Hydatos",
                fr: "Eurêka Hydatos",
            },
            high_end: false,
            content_kind: ContentKind::Eureka,
        },
        640 => DutyInfo {
            name: LocalisedText {
                en: "Air Force One",
                ja: "出撃！ エアフォースパイロット",
                de: "Luftwaffe, Feuer frei!",
                fr: "As de l'air",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        641 => DutyInfo {
            name: LocalisedText {
                en: "Air Force One",
                ja: "出撃！ エアフォースパイロット",
                de: "Luftwaffe, Feuer frei!",
                fr: "As de l'air",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        642 => DutyInfo {
            name: LocalisedText {
                en: "Air Force One",
                ja: "出撃！ エアフォースパイロット",
                de: "Luftwaffe, Feuer frei!",
                fr: "As de l'air",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        643 => DutyInfo {
            name: LocalisedText {
                en: "Novice Mahjong (Full Ranked Match)",
                ja: "ドマ式麻雀：半荘戦一般卓（段位変動有り）",
                de: "Anfänger-Mahjong (komplette Partie, gewertet)",
                fr: "Mahjong domien: tous rangs (partie classée)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        644 => DutyInfo {
            name: LocalisedText {
                en: "Advanced Mahjong (Full Ranked Match)",
                ja: "ドマ式麻雀：半荘戦有段卓（段位変動有り）",
                de: "Fortgeschrittenen-Mahjong (komplette Partie, gewertet)",
                fr: "Mahjong domien: dan seulement (partie classée)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        645 => DutyInfo {
            name: LocalisedText {
                en: "Four-player Mahjong (Full Match, Kuitan Enabled)",
                ja: "ドマ式麻雀：半荘戦4人セット卓（クイタン有り）",
                de: "4-Spieler-Mahjong (komplette Partie, Kuitan aktiviert)",
                fr: "Mahjong domien: 4 joueurs (partie avec kuitan)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        646 => DutyInfo {
            name: LocalisedText {
                en: "Messenger of the Winds",
                ja: "来訪せし風の御使",
                de: "Durch den Sturm und zurück",
                fr: "La Messagère du vent",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        648 => DutyInfo {
            name: LocalisedText {
                en: "A Requiem for Heroes",
                ja: "英雄への鎮魂歌",
                de: "Requiem der Helden",
                fr: "Un requiem pour les héros",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        649 => DutyInfo {
            name: LocalisedText {
                en: "Dohn Mheg",
                ja: "水妖幻園 ドォーヌ・メグ",
                de: "Dohn Mheg",
                fr: "Dohn Mheg",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        650 => DutyInfo {
            name: LocalisedText {
                en: "Four-player Mahjong (Full Match, Kuitan Disabled)",
                ja: "ドマ式麻雀：半荘戦4人セット卓（クイタン無し）",
                de: "4-Spieler-Mahjong (komplette Partie, Kuitan deaktiviert)",
                fr: "Mahjong domien: 4 joueurs (partie sans kuitan)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        651 => DutyInfo {
            name: LocalisedText {
                en: "The Qitana Ravel",
                ja: "古跡探索 キタンナ神影洞",
                de: "Irrungen der Qitari",
                fr: "L'Enchevêtrement des Qitari",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        652 => DutyInfo {
            name: LocalisedText {
                en: "Amaurot",
                ja: "終末幻想 アーモロート",
                de: "Amaurot",
                fr: "Amaurote",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        653 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Resurrection",
                ja: "希望の園エデン：覚醒編1",
                de: "Edens Erwachen - Auferstehung",
                fr: "L'Éveil d'Éden - Résurrection",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        654 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Resurrection (Savage)",
                ja: "希望の園エデン零式：覚醒編1",
                de: "Edens Erwachen - Auferstehung (episch)",
                fr: "L'Éveil d'Éden - Résurrection (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        655 => DutyInfo {
            name: LocalisedText {
                en: "The Twinning",
                ja: "異界遺構 シルクス・ツイニング",
                de: "Der Kristallzwilling",
                fr: "La Macle de Syrcus",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        656 => DutyInfo {
            name: LocalisedText {
                en: "Malikah's Well",
                ja: "爽涼離宮 マリカの大井戸",
                de: "Malikahs Brunnen",
                fr: "Le Puits de Malikah",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        657 => DutyInfo {
            name: LocalisedText {
                en: "The Dancing Plague",
                ja: "ティターニア討滅戦",
                de: "Offenbarung - Titania",
                fr: "La Valse du Monarque",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        658 => DutyInfo {
            name: LocalisedText {
                en: "The Dancing Plague (Extreme)",
                ja: "極ティターニア討滅戦",
                de: "Letzte Läuterung - Titania",
                fr: "La Valse du Monarque (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        659 => DutyInfo {
            name: LocalisedText {
                en: "Mt. Gulg",
                ja: "偽造天界 グルグ火山",
                de: "Der Gulg",
                fr: "Mont Gulg",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        661 => DutyInfo {
            name: LocalisedText {
                en: "Akadaemia Anyder",
                ja: "創造機関 アナイダアカデミア",
                de: "Akadaemia Anyder",
                fr: "Akadaemia Anydre",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        666 => DutyInfo {
            name: LocalisedText {
                en: "The Crown of the Immaculate",
                ja: "イノセンス討滅戦",
                de: "Offenbarung - Innozenz",
                fr: "La Couronne de l'Immaculé",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        667 => DutyInfo {
            name: LocalisedText {
                en: "The Crown of the Immaculate (Extreme)",
                ja: "極イノセンス討滅戦",
                de: "Letzte Läuterung - Innozenz",
                fr: "La Couronne de l'Immaculé (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        676 => DutyInfo {
            name: LocalisedText {
                en: "Holminster Switch",
                ja: "殺戮郷村 ホルミンスター",
                de: "Holminster",
                fr: "Holminster",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        678 => DutyInfo {
            name: LocalisedText {
                en: "The Hardened Heart",
                ja: "揺れる天秤",
                de: "Ob Mitleid oder Hass",
                fr: "Naissance d'un bourreau",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        679 => DutyInfo {
            name: LocalisedText {
                en: "The Lost and the Found",
                ja: "古の大再生魔法",
                de: "Alter Zauber",
                fr: "Magie ancestrale",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        680 => DutyInfo {
            name: LocalisedText {
                en: "Coming Clean",
                ja: "廃都ナバスアレン",
                de: "Vater und Bruder",
                fr: "Sur les rails de Nabaath Areng",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        681 => DutyInfo {
            name: LocalisedText {
                en: "Legend of the Not-so-hidden Temple",
                ja: "仕掛けと呪いと毒と",
                de: "Der Beichtstuhl von Toupasa dem Älteren",
                fr: "Le Confessionnal de Toupasa l'ancien",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        682 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Inundation",
                ja: "希望の園エデン：覚醒編3",
                de: "Edens Erwachen - Überflutung",
                fr: "L'Éveil d'Éden - Déluge",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        683 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Inundation (Savage)",
                ja: "希望の園エデン零式：覚醒編3",
                de: "Edens Erwachen - Überflutung (episch)",
                fr: "L'Éveil d'Éden - Déluge (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        684 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Descent",
                ja: "希望の園エデン：覚醒編2",
                de: "Edens Erwachen - Niederkunft",
                fr: "L'Éveil d'Éden - Descente",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        685 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Descent (Savage)",
                ja: "希望の園エデン零式：覚醒編2",
                de: "Edens Erwachen - Niederkunft (episch)",
                fr: "L'Éveil d'Éden - Descente (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        686 => DutyInfo {
            name: LocalisedText {
                en: "Nyelbert's Lament",
                ja: "ナイルベルトの後悔",
                de: "Ein großes Opfer",
                fr: "Une cupidité bien généreuse",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        687 => DutyInfo {
            name: LocalisedText {
                en: "The Dying Gasp",
                ja: "ハーデス討滅戦",
                de: "Offenbarung - Hades",
                fr: "Le Râle de l'Agonie",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        688 => DutyInfo {
            name: LocalisedText {
                en: "The Dungeons of Lyhe Ghiah",
                ja: "宝物庫 リェー・ギア・ダンジョン",
                de: "Verliese von Lyhe Ghiah",
                fr: "Le Donjon hypogéen du Lyhe Ghiah",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        689 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Sepulture",
                ja: "希望の園エデン：覚醒編4",
                de: "Edens Erwachen - Beerdigung",
                fr: "L'Éveil d'Éden - Inhumation",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        690 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Gate: Sepulture (Savage)",
                ja: "希望の園エデン零式：覚醒編4",
                de: "Edens Erwachen - Beerdigung (episch)",
                fr: "L'Éveil d'Éden - Inhumation (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        691 => DutyInfo {
            name: LocalisedText {
                en: "The Hunter's Legacy",
                ja: "勇気の狩人",
                de: "Der Legende auf der Spur",
                fr: "La chasseuse de légende",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        692 => DutyInfo {
            name: LocalisedText {
                en: "The Grand Cosmos",
                ja: "魔法宮殿 グラン・コスモス",
                de: "Chateau Cosmea",
                fr: "Le Cosmos coruscant",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        693 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Hades's Elegy",
                ja: "極ハーデス討滅戦",
                de: "Letzte Läuterung - Hades",
                fr: "Le Râle de l'Agonie (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        694 => DutyInfo {
            name: LocalisedText {
                en: "The Epic of Alexander (Ultimate)",
                ja: "絶アレキサンダー討滅戦",
                de: "Alexander (fatal)",
                fr: "L'Odyssée d'Alexander (fatal)",
            },
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        695 => DutyInfo {
            name: LocalisedText {
                en: "Papa Mia",
                ja: "豪腕の父親「フンババ・パパ」",
                de: "Papa Mia",
                fr: "Papa Humbaba, le paternel aux gros bras",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        696 => DutyInfo {
            name: LocalisedText {
                en: "Lock up Your Snorters",
                ja: "はないき爆破デスマッチ",
                de: "Explosives Schnaufen",
                fr: "Fungaaah! Et boum!",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        697 => DutyInfo {
            name: LocalisedText {
                en: "Dangerous When Dead",
                ja: "至妙の傀儡子「ドゥリン」",
                de: "Begnadeter Puppenspieler",
                fr: "Durinn, marionnettiste d'outre-tombe",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        698 => DutyInfo {
            name: LocalisedText {
                en: "Red, Fraught, and Blue",
                ja: "水と炎の歌",
                de: "Die Melodie von Feuer und Wasser",
                fr: "La mélodie du feu et de l'eau",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        699 => DutyInfo {
            name: LocalisedText {
                en: "The Catch of the Siegfried",
                ja: "世界一の剣士「ジークフリード」",
                de: "Siegfried, der beste Schwertkämpfer der Welt!",
                fr: "Siegfried, le plus grand bretteur du monde",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        700 => DutyInfo {
            name: LocalisedText {
                en: "The Copied Factory",
                ja: "複製サレタ工場廃墟",
                de: "Die kopierte Fabrik",
                fr: "La réplique de l'usine désaffectée",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        701 => DutyInfo {
            name: LocalisedText {
                en: "Onsal Hakair (Danshig Naadam)",
                ja: "オンサル・ハカイル (終節戦)",
                de: "Onsal Hakair (Danshig Naadam)",
                fr: "Onsal Hakair (Danshig Naadam)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        702 => DutyInfo {
            name: LocalisedText {
                en: "Vows of Virtue, Deeds of Cruelty",
                ja: "白き誓約、黒き密約",
                de: "Der Wolf und der Drachenreiter",
                fr: "Vœux de vertu, actes de cruauté",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        703 => DutyInfo {
            name: LocalisedText {
                en: "As the Heart Bids",
                ja: "この心が望むがままに",
                de: "Trubel im Traumland",
                fr: "À l'écoute de soi",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        705 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        706 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        707 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        708 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        709 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        710 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        711 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        712 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        713 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        714 => DutyInfo {
            name: LocalisedText {
                en: "Anamnesis Anyder",
                ja: "黒風海底 アニドラス・アナムネーシス",
                de: "Anamnesis Anyder",
                fr: "Anamnesis Anydre",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        715 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Fulmination",
                ja: "希望の園エデン：共鳴編1",
                de: "Edens Resonanz - Entladung",
                fr: "Les Accords d'Éden - Fulmination",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        716 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Fulmination (Savage)",
                ja: "希望の園エデン零式：共鳴編1",
                de: "Edens Resonanz - Entladung (episch)",
                fr: "Les Accords d'Éden - Fulmination (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        717 => DutyInfo {
            name: LocalisedText {
                en: "Cinder Drift",
                ja: "ルビーウェポン破壊作戦",
                de: "Rubinfeuer - Entfesselung",
                fr: "Les Nuées de Brandons",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        718 => DutyInfo {
            name: LocalisedText {
                en: "Cinder Drift (Extreme)",
                ja: "極ルビーウェポン破壊作戦",
                de: "Rubinfeuer - Trauma",
                fr: "Les Nuées de Brandons (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        719 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Furor",
                ja: "希望の園エデン：共鳴編2",
                de: "Edens Resonanz - Raserei",
                fr: "Les Accords d'Éden - Fureur",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        720 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Furor (Savage)",
                ja: "希望の園エデン零式：共鳴編2",
                de: "Edens Resonanz - Raserei (episch)",
                fr: "Les Accords d'Éden - Fureur (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        721 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        722 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem",
                ja: "雲海採集 ディアデム諸島",
                de: "Das Diadem - Erschließung",
                fr: "Le Diadème",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        723 => DutyInfo {
            name: LocalisedText {
                en: "The Bozja Incident",
                ja: "シタデル・ボズヤ蒸発事変",
                de: "Der Bozja-Vorfall",
                fr: "Prélude à la catastrophe",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        724 => DutyInfo {
            name: LocalisedText {
                en: "A Sleep Disturbed",
                ja: "汝、英雄の眠り妨げるは",
                de: "Von schlafenden Helden",
                fr: "L'épreuve ronka",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        725 => DutyInfo {
            name: LocalisedText {
                en: "Memoria Misera (Extreme)",
                ja: "極シタデル・ボズヤ追憶戦",
                de: "Memoria Misera (extrem)",
                fr: "Memoria Misera (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        726 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Iconoclasm",
                ja: "希望の園エデン：共鳴編3",
                de: "Edens Resonanz - Bildersturm",
                fr: "Les Accords d'Éden - Iconoclasme",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        727 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Iconoclasm (Savage)",
                ja: "希望の園エデン零式：共鳴編3",
                de: "Edens Resonanz - Bildersturm (episch)",
                fr: "Les Accords d'Éden - Iconoclasme (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        728 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Refulgence",
                ja: "希望の園エデン：共鳴編4",
                de: "Edens Resonanz - Erstarrung",
                fr: "Les Accords d'Éden - Éclat",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        729 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Verse: Refulgence (Savage)",
                ja: "希望の園エデン零式：共鳴編4",
                de: "Edens Resonanz - Erstarrung (episch)",
                fr: "Les Accords d'Éden - Éclat (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        730 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        731 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        732 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        733 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        734 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        735 => DutyInfo {
            name: LocalisedText {
                en: "The Bozjan Southern Front",
                ja: "南方ボズヤ戦線",
                de: "Bozja-Südfront",
                fr: "Front sud de Bozja",
            },
            high_end: false,
            content_kind: ContentKind::SavetheQueen,
        },
        736 => DutyInfo {
            name: LocalisedText {
                en: "The Puppets' Bunker",
                ja: "人形タチノ軍事基地",
                de: "Die Puppenfestung",
                fr: "La base militaire des Pantins",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        737 => DutyInfo {
            name: LocalisedText {
                en: "The Heroes' Gauntlet",
                ja: "漆黒決戦 ノルヴラント",
                de: "Schlacht um Norvrandt",
                fr: "La Traversée de Norvrandt",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        738 => DutyInfo {
            name: LocalisedText {
                en: "The Seat of Sacrifice",
                ja: "ウォーリア・オブ・ライト討滅戦",
                de: "Krieger des Lichts",
                fr: "Le Trône du Sacrifice",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        739 => DutyInfo {
            name: LocalisedText {
                en: "The Seat of Sacrifice (Extreme)",
                ja: "極ウォーリア・オブ・ライト討滅戦",
                de: "Krieger des Lichts (extrem)",
                fr: "Le Trône du Sacrifice (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        740 => DutyInfo {
            name: LocalisedText {
                en: "Sleep Now in Sapphire",
                ja: "飛べ！ ウェルリトへ ",
                de: "Luftangriff auf Werlyt",
                fr: "Sur la mer de saphir",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        741 => DutyInfo {
            name: LocalisedText {
                en: "Sleep Now in Sapphire",
                ja: "飛べ！ ウェルリトへ ",
                de: "Luftangriff auf Werlyt",
                fr: "Sur la mer de saphir",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        742 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem",
                ja: "雲海採集 ディアデム諸島",
                de: "Das Diadem - Erschließung",
                fr: "Le Diadème",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        743 => DutyInfo {
            name: LocalisedText {
                en: "Faded Memories",
                ja: "色あせた記憶",
                de: "Verblasste Erinnerungen",
                fr: "Souvenir périssable",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        745 => DutyInfo {
            name: LocalisedText {
                en: "The Shifting Oubliettes of Lyhe Ghiah",
                ja: "宝物庫 リェー・ギア・ダンジョン祭殿",
                de: "Das Karussell von Lyhe Ghiah",
                fr: "Le Jardin secret du Lyhe Ghiah",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        746 => DutyInfo {
            name: LocalisedText {
                en: "Matoya's Relict",
                ja: "魔術工房 マトーヤのアトリエ",
                de: "Matoyas Atelier",
                fr: "L'Atelier abandonné de Matoya",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        747 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Litany",
                ja: "希望の園エデン：再生編2",
                de: "Edens Verheißung - Litanei",
                fr: "La Promesse d'Éden - Litanie",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        748 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Litany (Savage)",
                ja: "希望の園エデン零式：再生編2",
                de: "Edens Verheißung - Litanei (episch)",
                fr: "La Promesse d'Éden - Litanie (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        749 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Umbra",
                ja: "希望の園エデン：再生編1",
                de: "Edens Verheißung - Umbra",
                fr: "La Promesse d'Éden - Nuée",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        750 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Umbra (Savage)",
                ja: "希望の園エデン零式：再生編1",
                de: "Edens Verheißung - Umbra (episch)",
                fr: "La Promesse d'Éden - Nuée (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        751 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Anamorphosis",
                ja: "希望の園エデン：再生編3",
                de: "Edens Verheißung - Anamorphose",
                fr: "La Promesse d'Éden - Anamorphose",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        752 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Anamorphosis (Savage)",
                ja: "希望の園エデン零式：再生編3",
                de: "Edens Verheißung - Anamorphose (episch)",
                fr: "La Promesse d'Éden - Anamorphose (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        753 => DutyInfo {
            name: LocalisedText {
                en: "The Diadem",
                ja: "雲海採集 ディアデム諸島",
                de: "Das Diadem - Erschließung",
                fr: "Le Diadème",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        754 => DutyInfo {
            name: LocalisedText {
                en: "Anything Gogo's",
                ja: "黄金頭巾「ものまね士ゴゴ」",
                de: "Gogo der Mime",
                fr: "Gogo le mime",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        755 => DutyInfo {
            name: LocalisedText {
                en: "Triple Triad Open Tournament",
                ja: "トリプルトライアド：オフィシャルトーナメント",
                de: "Triple Triad: Manderville-Turnier",
                fr: "Tournoi officiel de Triple Triade",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        756 => DutyInfo {
            name: LocalisedText {
                en: "Triple Triad Invitational Parlor",
                ja: "トリプルトライアド：カスタムトーナメントルーム",
                de: "Triple Triad: Privatturnier",
                fr: "Salle de tournoi libre de Triple Triade",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        758 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Eternity",
                ja: "希望の園エデン：再生編4",
                de: "Edens Verheißung - Ewigkeit",
                fr: "La Promesse d'Éden - Éternité",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        759 => DutyInfo {
            name: LocalisedText {
                en: "Eden's Promise: Eternity (Savage)",
                ja: "希望の園エデン零式：再生編4",
                de: "Edens Verheißung - Ewigkeit (episch)",
                fr: "La Promesse d'Éden - Éternité (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        760 => DutyInfo {
            name: LocalisedText {
                en: "Delubrum Reginae",
                ja: "グンヒルド・ディルーブラム",
                de: "Delubrum Reginae",
                fr: "Delubrum Reginae",
            },
            high_end: false,
            content_kind: ContentKind::SavetheQueen,
        },
        761 => DutyInfo {
            name: LocalisedText {
                en: "Delubrum Reginae (Savage)",
                ja: "グンヒルド・ディルーブラム零式",
                de: "Delubrum Reginae (episch)",
                fr: "Delubrum Reginae (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::SavetheQueen,
        },
        762 => DutyInfo {
            name: LocalisedText {
                en: "Castrum Marinum",
                ja: "エメラルドウェポン破壊作戦",
                de: "Smaragdsturm - Entfesselung",
                fr: "Castrum Marinum",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        763 => DutyInfo {
            name: LocalisedText {
                en: "Castrum Marinum (Extreme)",
                ja: "極エメラルドウェポン破壊作戦",
                de: "Smaragdsturm - Trauma",
                fr: "Castrum Marinum (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        764 => DutyInfo {
            name: LocalisedText {
                en: "The Great Ship Vylbrand",
                ja: "バイルブランドの船出",
                de: "Gute Winde für Vylbrand",
                fr: "Un navire nommé Vylbrand",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        765 => DutyInfo {
            name: LocalisedText {
                en: "Fit for a Queen",
                ja: "ゴッド・セイブ・ザ・クイーン",
                de: "Hinab in die Ruinen",
                fr: "Que les Dieux gardent la Reine",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        766 => DutyInfo {
            name: LocalisedText {
                en: "Novice Mahjong (Quick Ranked Match)",
                ja: "ドマ式麻雀：東風戦一般卓（段位変動有り）",
                de: "Anfänger-Mahjong (schnelle Partie, gewertet)",
                fr: "Mahjong domien: tous rangs (partie rapide classée)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        767 => DutyInfo {
            name: LocalisedText {
                en: "Advanced Mahjong (Quick Ranked Match)",
                ja: "ドマ式麻雀：東風戦有段卓（段位変動有り）",
                de: "Fortgeschrittenen-Mahjong (schnelle Partie, gewertet)",
                fr: "Mahjong domien: dan seulement (partie rapide classée)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        768 => DutyInfo {
            name: LocalisedText {
                en: "Four-player Mahjong (Quick Match, Kuitan Enabled)",
                ja: "ドマ式麻雀：東風戦4人セット卓（クイタン有り）",
                de: "4-Spieler-Mahjong (schnelle Partie, Kuitan aktiviert)",
                fr: "Mahjong domien: 4 joueurs (partie rapide avec kuitan)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        769 => DutyInfo {
            name: LocalisedText {
                en: "Four-player Mahjong (Quick Match, Kuitan Disabled)",
                ja: "ドマ式麻雀：東風戦4人セット卓（クイタン無し）",
                de: "4-Spieler-Mahjong (schnelle Partie, Kuitan deaktiviert)",
                fr: "Mahjong domien: 4 joueurs (partie rapide sans kuitan)",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        770 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        771 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        772 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        773 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        774 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        775 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        777 => DutyInfo {
            name: LocalisedText {
                en: "Paglth'an",
                ja: "黄金平原 パガルザン",
                de: "Die Goldene Ebene von Paglth'an",
                fr: "La grande prairie de Paglth'an",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        778 => DutyInfo {
            name: LocalisedText {
                en: "Zadnor",
                ja: "ザトゥノル高原",
                de: "Zadnor-Hochebene",
                fr: "Hauts plateaux de Zadnor",
            },
            high_end: false,
            content_kind: ContentKind::SavetheQueen,
        },
        779 => DutyInfo {
            name: LocalisedText {
                en: "The Tower at Paradigm's Breach",
                ja: "希望ノ砲台：「塔」",
                de: "Der Turm, Paradigmenbrecher",
                fr: "La tour de la Contingence",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        780 => DutyInfo {
            name: LocalisedText {
                en: "Death Unto Dawn",
                ja: "黎明の死闘",
                de: "Kampf im Morgengrauen",
                fr: "Aube meurtrière",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        781 => DutyInfo {
            name: LocalisedText {
                en: "The Cloud Deck",
                ja: "ダイヤウェポン捕獲作戦",
                de: "Diamantblitz - Entfesselung",
                fr: "Le Tillac des Cirrus",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        782 => DutyInfo {
            name: LocalisedText {
                en: "The Cloud Deck (Extreme)",
                ja: "極ダイヤウェポン捕獲作戦",
                de: "Diamantblitz - Trauma",
                fr: "Le Tillac des Cirrus (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        783 => DutyInfo {
            name: LocalisedText {
                en: "The Tower of Zot",
                ja: "異形楼閣 ゾットの塔",
                de: "Der Turm von Zot",
                fr: "La tour de Zott",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        784 => DutyInfo {
            name: LocalisedText {
                en: "The Stigma Dreamscape",
                ja: "電脳夢想 スティグマ・フォー",
                de: "Stigma-Holometrie",
                fr: "Rêve électrique de Stigma-4",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        785 => DutyInfo {
            name: LocalisedText {
                en: "The Tower of Babil",
                ja: "魔導神門 バブイルの塔",
                de: "Der Turm von Babil",
                fr: "La tour de Babil",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        786 => DutyInfo {
            name: LocalisedText {
                en: "The Aitiascope",
                ja: "星海潜航 アイティオン星晶鏡",
                de: "Das Aitiaskop",
                fr: "Le Prisme de l'Aitia",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        787 => DutyInfo {
            name: LocalisedText {
                en: "Ktisis Hyperboreia",
                ja: "創造環境 ヒュペルボレア造物院",
                de: "Ktisis Hyperboreia",
                fr: "L'Hyperborée",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        788 => DutyInfo {
            name: LocalisedText {
                en: "Dragonsong's Reprise (Ultimate)",
                ja: "絶竜詩戦争",
                de: "Drachenkrieg (fatal)",
                fr: "La Guerre du chant des dragons (fatal)",
            },
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        789 => DutyInfo {
            name: LocalisedText {
                en: "Vanaspati",
                ja: "終末樹海 ヴァナスパティ",
                de: "Vanaspati",
                fr: "Vanaspati",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        790 => DutyInfo {
            name: LocalisedText {
                en: "The Mothercrystal",
                ja: "ハイデリン討滅戦",
                de: "Prophetie - Hydaelyn",
                fr: "Le Cristal-mère",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        791 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Hydaelyn's Call",
                ja: "極ハイデリン討滅戦",
                de: "Eschatos - Hydaelyn",
                fr: "Le Cristal-mère (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        792 => DutyInfo {
            name: LocalisedText {
                en: "The Dead Ends",
                ja: "最終幻想 レムナント",
                de: "Das Sternengrab",
                fr: "L'Issue aux Impasses",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        793 => DutyInfo {
            name: LocalisedText {
                en: "In from the Cold",
                ja: "寒夜のこと",
                de: "In fremder Haut",
                fr: "Le voleur de corps",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        794 => DutyInfo {
            name: LocalisedText {
                en: "Smileton",
                ja: "楽園都市 スマイルトン",
                de: "Smileton",
                fr: "Risette-sur-lune",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        795 => DutyInfo {
            name: LocalisedText {
                en: "Worthy of His Back",
                ja: "前代アゼムの手ほどき",
                de: " Es reimt sich auf Gebell",
                fr: "Le défi de l'ancienne Azem",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        796 => DutyInfo {
            name: LocalisedText {
                en: "The Final Day",
                ja: "終焉の戦い",
                de: "Prophetie - Endsängerin",
                fr: "Le Répons final",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        797 => DutyInfo {
            name: LocalisedText {
                en: "The Phantoms' Feast",
                ja: "道化饗宴 ホーンテッドフィースト",
                de: "Lustiges Bankett",
                fr: "Le banquet cauchemardesque",
            },
            high_end: false,
            content_kind: ContentKind::Other(22),
        },
        798 => DutyInfo {
            name: LocalisedText {
                en: "Endwalker",
                ja: "暁月のフィナーレ",
                de: "Endschreiter",
                fr: "Arpenteur des finitudes",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        799 => DutyInfo {
            name: LocalisedText {
                en: "To Calmer Seas",
                ja: "融和への船出",
                de: "Im Hafen des Friedens",
                fr: "Cap sur la paix",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        800 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The Fourth Circle",
                ja: "万魔殿パンデモニウム：辺獄編4",
                de: "Asphodelos - Vierter Kreis",
                fr: "Les Limbes du Pandæmonium - Abîme",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        801 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The Fourth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：辺獄編4",
                de: "Asphodelos - Vierter Kreis (episch)",
                fr: "Les Limbes du Pandæmonium - Abîme (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        802 => DutyInfo {
            name: LocalisedText {
                en: "The Dark Inside",
                ja: "ゾディアーク討滅戦",
                de: "Prophetie - Zodiark",
                fr: "Le Cratère des Martyrs",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        803 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Zodiark's Fall",
                ja: "極ゾディアーク討滅戦",
                de: "Eschatos - Zodiark",
                fr: "Le Cratère des Martyrs (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        804 => DutyInfo {
            name: LocalisedText {
                en: "As the Heavens Burn",
                ja: "拡がる終末",
                de: "Rote Himmel, roter Schnee",
                fr: "L'arène des neiges",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        805 => DutyInfo {
            name: LocalisedText {
                en: "A Path Unveiled",
                ja: "開かれた道の先へ",
                de: "Offen für neue Wege",
                fr: "Des esprits et des hommes",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        806 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The Third Circle",
                ja: "万魔殿パンデモニウム：辺獄編3",
                de: "Asphodelos - Dritter Kreis",
                fr: "Les Limbes du Pandæmonium - Fournaise",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        807 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The Third Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：辺獄編3",
                de: "Asphodelos - Dritter Kreis (episch)",
                fr: "Les Limbes du Pandæmonium - Fournaise (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        808 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The First Circle",
                ja: "万魔殿パンデモニウム：辺獄編1",
                de: "Asphodelos - Erster Kreis",
                fr: "Les Limbes du Pandæmonium - Parvis",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        809 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The First Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：辺獄編1",
                de: "Asphodelos - Erster Kreis (episch)",
                fr: "Les Limbes du Pandæmonium - Parvis (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        810 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The Second Circle",
                ja: "万魔殿パンデモニウム：辺獄編2",
                de: "Asphodelos - Zweiter Kreis",
                fr: "Les Limbes du Pandæmonium - Cloaque",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        811 => DutyInfo {
            name: LocalisedText {
                en: "Asphodelos: The Second Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：辺獄編2",
                de: "Asphodelos - Zweiter Kreis (episch)",
                fr: "Les Limbes du Pandæmonium - Cloaque (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        812 => DutyInfo {
            name: LocalisedText {
                en: "A Frosty Reception",
                ja: "霜雪を踏みしめて",
                de: "Ein frostiger Empfang",
                fr: "Un accueil glacial",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        813 => DutyInfo {
            name: LocalisedText {
                en: "Sage's Focus",
                ja: "賢者の短杖",
                de: "Des Weisen wundersames Werkzeug",
                fr: "Les armes du sage",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        814 => DutyInfo {
            name: LocalisedText {
                en: "The Harvest Begins",
                ja: "大鎌の意味",
                de: "Die Bedeutung der Sense",
                fr: "La vraie puissance de la faux",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        815 => DutyInfo {
            name: LocalisedText {
                en: "Ever March Heavensward",
                ja: "蒼天を仰ぎ、歩み続ける",
                de: "Der Weg zur Erneuerung",
                fr: "La voie du renouveau",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        816 => DutyInfo {
            name: LocalisedText {
                en: "The Killing Art",
                ja: "暗殺道",
                de: "Die Kunst des Tötens",
                fr: "La voie du néant",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        817 => DutyInfo {
            name: LocalisedText {
                en: "Life Ephemeral, Path Eternal",
                ja: "人命は儚く、術のみちは永久に",
                de: "Das Leben ist kurz, die Kunst ist lang",
                fr: "Existences éphémères et savoir éternel",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        818 => DutyInfo {
            name: LocalisedText {
                en: "Laid to Rest",
                ja: "ドマの弔い",
                de: "Domanisches Begräbnis",
                fr: "Des adieux domiens",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        819 => DutyInfo {
            name: LocalisedText {
                en: "The Excitatron 6000",
                ja: "宝物庫 エキサイトロン",
                de: "Euphoratron",
                fr: "Le Ludodrome",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        820 => DutyInfo {
            name: LocalisedText {
                en: "The Gift of Mercy",
                ja: "僕たちは還り、君を見送ろう",
                de: "Trauer und Hoffnung",
                fr: "Acceptation",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        822 => DutyInfo {
            name: LocalisedText {
                en: "The Aetherfont",
                ja: "星霊間欠 ハーム島",
                de: "Ätherborn",
                fr: "L'île de Haam",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        823 => DutyInfo {
            name: LocalisedText {
                en: "The Lunar Subterrane",
                ja: "深淵潜行 月の地下渓谷",
                de: "Monduntergrund",
                fr: "Le Souterrain lunaire",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        824 => DutyInfo {
            name: LocalisedText {
                en: "Worqor Zormor",
                ja: "山嶺登頂 ウォーコー・ゾーモー",
                de: "Worqor Zormor",
                fr: "Worqor Zormor",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        825 => DutyInfo {
            name: LocalisedText {
                en: "Origenics",
                ja: "魂魄工廠 オリジェニクス",
                de: "Origenik",
                fr: "L'Origenèse",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        826 => DutyInfo {
            name: LocalisedText {
                en: "Ihuykatumu",
                ja: "濁流遡上 イフイカ・トゥム",
                de: "Ihuykatumu",
                fr: "La Remontée de l'Ihuykatumu",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        827 => DutyInfo {
            name: LocalisedText {
                en: "Alexandria",
                ja: "記憶幻想 アレクサンドリア",
                de: "Alexandria",
                fr: "Alexandrie",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        828 => DutyInfo {
            name: LocalisedText {
                en: "A Father First",
                ja: "王として、父として",
                de: "Des Vaters viele Sorgen",
                fr: "L'Aurarque et le père",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        829 => DutyInfo {
            name: LocalisedText {
                en: "The Skydeep Cenote",
                ja: "遺産踏査 天深きセノーテ",
                de: "Himmelstiefer Cenote",
                fr: "Le cénote des Cieux infinis",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        830 => DutyInfo {
            name: LocalisedText {
                en: "The Porta Decumana",
                ja: "アルテマウェポン破壊作戦",
                de: "Porta Decumana",
                fr: "Porta Decumana",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        831 => DutyInfo {
            name: LocalisedText {
                en: "Vanguard",
                ja: "外征前哨 ヴァンガード",
                de: "Der Außenposten",
                fr: "L'Avant-garde",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        832 => DutyInfo {
            name: LocalisedText {
                en: "Worqor Lar Dor",
                ja: "ヴァリガルマンダ討滅戦",
                de: "Zel Tajaal - Valigarmanda",
                fr: "Worqor Lar Dor",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        833 => DutyInfo {
            name: LocalisedText {
                en: "Worqor Lar Dor (Extreme)",
                ja: "極ヴァリガルマンダ討滅戦",
                de: "Gok Tajaal - Valigarmanda",
                fr: "Worqor Lar Dor (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        834 => DutyInfo {
            name: LocalisedText {
                en: "Tender Valley",
                ja: "荒野秘境 サボテンダーバレー",
                de: "Kaktorsenke",
                fr: "La Vallée des Pampas",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        835 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        836 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        837 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        838 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        839 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        840 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        841 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        842 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        843 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        844 => DutyInfo {
            name: LocalisedText {
                en: "Alzadaal's Legacy",
                ja: "近東秘宝 アルザダール海底遺跡群",
                de: "Alzadaals Vermächtnis",
                fr: "Le legs d'Alzadaal",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        845 => DutyInfo {
            name: LocalisedText {
                en: "Forlorn Glory",
                ja: "ネルウァの帝国",
                de: "Nervas Reich",
                fr: "Nerva, le patriote",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        846 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Endsinger's Aria",
                ja: "終極の戦い",
                de: "Eschatos - Endsängerin",
                fr: "Le Répons final (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        847 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        848 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        849 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        850 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        851 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        852 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        853 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        854 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        855 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        856 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        857 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        858 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        859 => DutyInfo {
            name: LocalisedText {
                en: "The Palaistra",
                ja: "パライストラ",
                de: "Die Palästra",
                fr: "Le Palestre",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        860 => DutyInfo {
            name: LocalisedText {
                en: "The Volcanic Heart",
                ja: "ヴォルカニック・ハート",
                de: "Das Herz des Vulkans",
                fr: "Le Cœur volcanique",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        861 => DutyInfo {
            name: LocalisedText {
                en: "Cloud Nine",
                ja: "クラウドナイン",
                de: "Wolke Sieben",
                fr: "Le Petit Nuage",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        862 => DutyInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Custom Match - The Palaistra)",
                ja: "クリスタルコンフリクト(パライストラ：カスタムマッチ)",
                de: "Crystalline Conflict: Die Palästra (Schaukampf)",
                fr: "Crystalline Conflict (partie personnalisée - Le Palestre)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        863 => DutyInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Custom Match - The Volcanic Heart)",
                ja: "クリスタルコンフリクト(ヴォルカニック・ハート：カスタムマッチ)",
                de: "Crystalline Conflict: Das Herz des Vulkans (Schaukampf)",
                fr: "Crystalline Conflict (partie personnalisée - Le Cœur volcanique)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        864 => DutyInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Custom Match - Cloud Nine)",
                ja: "クリスタルコンフリクト(クラウドナイン：カスタムマッチ)",
                de: "Crystalline Conflict: Wolke Sieben (Schaukampf)",
                fr: "Crystalline Conflict (partie personnalisée - Le Petit Nuage)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        865 => DutyInfo {
            name: LocalisedText {
                en: "Cape Westwind",
                ja: "リットアティン強襲戦",
                de: "Kap Westwind",
                fr: "Le Cap Vendouest",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        866 => DutyInfo {
            name: LocalisedText {
                en: "Aglaia",
                ja: "輝ける神域 アグライア",
                de: "Aglaia",
                fr: "Domaine divin - Aglaé",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        867 => DutyInfo {
            name: LocalisedText {
                en: "Devastation",
                ja: "アシエン・ラハブレア討伐戦",
                de: "Das Antlitz der Kaltblütigkeit",
                fr: "Le Praetorium en flammes",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        868 => DutyInfo {
            name: LocalisedText {
                en: "The Sil'dihn Subterrane",
                ja: "ヴァリアントダンジョン シラディハ水道",
                de: "Die Unterstadt von Sil'dih",
                fr: "Les canalisations sildiennes - Donjon à embranchements",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        869 => DutyInfo {
            name: LocalisedText {
                en: "The Fell Court of Troia",
                ja: "異界孤城 トロイアコート",
                de: "Der Schwarze Hof von Troia",
                fr: "Le Château de Troïa",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        870 => DutyInfo {
            name: LocalisedText {
                en: "Storm's Crown",
                ja: "バルバリシア討滅戦",
                de: "Prophetie - Barbarizia",
                fr: "La Toison des tempêtes",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        871 => DutyInfo {
            name: LocalisedText {
                en: "Storm's Crown (Extreme)",
                ja: "極バルバリシア討滅戦",
                de: "Eschatos - Barbarizia",
                fr: "La Toison des tempêtes (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        872 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Fifth Circle",
                ja: "万魔殿パンデモニウム：煉獄編1",
                de: "Abyssos - Fünfter Kreis",
                fr: "Le Purgatoire du Pandæmonium - Cages",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        873 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Fifth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：煉獄編1",
                de: "Abyssos - Fünfter Kreis (episch)",
                fr: "Le Purgatoire du Pandæmonium - Cages (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        874 => DutyInfo {
            name: LocalisedText {
                en: "Where Everything Begins",
                ja: "失われた力",
                de: "Wo alles seinen Anfang nimmt",
                fr: "Au commencement était Zero",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        876 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Seventh Circle",
                ja: "万魔殿パンデモニウム：煉獄編3",
                de: "Abyssos - Siebter Kreis",
                fr: "Le Purgatoire du Pandæmonium - Racines",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        877 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Seventh Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：煉獄編3",
                de: "Abyssos - Siebter Kreis (episch)",
                fr: "Le Purgatoire du Pandæmonium - Racines (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        878 => DutyInfo {
            name: LocalisedText {
                en: "Another Sil'dihn Subterrane",
                ja: "アナザーダンジョン 異聞シラディハ水道",
                de: "Die kuriose Unterstadt von Sil'dih",
                fr: "Les canalisations sildiennes annexes - Donjon alternatif",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        879 => DutyInfo {
            name: LocalisedText {
                en: "Another Sil'dihn Subterrane (Savage)",
                ja: "アナザーダンジョン 異聞シラディハ水道 零式",
                de: "Die kuriose Unterstadt von Sil'dih (episch)",
                fr: "Les canalisations sildiennes annexes - Donjon alternatif (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        880 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Sixth Circle",
                ja: "万魔殿パンデモニウム：煉獄編2",
                de: "Abyssos - Sechster Kreis",
                fr: "Le Purgatoire du Pandæmonium - Croisements",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        881 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Sixth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：煉獄編2",
                de: "Abyssos - Sechster Kreis (episch)",
                fr: "Le Purgatoire du Pandæmonium - Croisements (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        883 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Eighth Circle",
                ja: "万魔殿パンデモニウム：煉獄編4",
                de: "Abyssos - Achter Kreis",
                fr: "Le Purgatoire du Pandæmonium - Hérédité",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        884 => DutyInfo {
            name: LocalisedText {
                en: "Abyssos: The Eighth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：煉獄編4",
                de: "Abyssos - Achter Kreis (episch)",
                fr: "Le Purgatoire du Pandæmonium - Hérédité (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        885 => DutyInfo {
            name: LocalisedText {
                en: "The Steps of Faith",
                ja: "皇都イシュガルド防衛戦",
                de: "Der Schicksalsweg",
                fr: "Le Siège de la sainte Cité d'Ishgard",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        886 => DutyInfo {
            name: LocalisedText {
                en: "Mount Ordeals",
                ja: "ルビカンテ討滅戦",
                de: "Prophetie - Rubicante",
                fr: "Le Mont du Supplice",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        896 => DutyInfo {
            name: LocalisedText {
                en: "Lapis Manalis",
                ja: "雪山冥洞 ラピス・マナリス",
                de: "Lapis Manalis",
                fr: "Lapis Manalis",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        897 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 1-10)",
                ja: "オルト・エウレカ B1～B10",
                de: "Eureka Orthos (Ebenen 1-10)",
                fr: "Eurêka Orthos (sous-sols 1-10)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        898 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 11-20)",
                ja: "オルト・エウレカ B11～B20",
                de: "Eureka Orthos (Ebenen 11-20)",
                fr: "Eurêka Orthos (sous-sols 11-20)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        899 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 21-30)",
                ja: "オルト・エウレカ B21～B30",
                de: "Eureka Orthos (Ebenen 21-30)",
                fr: "Eurêka Orthos (sous-sols 21-30)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        900 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 31-40)",
                ja: "オルト・エウレカ B31～B40",
                de: "Eureka Orthos (Ebenen 31-40)",
                fr: "Eurêka Orthos (sous-sols 31-40)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        901 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 41-50)",
                ja: "オルト・エウレカ B41～B50",
                de: "Eureka Orthos (Ebenen 41-50)",
                fr: "Eurêka Orthos (sous-sols 41-50)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        902 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 51-60)",
                ja: "オルト・エウレカ B51～B60",
                de: "Eureka Orthos (Ebenen 51-60)",
                fr: "Eurêka Orthos (sous-sols 51-60)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        903 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 61-70)",
                ja: "オルト・エウレカ B61～B70",
                de: "Eureka Orthos (Ebenen 61-70)",
                fr: "Eurêka Orthos (sous-sols 61-70)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        904 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 71-80)",
                ja: "オルト・エウレカ B71～B80",
                de: "Eureka Orthos (Ebenen 71-80)",
                fr: "Eurêka Orthos (sous-sols 71-80)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        905 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 81-90)",
                ja: "オルト・エウレカ B81～B90",
                de: "Eureka Orthos (Ebenen 81-90)",
                fr: "Eurêka Orthos (sous-sols 81-90)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        906 => DutyInfo {
            name: LocalisedText {
                en: "Eureka Orthos (Floors 91-100)",
                ja: "オルト・エウレカ B91～B100",
                de: "Eureka Orthos (Ebenen 91-100)",
                fr: "Eurêka Orthos (sous-sols 91-100)",
            },
            high_end: false,
            content_kind: ContentKind::DeepDungeons,
        },
        907 => DutyInfo {
            name: LocalisedText {
                en: "Generational Bonding",
                ja: "はつらつとした親子",
                de: "Väterliche Liebe",
                fr: "Tel gentilhomme, tel gentilhomme",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        908 => DutyInfo {
            name: LocalisedText {
                en: "The Omega Protocol (Ultimate)",
                ja: "絶オメガ検証戦",
                de: "Omega (fatal)",
                fr: "Le Protocole Oméga (fatal)",
            },
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        909 => DutyInfo {
            name: LocalisedText {
                en: "The Shifting Gymnasion Agonon",
                ja: "宝物庫 エルピス・ギュムナシオン祭殿",
                de: "Gymnasion Agonon",
                fr: "Elpis Gymnasion",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        910 => DutyInfo {
            name: LocalisedText {
                en: "An Unforeseen Bargain",
                ja: "パンひとつと引き換えに",
                de: "Der Wert einer Buuds",
                fr: "Pour une bouchée de pain",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        911 => DutyInfo {
            name: LocalisedText {
                en: "Euphrosyne",
                ja: "喜びの神域 エウプロシュネ",
                de: "Euphrosyne",
                fr: "Domaine divin - Euphrosyne",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        912 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        913 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        914 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        915 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        916 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        917 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        918 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        919 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        920 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        921 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        922 => DutyInfo {
            name: LocalisedText {
                en: "The Clockwork Castletown",
                ja: "東方絡繰御殿",
                de: "Die Mechanische Menagerie",
                fr: "Le Traquenard oriental",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        923 => DutyInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Custom Match - The Clockwork Castletown)",
                ja: "クリスタルコンフリクト(東方絡繰御殿：カスタムマッチ)",
                de: "Crystalline Conflict: Die Mechanische Menagerie (Schaukampf)",
                fr: "Crys. Conflict (partie perso. - Le Traquenard oriental)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        924 => DutyInfo {
            name: LocalisedText {
                en: "Mount Ordeals (Extreme)",
                ja: "極ルビカンテ討滅戦",
                de: "Eschatos - Rubicante",
                fr: "Le Mont du Supplice (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        925 => DutyInfo {
            name: LocalisedText {
                en: "Fangs of the Viper",
                ja: "ヴァイパーの使命",
                de: "Einführung in die Viperologie",
                fr: "Le devoir des rôdeurs vipère",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        927 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        928 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        929 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        930 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        931 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        932 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        933 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        934 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        935 => DutyInfo {
            name: LocalisedText {
                en: "Leap of Faith",
                ja: "挑戦！ ジャンピングアスレチック",
                de: "Kaktor-Kletterwand",
                fr: "Haute voltige",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        936 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Ninth Circle",
                ja: "万魔殿パンデモニウム：天獄編1",
                de: "Anabaseios - Neunter Kreis",
                fr: "Le Paradis du Pandæmonium - Métempsycose",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        937 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Ninth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：天獄編1",
                de: "Anabaseios - Neunter Kreis (episch)",
                fr: "Le Paradis du Pandæmonium - Métempsycose (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        938 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Tenth Circle",
                ja: "万魔殿パンデモニウム：天獄編2",
                de: "Anabaseios - Zehnter Kreis",
                fr: "Le Paradis du Pandæmonium - Monochrome",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        939 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Tenth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：天獄編2",
                de: "Anabaseios - Zehnter Kreis (episch)",
                fr: "Le Paradis du Pandæmonium - Monochrome (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        940 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Eleventh Circle",
                ja: "万魔殿パンデモニウム：天獄編3",
                de: "Anabaseios - Elfter Kreis",
                fr: "Le Paradis du Pandæmonium - Tribunal",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        941 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Eleventh Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：天獄編3",
                de: "Anabaseios - Elfter Kreis (episch)",
                fr: "Le Paradis du Pandæmonium - Tribunal (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        942 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Twelfth Circle",
                ja: "万魔殿パンデモニウム：天獄編4",
                de: "Anabaseios - Zwölfter Kreis",
                fr: "Le Paradis du Pandæmonium - Apothéose",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        943 => DutyInfo {
            name: LocalisedText {
                en: "Anabaseios: The Twelfth Circle (Savage)",
                ja: "万魔殿パンデモニウム零式：天獄編4",
                de: "Anabaseios - Zwölfter Kreis (episch)",
                fr: "Le Paradis du Pandæmonium - Apothéose (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        944 => DutyInfo {
            name: LocalisedText {
                en: "The Gilded Araya",
                ja: "アスラ討滅戦",
                de: "Prophetie - Asura",
                fr: "Le temple doré d'Araya",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        945 => DutyInfo {
            name: LocalisedText {
                en: "Mount Rokkon",
                ja: "ヴァリアントダンジョン 六根山",
                de: "Der Rokkon",
                fr: "Le mont Rokkon - Donjon à embranchements",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        946 => DutyInfo {
            name: LocalisedText {
                en: "Another Mount Rokkon",
                ja: "アナザーダンジョン 異聞六根山",
                de: "Der kuriose Rokkon",
                fr: "Le mont Rokkon annexe - Donjon alternatif",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        947 => DutyInfo {
            name: LocalisedText {
                en: "Another Mount Rokkon (Savage)",
                ja: "アナザーダンジョン 異聞六根山 零式",
                de: "Der kuriose Rokkon (episch)",
                fr: "Le mont Rokkon annexe - Donjon alternatif (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        948 => DutyInfo {
            name: LocalisedText {
                en: "A Golden Opportunity",
                ja: "黄金闘士「ゴールドル」",
                de: "Goldor der Güldene",
                fr: "Goldor, le mage doré",
            },
            high_end: false,
            content_kind: ContentKind::TheMaskedCarnivale,
        },
        949 => DutyInfo {
            name: LocalisedText {
                en: "The Voidcast Dais",
                ja: "ゴルベーザ討滅戦",
                de: "Prophetie - Golbez",
                fr: "La Chaire de l'Exilée",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        950 => DutyInfo {
            name: LocalisedText {
                en: "The Voidcast Dais (Extreme)",
                ja: "極ゴルベーザ討滅戦",
                de: "Eschatos - Golbez",
                fr: "La Chaire de l'Exilée (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        952 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        953 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        954 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        955 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        956 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        957 => DutyInfo {
            name: LocalisedText {
                en: "Ocean Fishing",
                ja: "オーシャンフィッシング",
                de: "Auf großer Fahrt",
                fr: "Pêche en mer",
            },
            high_end: false,
            content_kind: ContentKind::DisciplesoftheLand,
        },
        958 => DutyInfo {
            name: LocalisedText {
                en: "Blunderville",
                ja: "ブランダーヴィル",
                de: "Stolperville",
                fr: "Blunderville",
            },
            high_end: false,
            content_kind: ContentKind::GoldSaucer,
        },
        959 => DutyInfo {
            name: LocalisedText {
                en: "Memory of Embers",
                ja: "炎影の旅路",
                de: "Der Pfad der Flammen",
                fr: "Flammes primordiales",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        960 => DutyInfo {
            name: LocalisedText {
                en: "The Game Is Afoot",
                ja: "その瞳が見据えるもの",
                de: "Die Jagd ist eröffnet",
                fr: "Par-delà l'horizon",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        961 => DutyInfo {
            name: LocalisedText {
                en: "Aloalo Island",
                ja: "ヴァリアントダンジョン アロアロ島",
                de: "Aloalo",
                fr: "L'île d'Aloalo - Donjon à embranchements",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        962 => DutyInfo {
            name: LocalisedText {
                en: "Thaleia",
                ja: "華めく神域 タレイア",
                de: "Thaleia",
                fr: "Domaine divin - Thalie",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        964 => DutyInfo {
            name: LocalisedText {
                en: "The Abyssal Fracture",
                ja: "ゼロムス討滅戦",
                de: "Prophetie - Zeromus",
                fr: "La Fracture abyssale",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        965 => DutyInfo {
            name: LocalisedText {
                en: "The Abyssal Fracture (Extreme)",
                ja: "極ゼロムス討滅戦",
                de: "Eschatos - Zeromus",
                fr: "La Fracture abyssale (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        966 => DutyInfo {
            name: LocalisedText {
                en: "Somewhere Only She Knows",
                ja: "旅する画家は幻想を征く",
                de: "Begegnungen und Abschiede",
                fr: "L'œuvre de la maturité",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        967 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        968 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        969 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        970 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        971 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        972 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        973 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        974 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        975 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        976 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        977 => DutyInfo {
            name: LocalisedText {
                en: "The Red Sands",
                ja: "レッド・サンズ",
                de: "Die Roten Sande",
                fr: "Les Sables sanglants",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        978 => DutyInfo {
            name: LocalisedText {
                en: "Crystalline Conflict (Custom Match - The Red Sands)",
                ja: "クリスタルコンフリクト(レッド・サンズ：カスタムマッチ)",
                de: "Crystalline Conflict: Die Roten Sande (Schaukampf)",
                fr: "Crystalline Conflict (partie personnalisée - Les Sables sanglants)",
            },
            high_end: false,
            content_kind: ContentKind::PvP,
        },
        979 => DutyInfo {
            name: LocalisedText {
                en: "Another Aloalo Island",
                ja: "アナザーダンジョン 異聞アロアロ島",
                de: "Kurioses Aloalo",
                fr: "L'île d'Aloalo annexe - Donjon alternatif",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        980 => DutyInfo {
            name: LocalisedText {
                en: "Another Aloalo Island (Savage)",
                ja: "アナザーダンジョン 異聞アロアロ島 零式",
                de: "Kurioses Aloalo (episch)",
                fr: "L'île d'Aloalo annexe - Donjon alternatif (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::VCDungeonFinder,
        },
        981 => DutyInfo {
            name: LocalisedText {
                en: "The Strayborough Deadwalk",
                ja: "悪夢遊園 ストレイバロー",
                de: "Mahrlingen-Promenade",
                fr: "Le Parc-aux-Errants",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        982 => DutyInfo {
            name: LocalisedText {
                en: "The Feat of the Brotherhood",
                ja: "友の試練",
                de: "Von Schuld und Sühne",
                fr: "L'épreuve de l'amitié",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        983 => DutyInfo {
            name: LocalisedText {
                en: "Taking a Stand",
                ja: "笑顔を護るための戦い",
                de: "Mit scharfer Axt und leichtem Lächeln",
                fr: "Préserver la paix",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        984 => DutyInfo {
            name: LocalisedText {
                en: "The Interphos",
                ja: "エターナルクイーン討滅戦",
                de: "Zel Tajaal - Ewige Königin",
                fr: "Interphos",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        985 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M1",
                ja: "至天の座アルカディア：ライトヘビー級1",
                de: "Arkadion - Halbschwergewicht R1",
                fr: "Poids mi-lourds CCA - match 1",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        986 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M1 (Savage)",
                ja: "至天の座アルカディア零式：ライトヘビー級1",
                de: "Arkadion - Halbschwergewicht R1 (episch)",
                fr: "Poids mi-lourds CCA - match 1 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        987 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M2",
                ja: "至天の座アルカディア：ライトヘビー級2",
                de: "Arkadion - Halbschwergewicht R2",
                fr: "Poids mi-lourds CCA - match 2",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        988 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M2 (Savage)",
                ja: "至天の座アルカディア零式：ライトヘビー級2",
                de: "Arkadion - Halbschwergewicht R2 (episch)",
                fr: "Poids mi-lourds CCA - match 2 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        989 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M3",
                ja: "至天の座アルカディア：ライトヘビー級3",
                de: "Arkadion - Halbschwergewicht R3",
                fr: "Poids mi-lourds CCA - match 3",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        990 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M3 (Savage)",
                ja: "至天の座アルカディア零式：ライトヘビー級3",
                de: "Arkadion - Halbschwergewicht R3 (episch)",
                fr: "Poids mi-lourds CCA - match 3 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        991 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M4",
                ja: "至天の座アルカディア：ライトヘビー級4",
                de: "Arkadion - Halbschwergewicht R4",
                fr: "Poids mi-lourds CCA - match 4",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        992 => DutyInfo {
            name: LocalisedText {
                en: "AAC Light-heavyweight M4 (Savage)",
                ja: "至天の座アルカディア零式：ライトヘビー級4",
                de: "Arkadion - Halbschwergewicht R4 (episch)",
                fr: "Poids mi-lourds CCA - match 4 (sadique)",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        993 => DutyInfo {
            name: LocalisedText {
                en: "Cenote Ja Ja Gural",
                ja: "宝物庫 セノーテ・ジャジャグラル",
                de: "Cenote Ja Ja Gural",
                fr: "Cénote Ja Ja Gural",
            },
            high_end: false,
            content_kind: ContentKind::TreasureHunt,
        },
        994 => DutyInfo {
            name: LocalisedText {
                en: "An Antidote for Anarchy",
                ja: "毒と癒やしの頂上決戦",
                de: "Heilkunst gegen Hexerei",
                fr: "Les deux faces du scorpion",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        995 => DutyInfo {
            name: LocalisedText {
                en: "Everkeep",
                ja: "ゾラージャ討滅戦",
                de: "Zel Tajaal - Zoraal Ja",
                fr: "Le Pinacle de l'Éternité",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        996 => DutyInfo {
            name: LocalisedText {
                en: "Everkeep (Extreme)",
                ja: "極ゾラージャ討滅戦",
                de: "Gok Tajaal - Zoraal Ja",
                fr: "Le Pinacle de l'Éternité (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        997 => DutyInfo {
            name: LocalisedText {
                en: "A Hunter True",
                ja: "一端の狩人",
                de: "Krönender Abschuss",
                fr: "Chasseuse à lunettes... chasseuse honnête",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        998 => DutyInfo {
            name: LocalisedText {
                en: "The Protector and the Destroyer",
                ja: "護る者、壊す者",
                de: "Schützer des Volkes, Schlächter des Volkes",
                fr: "Protecteurs et destructeurs",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        999 => DutyInfo {
            name: LocalisedText {
                en: "The Mightiest Shield",
                ja: "打倒！ シャルトフィット盗賊団",
                de: "Der mächtigste Schild",
                fr: "Mauvais cru pour Chalteaufite",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1000 => DutyInfo {
            name: LocalisedText {
                en: "Mind over Manor",
                ja: "旅する画家は森都を征く",
                de: "Nächtliche Ruhestörung",
                fr: "Tapage nocturne",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1001 => DutyInfo {
            name: LocalisedText {
                en: "Heroes and Pretenders",
                ja: "祭器と作られた虚像",
                de: "Ein Mythos in Trümmern",
                fr: "Le château de cartes s'effondre",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1002 => DutyInfo {
            name: LocalisedText {
                en: "Vengeance of the Viper",
                ja: "トラルヴィドラールを狩る者",
                de: "Der Wille der Natur",
                fr: "Le fléau des Tural vidraal",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1003 => DutyInfo {
            name: LocalisedText {
                en: "Dreams of a New Day",
                ja: "寝ても覚めても",
                de: "Neue Bande der Freundschaft",
                fr: "Rêves éveillés",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1006 => DutyInfo {
            name: LocalisedText {
                en: "Futures Rewritten (Ultimate)",
                ja: "絶もうひとつの未来",
                de: "Eine zweite Zukunft (fatal)",
                fr: "Avenirs réécrits (fatal)",
            },
            high_end: true,
            content_kind: ContentKind::UltimateRaids,
        },
        1008 => DutyInfo {
            name: LocalisedText {
                en: "Yuweyawata Field Station",
                ja: "廃地討究 ユウェヤーワータ",
                de: "Forschungsstation Yuweyawata",
                fr: "Le centre de recherche de Yuweyawata",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        1009 => DutyInfo {
            name: LocalisedText {
                en: "The Warmth of Family",
                ja: "王の家族",
                de: "Ein Land, eine Familie",
                fr: "La famille de l'Aurarque",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1010 => DutyInfo {
            name: LocalisedText {
                en: "The Cloud of Darkness (Chaotic)",
                ja: "滅暗闇の雲激闘戦",
                de: "Die Wolke der Dunkelheit (chaotisch)",
                fr: "La Tour de Ténèbres (chaotique)",
            },
            high_end: true,
            content_kind: ContentKind::ChaoticAllianceRaid,
        },
        1012 => DutyInfo {
            name: LocalisedText {
                en: "React to Attack Markers",
                ja: "マーカーが出る攻撃に対処しよう！",
                de: "Achte auf Attackenmarkierungen!",
                fr: "Réagir à des marquages",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        1013 => DutyInfo {
            name: LocalisedText {
                en: "React to Floor Markers",
                ja: "地面に予兆が出る攻撃に対処しよう！",
                de: "Achte auf Geländemarkierungen!",
                fr: "Réagir à des indications au sol",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        1014 => DutyInfo {
            name: LocalisedText {
                en: "React to Advanced Visual Indicators",
                ja: "さまざまな攻撃に対処しよう！",
                de: "Stelle dich allerlei Angriffsmanövern!",
                fr: "Réagir à des mécaniques variées",
            },
            high_end: false,
            content_kind: ContentKind::Other(20),
        },
        1015 => DutyInfo {
            name: LocalisedText {
                en: "Jeuno: The First Walk",
                ja: "ジュノ：ザ・ファーストウォーク",
                de: "Jeuno: Die erste Etappe",
                fr: "Jeuno - La première perambulation",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        1016 => DutyInfo {
            name: LocalisedText {
                en: "Bar the Passage",
                ja: "決戦、ゾーゴー永結橋",
                de: "Kampf an Zorgor der Weiten",
                fr: "Pour s'affranchir de la tyrannie, tout s'appelle vertu",
            },
            high_end: false,
            content_kind: ContentKind::QuestBattles,
        },
        1017 => DutyInfo {
            name: LocalisedText {
                en: "The Minstrel's Ballad: Sphene's Burden",
                ja: "極エターナルクイーン討滅戦",
                de: "Gok Tajaal - Ewige Königin",
                fr: "Interphos (extrême)",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        1019 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M1",
                ja: "至天の座アルカディア：クルーザー級1",
                de: "Arkadion - Schwergewicht R1",
                fr: "Poids lourds-légers CCA - match 1",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        1020 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M1 (Savage)",
                ja: "至天の座アルカディア零式：クルーザー級1",
                de: "Arkadion - Schwergewicht R1 (episch)",
                fr: "Poids lourds-légers CCA - match 1 (sadique)",
            },
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        1021 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M2",
                ja: "至天の座アルカディア：クルーザー級2",
                de: "Arkadion - Schwergewicht R2",
                fr: "Poids lourds-légers CCA - match 2",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        1022 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M2 (Savage)",
                ja: "至天の座アルカディア零式：クルーザー級2",
                de: "Arkadion - Schwergewicht R2 (episch)",
                fr: "Poids lourds-légers CCA - match 2 (sadique)",
            },
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        1023 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M3",
                ja: "至天の座アルカディア：クルーザー級3",
                de: "Arkadion - Schwergewicht R3",
                fr: "Poids lourds-légers CCA - match 3",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        1024 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M3 (Savage)",
                ja: "至天の座アルカディア零式：クルーザー級3",
                de: "Arkadion - Schwergewicht R3 (episch)",
                fr: "Poids lourds-légers CCA - match 3 (sadique)",
            },
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        1025 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M4",
                ja: "至天の座アルカディア：クルーザー級4",
                de: "Arkadion - Schwergewicht R4",
                fr: "Poids lourds-légers CCA - match 4",
            },
            high_end: false,
            content_kind: ContentKind::Raids,
        },
        1026 => DutyInfo {
            name: LocalisedText {
                en: "AAC Cruiserweight M4 (Savage)",
                ja: "至天の座アルカディア零式：クルーザー級4",
                de: "Arkadion - Schwergewicht R4 (episch)",
                fr: "Poids lourds-légers CCA - match 4 (sadique)",
            },
            high_end: true,
            content_kind: ContentKind::Raids,
        },
        1027 => DutyInfo {
            name: LocalisedText {
                en: "The Underkeep",
                ja: "王城旧跡 アンダーキープ",
                de: "Unterfried-Ruinen",
                fr: "La Gardienne de l'Immémorial",
            },
            high_end: false,
            content_kind: ContentKind::Dungeons,
        },
        1029 => DutyInfo {
            name: LocalisedText {
                en: "Hells' Kier (Unreal)",
                ja: "幻朱雀征魂戦",
                de: "Traumprüfung - Suzaku",
                fr: "Le Nid des Lamentations (irréel)",
            },
            high_end: true,
            content_kind: ContentKind::Trials,
        },
        1030 => DutyInfo {
            name: LocalisedText {
                en: "Recollection",
                ja: "ゼレニア討滅戦",
                de: "Zel Tajaal - Zelenia",
                fr: "Le Sanctuaire du Serment",
            },
            high_end: false,
            content_kind: ContentKind::Trials,
        },
        1031 => DutyInfo {
            name: LocalisedText {
                en: "Recollection (Extreme)",
                ja: "極ゼレニア討滅戦",
                de: "Gok Tajaal - Zelenia",
                fr: "Le Sanctuaire du Serment (extrême)",
            },
            high_end: true,
            content_kind: ContentKind::Trials,
        },
    };
}
